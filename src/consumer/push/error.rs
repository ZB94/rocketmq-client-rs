use std::ffi::NulError;

use crossbeam_channel::{RecvError, RecvTimeoutError, TryRecvError};

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

#[derive(Debug, Error, Copy, Clone, Eq, PartialEq)]
pub enum PushConsumerReceiveError {
    #[error("未接收到数据")]
    Empty,
    #[error("数据接收超时")]
    Timeout,
    #[error("接收通道已被关闭")]
    Disconnect,
}

impl From<RecvError> for PushConsumerReceiveError {
    fn from(_: RecvError) -> Self {
        Self::Disconnect
    }
}

impl From<TryRecvError> for PushConsumerReceiveError {
    fn from(e: TryRecvError) -> Self {
        match e {
            TryRecvError::Empty => Self::Empty,
            TryRecvError::Disconnected => Self::Disconnect
        }
    }
}

impl From<RecvTimeoutError> for PushConsumerReceiveError {
    fn from(e: RecvTimeoutError) -> Self {
        match e {
            RecvTimeoutError::Timeout => Self::Timeout,
            RecvTimeoutError::Disconnected => Self::Disconnect
        }
    }
}
