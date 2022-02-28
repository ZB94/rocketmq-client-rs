use std::ffi::{c_void, CString};
use std::ptr::null_mut;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

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
    counter: Arc<AtomicUsize>,
}

impl Producer {
    pub fn builder(group: &str) -> ProducerBuilder {
        ProducerBuilder::new(group)
    }

    fn from_ptr(ptr: *mut CProducer) -> Self {
        Self {
            ptr: Arc::new(AtomicPtr::new(ptr)),
            counter: Arc::new(AtomicUsize::new(1)),
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
    /// # Safety
    /// 如果回调没有被触发将会发生内存泄漏
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
    pub fn shutdown(&self) -> Result<(), ProducerError> {
        let ptr = self.ptr.swap(null_mut(), Ordering::Relaxed);
        if ptr.is_null() {
            Ok(())
        } else {
            ProducerError::check(unsafe { ShutdownProducer(ptr) })
        }
    }
}

impl Clone for Producer {
    fn clone(&self) -> Self {
        self.counter.fetch_add(1, Ordering::Release);
        Self {
            ptr: self.ptr.clone(),
            counter: self.counter.clone(),
        }
    }
}

impl Drop for Producer {
    fn drop(&mut self) {
        let counter = self.counter.fetch_sub(1, Ordering::Release);
        if counter == 1 {
            let _ = self.shutdown();
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
