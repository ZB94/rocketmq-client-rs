use std::ffi::CString;

use rocketmq_client_sys::*;

use crate::utils::from_c_str;

use super::Message;

#[derive(Debug, Clone)]
pub struct MessageExt {
    pub id: String,
    pub queue_id: i32,
    pub reconsume_times: i32,
    pub store_size: i32,
    pub born_timestamp: i64,
    pub store_timestamp: i64,
    pub queue_offset: i64,
    pub commit_log_offset: i64,
    pub prepared_transaction_offset: i64,

    pub message: Message,
}

pub(crate) struct MessageExtPtr {
    ptr: *mut CMessageExt,
}


impl std::fmt::Debug for MessageExtPtr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MessageExt")
            .field("id", &self.id())
            .field("queue_id", &self.queue_id())
            .field("reconsume_times", &self.reconsume_times())
            .field("store_size", &self.store_size())
            .field("born_timestamp", &self.born_timestamp())
            .field("store_timestamp", &self.store_timestamp())
            .field("queue_offset", &self.queue_offset())
            .field("commit_log_offset", &self.commit_log_offset())
            .field("prepared_transaction_offset", &self.prepared_transaction_offset())

            .field("topic", &self.topic())
            .field("tags", &self.tags())
            .field("keys", &self.keys())
            .field("body", &self.body())
            .field("delay_time_level", &self.delay_time_level())
            .field("property", &format_args!("..."))
            .finish()
    }
}


impl MessageExtPtr {
    pub fn to_message_ext(&self, property_keys: &[String]) -> MessageExt {
        MessageExt {
            id: self.id(),
            queue_id: self.queue_id(),
            reconsume_times: self.reconsume_times(),
            store_size: self.store_size(),
            born_timestamp: self.born_timestamp(),
            store_timestamp: self.store_timestamp(),
            queue_offset: self.queue_offset(),
            commit_log_offset: self.commit_log_offset(),
            prepared_transaction_offset: self.prepared_transaction_offset(),
            message: Message {
                topic: self.topic(),
                tags: self.tags(),
                keys: self.keys(),
                body: self.body(),
                delay_time_level: self.delay_time_level(),
                property: property_keys.iter()
                    .map(|k| (k.clone(), self.property(k.as_str()).unwrap_or_default()))
                    .collect(),
            },
        }
    }
}

impl MessageExtPtr {
    pub(crate) fn new(ptr: *mut CMessageExt) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn topic(&self) -> String {
        from_c_str(unsafe { GetMessageTags(self.ptr) }).unwrap_or_default()
    }

    pub fn tags(&self) -> String {
        from_c_str(unsafe { GetMessageTags(self.ptr) }).unwrap_or_default()
    }

    pub fn keys(&self) -> String {
        from_c_str(unsafe { GetMessageKeys(self.ptr) }).unwrap_or_default()
    }

    pub fn body(&self) -> String {
        from_c_str(unsafe { GetMessageBody(self.ptr) }).unwrap_or_default()
    }

    pub fn property(&self, key: &str) -> Option<String> {
        if let Ok(key) = CString::new(key) {
            from_c_str(unsafe { GetMessageProperty(self.ptr, key.as_ptr()) })
        } else {
            None
        }
    }

    /// message_id
    pub fn id(&self) -> String {
        from_c_str(unsafe { GetMessageId(self.ptr) }).unwrap_or_default()
    }

    pub fn delay_time_level(&self) -> i32 {
        unsafe { GetMessageDelayTimeLevel(self.ptr) }
    }

    pub fn queue_id(&self) -> i32 {
        unsafe { GetMessageQueueId(self.ptr) }
    }

    pub fn reconsume_times(&self) -> i32 {
        unsafe { GetMessageReconsumeTimes(self.ptr) }
    }

    pub fn store_size(&self) -> i32 {
        unsafe { GetMessageStoreSize(self.ptr) }
    }

    pub fn born_timestamp(&self) -> i64 {
        unsafe { GetMessageBornTimestamp(self.ptr) }
    }

    pub fn store_timestamp(&self) -> i64 {
        unsafe { GetMessageStoreTimestamp(self.ptr) }
    }

    pub fn queue_offset(&self) -> i64 {
        unsafe { GetMessageQueueOffset(self.ptr) }
    }

    pub fn commit_log_offset(&self) -> i64 {
        unsafe { GetMessageCommitLogOffset(self.ptr) }
    }

    pub fn prepared_transaction_offset(&self) -> i64 {
        unsafe { GetMessagePreparedTransactionOffset(self.ptr) }
    }
}
