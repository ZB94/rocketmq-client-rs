use std::ffi::NulError;

use rocketmq_client_sys::*;

use crate::utils::get_last_error;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PullConsumerErrorCode {
    StartFailed,
    FetchMessageFailed,
    FetchQueueFailed,

    NulError,
}

#[derive(Debug, Clone, Eq, PartialEq, Error)]
#[error("{code:?}: {message}")]
pub struct PullConsumerError {
    pub code: PullConsumerErrorCode,
    pub message: String,
}

impl PullConsumerError {
    pub(crate) fn check(code: i32) -> Result<(), Self> {
        #[allow(non_upper_case_globals)] let code = match code {
            _CStatus__OK => return Ok(()),
            _CStatus__PULLCONSUMER_FETCH_MESSAGE_FAILED => PullConsumerErrorCode::FetchMessageFailed,
            _CStatus__PULLCONSUMER_FETCH_MQ_FAILED => PullConsumerErrorCode::FetchQueueFailed,
            _CStatus__PULLCONSUMER_START_FAILED => PullConsumerErrorCode::StartFailed,
            _ => panic!("未知的拉取错误"),
        };
        let message = get_last_error();
        Err(Self { code, message })
    }
}


impl From<NulError> for PullConsumerError {
    fn from(_: NulError) -> Self {
        Self {
            code: PullConsumerErrorCode::NulError,
            message: "字符串中包含'\0'字符".to_string(),
        }
    }
}
