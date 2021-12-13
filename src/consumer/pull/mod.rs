use std::ffi::CString;
use std::ptr::null_mut;

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
    pull_consumer: *mut CPullConsumer,
}

impl PullConsumer {
    pub fn new(group: &str) -> PullConsumerBuilder {
        PullConsumerBuilder::new(group)
    }

    pub fn version(&self) -> String {
        from_c_str(unsafe { ShowPullConsumerVersion(self.pull_consumer) }).unwrap_or_default()
    }

    pub fn fetch_mq_list(&mut self, topic: &str) -> Result<MessageQueueList, PullConsumerError> {
        let mut ptr = null_mut();
        let mut size = 0;
        let topic = CString::new(topic)?;
        PullConsumerError::check(unsafe {
            FetchSubscriptionMessageQueues(
                self.pull_consumer,
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
        property_keys: &[String]
    ) -> Result<PullResult, PullConsumerError> {
        let expression = CString::new(expression)?;
        let cpr = unsafe { Pull(self.pull_consumer, mq.ptr, expression.as_ptr(), offset, max_nums) };
        let pr = PullResult::new(&cpr, property_keys);
        unsafe { ReleasePullResult(cpr) };
        Ok(pr)
    }
}


impl Drop for PullConsumer {
    fn drop(&mut self) {
        unsafe { ShutdownPullConsumer(self.pull_consumer) };
        unsafe { DestroyPullConsumer(self.pull_consumer) };
    }
}
