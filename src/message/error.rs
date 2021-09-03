use std::ffi::NulError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Error)]
pub enum MessageError {
    /// 字符串中包含`\0`字符
    #[error("字符串中包含`\0`字符")]
    NulError,
    /// 消息体大小超过最大大小(i32::MAX)
    #[error("消息体大小超过{}", i32::MAX)]
    BodyLenOutOfSize,
    /// 主题大超过最大大小(512)
    #[error("主题长度超过{}", rocketmq_client_sys::MAX_TOPIC_LENGTH)]
    TopicLenOutOfSize,
}

impl From<NulError> for MessageError {
    fn from(_: NulError) -> Self {
        Self::NulError
    }
}
