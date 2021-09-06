use rocketmq_client_sys::*;

use crate::message::ext::MessageExtPtr;
use crate::message::MessageExt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PullStatus {
    Found,
    NoNewMessage,
    NoMatchedMessage,
    OffsetIllegal,
    BrokerTimeout,
}

#[derive(Debug, Clone)]
pub struct PullResult {
    pub status: PullStatus,
    pub next_begin_offset: i64,
    pub min_offset: i64,
    pub max_offset: i64,
    pub message_found_list: Vec<MessageExt>,
}

impl PullResult {
    pub(crate) fn new(pr: &CPullResult, property_key: &[String]) -> Self {
        #[allow(non_upper_case_globals)] let status = match pr.pullStatus {
            E_CPullStatus_E_FOUND => PullStatus::Found,
            E_CPullStatus_E_NO_NEW_MSG => PullStatus::NoNewMessage,
            E_CPullStatus_E_NO_MATCHED_MSG => PullStatus::NoMatchedMessage,
            E_CPullStatus_E_OFFSET_ILLEGAL => PullStatus::OffsetIllegal,
            E_CPullStatus_E_BROKER_TIMEOUT => PullStatus::BrokerTimeout,
            other => panic!("未知的拉取状态: {}", other),
        };
        let next_begin_offset = pr.nextBeginOffset;
        let min_offset = pr.minOffset;
        let max_offset = pr.maxOffset;
        let message_found_list = unsafe {
            std::slice::from_raw_parts_mut(pr.msgFoundList, pr.size as usize)
                .into_iter()
                .map(|msg| MessageExtPtr::new(*msg).to_message_ext(property_key))
                .collect::<Vec<_>>()
        };

        Self {
            status,
            next_begin_offset,
            min_offset,
            max_offset,
            message_found_list,
        }
    }
}
