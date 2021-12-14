use std::ffi::CString;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicPtr, Ordering};

pub use builder::PullConsumerBuilder;
pub use message_queue::{MessageQueue, MessageQueueList};
pub use pull_result::{PullResult, PullStatus};
use rocketmq_client_sys::*;

use crate::consumer::pull::error::PullConsumerError;
use crate::utils::from_c_str;

mod builder;
mod pull_result;
mod message_queue;
pub mod error;

pub struct PullConsumer {
    ptr: Arc<AtomicPtr<CPullConsumer>>,
    counter: Arc<Mutex<usize>>,
}

impl PullConsumer {
    pub fn new(group: &str) -> PullConsumerBuilder {
        PullConsumerBuilder::new(group)
    }

    fn from_ptr(ptr: *mut CPullConsumer) -> Self {
        Self {
            ptr: Arc::new(AtomicPtr::new(ptr)),
            counter: Arc::new(Mutex::new(1)),
        }
    }

    pub fn version(&self) -> String {
        from_c_str(unsafe { ShowPullConsumerVersion(self.ptr.load(Ordering::Relaxed)) })
            .unwrap_or_default()
    }

    pub fn fetch_mq_list(&mut self, topic: &str) -> Result<MessageQueueList, PullConsumerError> {
        let mut ptr = null_mut();
        let mut size = 0;
        let topic = CString::new(topic)?;

        PullConsumerError::check(unsafe {
            FetchSubscriptionMessageQueues(
                self.ptr.load(Ordering::Relaxed),
                topic.as_ptr(),
                &mut ptr,
                &mut size,
            )
        })?;

        Ok(MessageQueueList::new(ptr, size as usize))
    }

    pub fn pull(
        &self,
        mq: &MessageQueue,
        expression: &str,
        offset: i64,
        max_nums: i32,
        property_keys: &[String],
    ) -> Result<PullResult, PullConsumerError> {
        let expression = CString::new(expression)?;

        let cpr = unsafe {
            Pull(
                self.ptr.load(Ordering::Relaxed),
                mq.ptr, expression.as_ptr(),
                offset,
                max_nums,
            )
        };
        let pr = PullResult::new(&cpr, property_keys);

        unsafe { ReleasePullResult(cpr) };

        Ok(pr)
    }
}

impl Clone for PullConsumer {
    fn clone(&self) -> Self {
        *self.counter.lock().unwrap() += 1;
        Self {
            ptr: self.ptr.clone(),
            counter: self.counter.clone(),
        }
    }
}

impl Drop for PullConsumer {
    fn drop(&mut self) {
        let mut counter = self.counter.lock().unwrap();
        if *counter == 1 {
            let ptr = self.ptr.swap(null_mut(), Ordering::Relaxed);
            if !ptr.is_null() {
                unsafe { ShutdownPullConsumer(ptr) };
                unsafe { DestroyPullConsumer(ptr) };
            }
        } else {
            *counter -= 1;
        }
    }
}
