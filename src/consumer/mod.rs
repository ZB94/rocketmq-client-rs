pub use pull::{
    error::{
        PullConsumerError,
        PullConsumerErrorCode,
    },
    MessageQueue,
    PullConsumer,
    PullConsumerBuilder,
    PullResult,
    PullStatus,
};
pub use push::{
    error::{
        PushConsumerError,
        PushConsumerReceiveError,
    },
    PushConsumer,
    PushConsumerBuilder,
    PushConsumerType,
};
use rocketmq_client_sys::*;

pub mod pull;
pub mod push;

pub enum MessageModel {
    Broadcasting = _CMessageModel__BROADCASTING as isize,
    Clustering = _CMessageModel__CLUSTERING as isize,
}

