use std::collections::HashMap;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicPtr, Ordering};

use crossbeam_channel::{Receiver, Sender};
use once_cell::sync::Lazy;

pub use builder::{PushConsumerBuilder, PushConsumerType};
use rocketmq_client_sys::*;

use crate::message::MessageExt;
use crate::utils::from_c_str;

mod builder;
pub mod error;

static CB_INFO: Lazy<RwLock<HashMap<usize, (Vec<String>, Sender<MessageExt>)>>> = Lazy::new(|| Default::default());

pub struct PushConsumer {
    pub(crate) ptr: Arc<AtomicPtr<CPushConsumer>>,
    ty: PushConsumerType,
    receiver: Receiver<MessageExt>,
    counter: Arc<Mutex<usize>>,
}

impl PushConsumer {
    pub fn builder(ty: PushConsumerType, group: &str, property_keys: Vec<String>) -> PushConsumerBuilder {
        PushConsumerBuilder::new(ty, group, property_keys)
    }

    fn from_ptr(ptr: *mut CPushConsumer, ty: PushConsumerType, receiver: Receiver<MessageExt>) -> Self {
        Self {
            ptr: Arc::new(AtomicPtr::new(ptr)),
            ty,
            receiver,
            counter: Arc::new(Mutex::new(1)),
        }
    }

    pub fn version(&self) -> String {
        from_c_str(unsafe { ShowPushConsumerVersion(self.ptr.load(Ordering::Relaxed)) })
            .unwrap_or_default()
    }

    pub fn receiver(&self) -> &Receiver<MessageExt> {
        &self.receiver
    }
}

impl Clone for PushConsumer {
    fn clone(&self) -> Self {
        *self.counter.lock().unwrap() += 1;
        Self {
            ptr: self.ptr.clone(),
            ty: self.ty,
            receiver: self.receiver.clone(),
            counter: self.counter.clone(),
        }
    }
}

impl Drop for PushConsumer {
    fn drop(&mut self) {
        let mut counter = self.counter.lock().unwrap();
        if *counter == 1 {
            let ptr = self.ptr.swap(null_mut(), Ordering::Relaxed);

            if ptr.is_null() {
                return;
            }

            let _ = CB_INFO.write()
                .unwrap()
                .remove(&(ptr as usize));

            unsafe {
                match self.ty {
                    PushConsumerType::Default => UnregisterMessageCallback(ptr),
                    PushConsumerType::Orderly => UnregisterMessageCallbackOrderly(ptr),
                };
                ShutdownPushConsumer(ptr);
                DestroyPushConsumer(ptr);
            }
        } else {
            *counter -= 1;
        }
    }
}
