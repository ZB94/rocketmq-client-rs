use std::ffi::CString;

use crossbeam_channel::unbounded;

use rocketmq_client_sys::*;

use crate::consumer::MessageModel;
use crate::consumer::push::error::PushConsumerError;
use crate::consumer::push::PushConsumer;
use crate::LogLevel;
use crate::message::MessageExt;

use super::CB_INFO;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PushConsumerType {
    Default,
    Orderly,
}

pub struct PushConsumerBuilder {
    ty: PushConsumerType,
    group: String,
    address: Vec<String>,
    domain: Option<String>,
    thread_count: Option<i32>,
    message_batch_max_size: Option<i32>,
    instant_name: Option<String>,
    session_credentials: Option<(String, String, String)>,
    log: Option<(LogLevel, i64, i32, String)>,
    message_model: Option<MessageModel>,
    max_message_cache_size: Option<(bool, i32)>,
    trace: bool,
}


impl PushConsumerBuilder {
    pub fn new(ty: PushConsumerType, group: &str) -> Self {
        Self {
            ty,
            group: group.to_string(),
            address: vec![],
            domain: None,
            thread_count: None,
            message_batch_max_size: None,
            instant_name: None,
            session_credentials: None,
            log: None,
            message_model: None,
            max_message_cache_size: None,
            trace: false,
        }
    }


    pub fn name_server_address(mut self, address: Vec<String>) -> Self {
        self.address = address;
        self
    }

    pub fn name_server_domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_string());
        self
    }

    pub fn thread_count(mut self, count: i32) -> Self {
        self.thread_count = Some(count);
        self
    }

    pub fn message_batch_max_size(mut self, size: i32) -> Self {
        self.message_batch_max_size = Some(size);
        self
    }

    pub fn instant_name(mut self, name: &str) -> Self {
        self.instant_name = Some(name.to_string());
        self
    }

    pub fn session_credentials(mut self, access_key: &str, secret_key: &str, channel: &str) -> Self {
        self.session_credentials = Some((access_key.to_string(), secret_key.to_string(), channel.to_string()));
        self
    }

    pub fn log_config(mut self, level: LogLevel, file_size: i64, file_num: i32, file_path: &str) -> Self {
        self.log = Some((level, file_size, file_num, file_path.to_string()));
        self
    }

    pub fn message_model(mut self, model: MessageModel) -> Self {
        self.message_model = Some(model);
        self
    }

    /// `mb`表示设置的单位是否为`Mb`
    pub fn max_message_cache_size(mut self, mb: bool, size: i32) -> Self {
        self.max_message_cache_size = Some((mb, size));
        self
    }

    pub fn trace(mut self, open: bool) -> Self {
        self.trace = open;
        self
    }

    pub fn start(self, topic: &str, expression: &str) -> Result<PushConsumer, PushConsumerError> {
        let group = CString::new(self.group.as_str())?;
        let (sender, receiver) = unbounded();

        let c = PushConsumer {
            ptr: unsafe { CreatePushConsumer(group.as_ptr()) },
            ty: self.ty,
            receiver,
        };

        CB_INFO.write().unwrap()
            .insert(c.ptr as usize, sender);

        let address = CString::new(self.address.join(";"))?;
        unsafe { SetPushConsumerNameServerAddress(c.ptr, address.as_ptr()) };

        if let Some(domain) = self.domain {
            let domain = CString::new(domain.as_str())?;
            unsafe { SetPushConsumerNameServerDomain(c.ptr, domain.as_ptr()) };
        }

        if let Some(count) = self.thread_count {
            unsafe { SetPushConsumerThreadCount(c.ptr, count) };
        }

        if let Some(size) = self.message_batch_max_size {
            unsafe { SetPushConsumerMessageBatchMaxSize(c.ptr, size) };
        }

        if let Some(name) = self.instant_name {
            let name = CString::new(name)?;
            unsafe { SetPushConsumerInstanceName(c.ptr, name.as_ptr()) };
        }

        if let Some((mb, size)) = self.max_message_cache_size {
            if mb {
                unsafe { SetPushConsumerMaxCacheMessageSizeInMb(c.ptr, size) };
            } else {
                unsafe { SetPushConsumerMaxCacheMessageSize(c.ptr, size) };
            }
        }

        if let Some((access_key, secret_key, channel)) = self.session_credentials {
            let access_key = CString::new(access_key.as_str())?;
            let secret_key = CString::new(secret_key.as_str())?;
            let channel = CString::new(channel.as_str())?;
            unsafe {
                SetPushConsumerSessionCredentials(
                    c.ptr,
                    access_key.as_ptr(),
                    secret_key.as_ptr(),
                    channel.as_ptr(),
                )
            };
        }

        if let Some((level, size, num, path)) = self.log {
            let path = CString::new(path.as_str())?;
            unsafe { SetPushConsumerLogPath(c.ptr, path.as_ptr()) };
            unsafe { SetPushConsumerLogFileNumAndSize(c.ptr, num, size) };
            unsafe { SetPushConsumerLogLevel(c.ptr, level as u32) };
        }

        if let Some(model) = self.message_model {
            unsafe { SetPushConsumerMessageModel(c.ptr, model as u32) };
        }

        #[allow(non_upper_case_globals)]
            let t = if self.trace {
            _CTraceModel__OPEN
        } else {
            _CTraceModel__CLOSE
        };
        unsafe { SetPushConsumerMessageTrace(c.ptr, t) };

        let topic = CString::new(topic)?;
        let expression = CString::new(expression)?;
        unsafe { Subscribe(c.ptr, topic.as_ptr(), expression.as_ptr()) };

        match self.ty {
            PushConsumerType::Default => unsafe { RegisterMessageCallback(c.ptr, Some(message_callback)) },
            PushConsumerType::Orderly => unsafe { RegisterMessageCallbackOrderly(c.ptr, Some(message_callback)) },
        };

        PushConsumerError::check(unsafe { StartPushConsumer(c.ptr) })?;

        Ok(c)
    }
}


unsafe extern "C" fn message_callback(c: *mut CPushConsumer, msg: *mut CMessageExt) -> i32 {
    if let Ok(Some(sender)) = CB_INFO.read().map(|m| m.get(&(c as usize)).cloned()) {
        let msg = MessageExt::new(msg);
        let _ = sender.send(msg);
        E_CConsumeStatus_E_CONSUME_SUCCESS as i32
    } else {
        E_CConsumeStatus_E_RECONSUME_LATER as i32
    }
}
