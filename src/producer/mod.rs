use std::ffi::{c_void, CString};
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicPtr, Ordering};

pub use builder::ProducerBuilder;
use error::ProducerError;
use rocketmq_client_sys::*;
pub use send_result::{SendResult, SendStatus};

use crate::message::Message;
use crate::utils::from_c_str;

mod send_result;
mod builder;
pub mod error;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProducerType {
    Default,
    Orderly,
    // Transaction,
}


impl Default for ProducerType {
    fn default() -> Self {
        Self::Default
    }
}

pub struct Producer {
    pub(crate) ptr: Arc<AtomicPtr<CProducer>>,
    counter: Arc<Mutex<usize>>,
}

impl Producer {
    pub fn new(group: &str) -> ProducerBuilder {
        ProducerBuilder::new(group)
    }

    fn from_ptr(ptr: *mut CProducer) -> Self {
        Self {
            ptr: Arc::new(AtomicPtr::new(ptr)),
            counter: Arc::new(Mutex::new(1)),
        }
    }

    pub fn version(&self) -> String {
        from_c_str(unsafe {
            ShowProducerVersion(self.ptr.load(Ordering::Relaxed))
        }).unwrap_or_default()
    }
}

impl Producer {
    pub fn send(&self, msg: Message) -> Result<SendResult, ProducerError> {
        let msg = msg.to_c()?;
        let mut sr = unsafe { std::mem::zeroed() };
        let r = ProducerError::check(unsafe {
            SendMessageSync(self.ptr.load(Ordering::Relaxed), msg, &mut sr)
        });

        Message::drop_c(msg);

        r.map(|_| sr.into())
    }

    pub fn send_oneway(&self, msg: Message) -> Result<(), ProducerError> {
        let msg = msg.to_c()?;
        let r = ProducerError::check(unsafe {
            SendMessageOneway(self.ptr.load(Ordering::Relaxed), msg)
        });
        Message::drop_c(msg);
        r
    }

    pub fn send_orderly(&self, msg: Message, sharing_key: &str) -> Result<SendResult, ProducerError> {
        let sharing_key = CString::new(sharing_key)?;
        let msg = msg.to_c()?;
        let mut sr = unsafe { std::mem::zeroed() };

        let r = ProducerError::check(unsafe {
            SendMessageOrderlyByShardingKey(
                self.ptr.load(Ordering::Relaxed), msg,
                sharing_key.as_ptr(),
                &mut sr,
            )
        });

        Message::drop_c(msg);
        r.map(|_| sr.into())
    }
}

impl Producer {
    /// **警告：** 该方法有可能发生内存泄露
    pub unsafe fn send_async<F: FnOnce(Result<SendResult, ProducerError>)>(&self, msg: Message, callback: F) -> Result<(), ProducerError> {
        let msg = msg.to_c()?;
        let cb = Box::into_raw(Box::new(Box::new(callback)));
        let r = ProducerError::check(
            SendAsync(
                self.ptr.load(Ordering::Relaxed),
                msg,
                Some(on_send_success::<F>),
                Some(on_send_exception::<F>),
                cb as *mut c_void,
            )
        );

        if r.is_err() {
            Message::drop_c(msg);
            Box::from_raw(cb);
        }

        r
    }
}

impl Producer {
    /// 调用之后如果再调用其他方法将会返回错误代码为[`error::ProducerErrorCode::NulError`]的错误
    pub fn shutdown(&mut self) -> Result<(), ProducerError> {
        let ptr = self.ptr.load(Ordering::Acquire);
        if ptr.is_null() {
            Ok(())
        } else {
            self.ptr.store(null_mut(), Ordering::Release);
            ProducerError::check(unsafe { ShutdownProducer(ptr) })
        }
    }
}

impl Clone for Producer {
    fn clone(&self) -> Self {
        *self.counter.lock().unwrap() += 1;
        Self {
            ptr: self.ptr.clone(),
            counter: self.counter.clone(),
        }
    }
}

impl Drop for Producer {
    fn drop(&mut self) {
        let mut counter = self.counter.lock().unwrap();
        if *counter == 1 {
            drop(counter);
            let _ = self.shutdown();
        } else {
            *counter -= 1;
        }
    }
}

unsafe extern "C" fn on_send_success<F: FnOnce(Result<SendResult, ProducerError>)>(result: CSendResult, msg: *mut CMessage, user_data: *mut c_void) {
    Message::drop_c(msg);
    let cb = Box::from_raw(user_data as *mut Box<F>);
    cb(Ok(SendResult::from(result)));
}

unsafe extern "C" fn on_send_exception<F: FnOnce(Result<SendResult, ProducerError>)>(err: CMQException, msg: *mut CMessage, user_data: *mut c_void) {
    Message::drop_c(msg);
    let cb = Box::from_raw(user_data as *mut Box<F>);
    cb(Err(ProducerError::from(err)));
}
