use std::collections::HashMap;
use std::ffi::CString;

pub use error::MessageError;
pub use ext::MessageExt;
use rocketmq_client_sys::*;

pub mod error;
pub mod ext;

#[derive(Debug, Clone, Default)]
pub struct Message {
    pub topic: String,
    pub tags: String,
    pub keys: String,
    pub body: String,
    pub delay_time_level: i32,
    pub property: HashMap<String, String>,
}


impl Message {
    pub const MAX_TOPIC_LENGTH: usize = MAX_TOPIC_LENGTH as usize;
    pub const MAX_BODY_LENGTH: usize = i32::MAX as usize;

    pub fn new(topic: &str) -> Self {
        Self {
            topic: topic.to_string(),
            ..Default::default()
        }
    }

    /// 检查消息内容是否合法
    pub fn check(&self) -> Result<(), MessageError> {
        // 转为CString后徐添加`\0`
        if self.body.as_bytes().len() >= Self::MAX_BODY_LENGTH {
            return Err(MessageError::BodyLenOutOfSize);
        }

        if self.topic.as_bytes().len() >= Self::MAX_TOPIC_LENGTH {
            return Err(MessageError::TopicLenOutOfSize);
        }

        const NUL: char = '\0';
        if self.topic.contains(NUL) ||
            self.tags.contains(NUL) ||
            self.keys.contains(NUL) ||
            self.body.contains(NUL) ||
            self.property.iter().any(|(k, v)| k.contains(NUL) || v.contains(NUL))
        {
            return Err(MessageError::NulError);
        }

        Ok(())
    }
}


impl Message {
    /// # Panics
    /// 当结构中的任何字符串包含`\0`
    pub(crate) fn to_c(&self) -> Result<*mut CMessage, MessageError> {
        self.check()?;

        let topic = CString::new(self.topic.as_str())?;
        let tags = CString::new(self.tags.as_str())?;
        let keys = CString::new(self.keys.as_str())?;
        let body = CString::new(self.body.as_str())?;
        let property = {
            let mut p = Vec::with_capacity(self.property.len());
            for (k, v) in &self.property {
                p.push((CString::new(k.as_str())?, CString::new(v.as_str())?));
            }
            p
        };

        unsafe {
            let msg = CreateMessage(topic.as_ptr());
            SetMessageTags(msg, tags.as_ptr());
            SetMessageKeys(msg, keys.as_ptr());
            if self.body.is_empty() {
                SetByteMessageBody(msg, body.as_ptr(), body.as_bytes().len() as i32);
            }

            for (k, v) in &property {
                SetMessageProperty(msg, k.as_ptr(), v.as_ptr());
            }

            SetDelayTimeLevel(msg, self.delay_time_level);

            Ok(msg)
        }
    }

    pub(crate) fn drop_c(msg: *mut CMessage) {
        if !msg.is_null() {
            unsafe { DestroyMessage(msg); }
        }
    }
}
