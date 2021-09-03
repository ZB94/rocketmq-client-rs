use rocketmq_client_sys::*;

use crate::utils::from_c_str;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SendStatus {
    OK,
    FlushDiskTimeout,
    FlushSlaveTimeout,
    SlaveNotAvailable,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SendResult {
    pub status: SendStatus,
    pub message_id: String,
    pub offset: i64,
}


impl From<CSendResult> for SendResult {
    fn from(r: CSendResult) -> Self {
        #[allow(non_upper_case_globals)] let status = match r.sendStatus {
            E_CSendStatus__E_SEND_OK => SendStatus::OK,
            E_CSendStatus__E_SEND_FLUSH_DISK_TIMEOUT => SendStatus::FlushDiskTimeout,
            E_CSendStatus__E_SEND_FLUSH_SLAVE_TIMEOUT => SendStatus::FlushSlaveTimeout,
            E_CSendStatus__E_SEND_SLAVE_NOT_AVAILABLE => SendStatus::SlaveNotAvailable,
            _ => panic!("未知的消息发送状态"),
        };
        let message_id = from_c_str(r.msgId.as_ptr()).unwrap_or_default();
        let offset = r.offset;

        Self {
            status,
            message_id,
            offset,
        }
    }
}
