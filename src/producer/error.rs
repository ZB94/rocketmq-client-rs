use std::ffi::NulError;

use rocketmq_client_sys::*;

use crate::message::MessageError;
use crate::utils::{from_c_str, get_last_error};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ProducerErrorCode {
    StartFailed,
    SendSyncFailed,
    SendOnewayFailed,
    SendOrderlyFailed,
    SendAsyncFailed,
    SendOrderlyAsyncFailed,
    SendTransactionFailed,

    NulError,
    MessageError,

    MQException,
}

#[derive(Debug, Clone, Error)]
#[error("{code:?}: {message}")]
pub struct ProducerError {
    pub code: ProducerErrorCode,
    pub message: String,
}

impl ProducerError {
    pub(crate) fn check(code: i32) -> Result<(), Self> {
        #[allow(non_upper_case_globals)] let code = match code {
            _CStatus__OK => return Ok(()),
            _CStatus__PRODUCER_START_FAILED => ProducerErrorCode::StartFailed,
            _CStatus__PRODUCER_SEND_SYNC_FAILED => ProducerErrorCode::SendSyncFailed,
            _CStatus__PRODUCER_SEND_ONEWAY_FAILED => ProducerErrorCode::SendOnewayFailed,
            _CStatus__PRODUCER_SEND_ORDERLY_FAILED => ProducerErrorCode::SendOrderlyFailed,
            _CStatus__PRODUCER_SEND_ASYNC_FAILED => ProducerErrorCode::SendAsyncFailed,
            _CStatus__PRODUCER_SEND_ORDERLYASYNC_FAILED => ProducerErrorCode::SendOrderlyAsyncFailed,
            _CStatus__PRODUCER_SEND_TRANSACTION_FAILED => ProducerErrorCode::SendTransactionFailed,
            _ => panic!("未知的PProducer错误码: {}", code)
        };
        let message = get_last_error();
        Err(Self {
            code,
            message,
        })
    }
}

impl From<NulError> for ProducerError {
    fn from(_: NulError) -> Self {
        Self {
            code: ProducerErrorCode::NulError,
            message: "输入文本中包含`\0`字符".to_string(),
        }
    }
}

impl From<MessageError> for ProducerError {
    fn from(e: MessageError) -> Self {
        Self {
            code: ProducerErrorCode::MessageError,
            message: e.to_string(),
        }
    }
}

impl From<_CMQException_> for ProducerError {
    fn from(e: _CMQException_) -> Self {
        Self {
            code: ProducerErrorCode::MQException,
            message: format!(
                "[{file}:{line}][{ty}]{msg}({code})",
                file = from_c_str(e.file.as_ptr()).unwrap_or_default(),
                line = e.line,
                ty = from_c_str(e.type_.as_ptr()).unwrap_or_default(),
                msg = from_c_str(e.msg.as_ptr()).unwrap_or_default(),
                code = e.error,
            ),
        }
    }
}
