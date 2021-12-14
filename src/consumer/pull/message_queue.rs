use std::ops::Deref;

use rocketmq_client_sys::*;

use crate::utils::from_c_str;

pub struct MessageQueueList {
    list: Vec<MessageQueue>,
    ptr: *mut CMessageQueue,
}

impl MessageQueueList {
    pub(crate) fn new(ptr: *mut CMessageQueue, size: usize) -> Self {
        let mut list = Vec::with_capacity(size);
        for i in 0..size as isize {
            list.push(MessageQueue { ptr: unsafe { ptr.offset(i) } });
        }
        Self {
            list,
            ptr,
        }
    }
}

impl Deref for MessageQueueList {
    type Target = Vec<MessageQueue>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl Drop for MessageQueueList {
    fn drop(&mut self) {
        self.list.clear();
        unsafe { ReleaseSubscriptionMessageQueue(self.ptr) };
    }
}

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
