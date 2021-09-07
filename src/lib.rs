#[cfg(feature = "generated")]
extern crate rocketmq_client_sys;
#[cfg(feature = "autogen")]
extern crate rocketmq_client_sys_autogen as rocketmq_client_sys;
#[macro_use]
extern crate thiserror;


pub mod message;
pub mod producer;
pub mod consumer;
mod utils;


#[allow(non_upper_case_globals)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LogLevel {
    Trace = rocketmq_client_sys::_CLogLevel__E_LOG_LEVEL_TRACE as isize,
    Debug = rocketmq_client_sys::_CLogLevel__E_LOG_LEVEL_DEBUG as isize,
    Info = rocketmq_client_sys::_CLogLevel__E_LOG_LEVEL_INFO as isize,
    Warning = rocketmq_client_sys::_CLogLevel__E_LOG_LEVEL_WARN as isize,
    Error = rocketmq_client_sys::_CLogLevel__E_LOG_LEVEL_ERROR as isize,
    Fatal = rocketmq_client_sys::_CLogLevel__E_LOG_LEVEL_FATAL as isize,
    LevelNum = rocketmq_client_sys::_CLogLevel__E_LOG_LEVEL_LEVEL_NUM as isize,
}
