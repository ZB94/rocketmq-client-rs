use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use once_cell::sync::Lazy;

pub use builder::{PushConsumerBuilder, PushConsumerType};
use rocketmq_client_sys::*;

use crate::consumer::push::error::PushConsumerReceiveError;
use crate::message::MessageExt;
use crate::utils::from_c_str;

mod builder;
pub mod error;

static CB_INFO: Lazy<RwLock<HashMap<usize, (Vec<String>, Sender<MessageExt>)>>> = Lazy::new(|| Default::default());

pub struct PushConsumer {
    ptr: *mut CPushConsumer,
    ty: PushConsumerType,
    receiver: Receiver<MessageExt>,
}

impl PushConsumer {
    pub fn new(ty: PushConsumerType, group: &str, property_keys: Vec<String>) -> PushConsumerBuilder {
        PushConsumerBuilder::new(ty, group, property_keys)
    }

    pub fn version(&self) -> String {
        from_c_str(unsafe { ShowPushConsumerVersion(self.ptr) }).unwrap_or_default()
    }

    pub fn recv(&self) -> Result<MessageExt, PushConsumerReceiveError> {
        Ok(self.receiver.recv()?)
    }

    pub fn try_recv(&self) -> Result<MessageExt, PushConsumerReceiveError> {
        Ok(self.receiver.try_recv()?)
    }

    pub fn recv_timeout(&self, timeout: Duration) -> Result<MessageExt, PushConsumerReceiveError> {
        Ok(self.receiver.recv_timeout(timeout)?)
    }
}

impl Drop for PushConsumer {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }

        let _ = CB_INFO.write()
            .unwrap()
            .remove(&(self.ptr as usize));

        unsafe {
            match self.ty {
                PushConsumerType::Default => UnregisterMessageCallback(self.ptr),
                PushConsumerType::Orderly => UnregisterMessageCallbackOrderly(self.ptr),
            };
            ShutdownPushConsumer(self.ptr);
            DestroyPushConsumer(self.ptr);
        }
    }
}
