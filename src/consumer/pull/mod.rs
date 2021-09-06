use std::ffi::CString;
use std::ptr::null_mut;

pub use builder::PullConsumerBuilder;
pub use message_queue::MessageQueue;
pub use pull_result::{PullResult, PullStatus};
use rocketmq_client_sys::*;

use crate::consumer::pull::error::PullConsumerError;
use crate::utils::from_c_str;

mod builder;
mod pull_result;
mod message_queue;
pub mod error;

pub struct PullConsumer {
    pull_consumer: *mut CPullConsumer,
    mq_list: *mut CMessageQueue,
    mq_size: i32,
    message_queue_list: Vec<MessageQueue>,
}

impl PullConsumer {
    pub fn new(group: &str) -> PullConsumerBuilder {
        PullConsumerBuilder::new(group)
    }

    pub fn version(&self) -> String {
        from_c_str(unsafe { ShowPullConsumerVersion(self.pull_consumer) }).unwrap_or_default()
    }

    pub fn fetch_mq_list(&mut self, topic: &str) -> Result<(), PullConsumerError> {
        self.release_mq_list();

        let topic = CString::new(topic)?;
        PullConsumerError::check(unsafe {
            FetchSubscriptionMessageQueues(
                self.pull_consumer,
                topic.as_ptr(),
                &mut self.mq_list,
                &mut self.mq_size,
            )
        })?;

        self.message_queue_list = unsafe {
            std::slice::from_raw_parts(&self.mq_list, self.mq_size as usize)
                .into_iter()
                .map(|ptr| MessageQueue { ptr: *ptr as *const _ })
                .collect()
        };

        Ok(())
    }

    pub fn mq_list(&self) -> &[MessageQueue] {
        &self.message_queue_list
    }

    pub fn pull(&self, mq: &MessageQueue, expression: &str, offset: i64, max_nums: i32, property_keys: &[String]) -> Result<PullResult, PullConsumerError> {
        let expression = CString::new(expression)?;
        let cpr = unsafe { Pull(self.pull_consumer, mq.ptr, expression.as_ptr(), offset, max_nums) };
        let pr = PullResult::new(&cpr, property_keys);
        unsafe { ReleasePullResult(cpr) };
        Ok(pr)
    }

    fn release_mq_list(&mut self) {
        self.message_queue_list.clear();
        if !self.pull_consumer.is_null() {
            unsafe { ReleaseSubscriptionMessageQueue(self.mq_list) };
            self.mq_list = null_mut();
            self.mq_size = 0;
        }
    }
}


impl Drop for PullConsumer {
    fn drop(&mut self) {
        self.release_mq_list();
        unsafe { ShutdownPullConsumer(self.pull_consumer) };
        unsafe { DestroyPullConsumer(self.pull_consumer) };
    }
}
