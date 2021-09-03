use std::ffi::CString;

use rocketmq_client_sys::*;

use crate::message::MessageError;
use crate::utils::from_c_str;

/// 不要手动初始化该结构体
pub struct MessageExt {
    id: String,
    topic: String,
    keys: String,
    body: String,
    ptr: *mut CMessageExt,
}

impl std::fmt::Debug for MessageExt {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MessageExt")
            .field("id", &self.id())
            .field("topic", &self.topic())
            .field("keys", &self.keys())
            .field("body", &self.body())
            .field("queue_id", &self.queue_id())
            .field("reconsume_times", &self.reconsume_times())
            .field("store_size", &self.store_size())
            .field("born_timestamp", &self.born_timestamp())
            .field("store_timestamp", &self.store_timestamp())
            .field("queue_offset", &self.queue_offset())
            .field("commit_log_offset", &self.commit_log_offset())
            .field("prepared_transaction_offset", &self.prepared_transaction_offset())
            .field("property", &format_args!("..."))
            .finish()
    }
}

impl MessageExt {
    pub fn topic(&self) -> &str {
        &self.topic
    }
    pub fn keys(&self) -> &str {
        &self.keys
    }
    pub fn body(&self) -> &str {
        &self.body
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}


impl MessageExt {
    pub(crate) fn new(ptr: *mut CMessageExt) -> Self {
        assert!(!ptr.is_null());
        let mut r = Self {
            topic: "".to_string(),
            keys: "".to_string(),
            body: "".to_string(),
            id: "".to_string(),
            ptr,
        };
        r.topic = r.c_topic();
        r.keys = r.c_keys();
        r.body = r.c_body();
        r.id = r.c_id();
        r
    }

    fn c_topic(&self) -> String {
        from_c_str(unsafe { GetMessageTags(self.ptr) }).unwrap_or_default()
    }

    fn c_keys(&self) -> String {
        from_c_str(unsafe { GetMessageKeys(self.ptr) }).unwrap_or_default()
    }

    fn c_body(&self) -> String {
        from_c_str(unsafe { GetMessageBody(self.ptr) }).unwrap_or_default()
    }

    pub fn property(&self, key: &str) -> Result<Option<String>, MessageError> {
        let key = CString::new(key)?;
        Ok(from_c_str(unsafe { GetMessageProperty(self.ptr, key.as_ptr()) }))
    }

    /// message_id
    fn c_id(&self) -> String {
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
