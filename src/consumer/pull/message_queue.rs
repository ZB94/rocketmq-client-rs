use rocketmq_client_sys::*;

use crate::utils::from_c_str;

pub struct MessageQueue {
    pub(crate) ptr: *const CMessageQueue,
}

impl MessageQueue {
    pub fn id(&self) -> i32 {
        unsafe { (*self.ptr).queueId }
    }

    pub fn topic(&self) -> String {
        from_c_str(unsafe { (*self.ptr).topic.as_ptr() }).unwrap_or_default()
    }

    pub fn broker_name(&self) -> String {
        from_c_str(unsafe { (*self.ptr).brokerName.as_ptr() }).unwrap_or_default()
    }
}

impl std::fmt::Debug for MessageQueue {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MessageQueue")
            .field("id", &self.id())
            .field("topic", &self.topic())
            .field("broker_name", &self.broker_name())
            .finish()
    }
}
