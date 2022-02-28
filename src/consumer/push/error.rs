use std::ffi::NulError;

use rocketmq_client_sys::*;

use crate::utils::get_last_error;

#[derive(Debug, Error)]
pub enum PushConsumerError {
    /// 字符串中包含`\0`字符
    #[error("字符串中包含`\0`字符")]
    NulError,
    /// 启动失败
    #[error("启动失败: {0}")]
    StartFailedError(String),
}

impl PushConsumerError {
    pub fn check(code: i32) -> Result<(), Self> {
        if code == _CStatus__OK {
            Ok(())
        } else {
            Err(Self::StartFailedError(get_last_error()))
        }
    }
}

impl From<NulError> for PushConsumerError {
    fn from(_: NulError) -> Self {
        Self::NulError
    }
}
