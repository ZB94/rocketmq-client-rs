use std::ffi::CString;
use std::sync::atomic::Ordering;

use rocketmq_client_sys::*;

use crate::LogLevel;
use crate::producer::{Producer, ProducerType};
use crate::producer::error::ProducerError;

pub struct ProducerBuilder {
    group: String,
    ty: Option<ProducerType>,
    name_server_address: Vec<String>,
    name_server_domain: Option<String>,
    instance_name: Option<String>,
    session_credentials: Option<(String, String, String)>,
    log_config: Option<(LogLevel, i32, i64, String)>,
    send_timeout: Option<i32>,
    compress_level: Option<i32>,
    max_message_size: Option<i32>,
    trace: bool,
}

impl ProducerBuilder {
    pub fn new(group: &str) -> Self {
        Self {
            group: group.to_string(),
            ty: None,
            name_server_address: vec![],
            name_server_domain: None,
            instance_name: None,
            session_credentials: None,
            log_config: None,
            send_timeout: None,
            compress_level: None,
            max_message_size: None,
            trace: false,
        }
    }

    pub fn start(self) -> Result<Producer, ProducerError> {
        let group = CString::new(self.group.as_str())?;
        let ty = self.ty.unwrap_or_default();
        let ptr = match ty {
            ProducerType::Default => unsafe { CreateProducer(group.as_ptr()) },
            ProducerType::Orderly => unsafe { CreateOrderlyProducer(group.as_ptr()) },
        };
        let p = Producer::from_ptr(ptr);
        let ptr = p.ptr.load(Ordering::Relaxed);

        if self.name_server_address.len() > 0 {
            let address = CString::new(self.name_server_address.join(";"))?;
            ProducerError::check(unsafe { SetProducerNameServerAddress(ptr, address.as_ptr()) })?;
        }

        if let Some(domain) = self.name_server_domain {
            let domain = CString::new(domain.as_str())?;
            ProducerError::check(unsafe { SetProducerNameServerDomain(ptr, domain.as_ptr()) })?;
        }

        if let Some(name) = self.instance_name {
            let name = CString::new(name.as_str())?;
            ProducerError::check(unsafe { SetProducerInstanceName(ptr, name.as_ptr()) })?;
        }

        if let Some((access_key, secret, ons_channel)) = self.session_credentials {
            let access_key = CString::new(access_key.as_str())?;
            let secret_key = CString::new(secret.as_str())?;
            let ons_channel = CString::new(ons_channel.as_str())?;

            ProducerError::check(unsafe {
                SetProducerSessionCredentials(
                    ptr,
                    access_key.as_ptr(), secret_key.as_ptr(), ons_channel.as_ptr())
            })?;
        }

        if let Some((level, num, size, path)) = self.log_config {
            let level = level as u32;
            let path = CString::new(path.as_str())?;

            ProducerError::check(unsafe { SetProducerLogPath(ptr, path.as_ptr()) })?;
            ProducerError::check(unsafe { SetProducerLogFileNumAndSize(ptr, num, size) })?;
            ProducerError::check(unsafe { SetProducerLogLevel(ptr, level) })?;
        }

        if let Some(timeout) = self.send_timeout {
            ProducerError::check(unsafe { SetProducerSendMsgTimeout(ptr, timeout) })?;
        }

        if let Some(level) = self.compress_level {
            ProducerError::check(unsafe { SetProducerCompressLevel(ptr, level) })?;
        }

        if let Some(size) = self.max_message_size {
            ProducerError::check(unsafe { SetProducerMaxMessageSize(ptr, size) })?;
        }

        let t = if self.trace {
            _CTraceModel__OPEN
        } else {
            _CTraceModel__CLOSE
        };
        ProducerError::check(unsafe { SetProducerMessageTrace(ptr, t) })?;

        ProducerError::check(unsafe { StartProducer(ptr) })?;

        Ok(p)
    }

    pub fn group_name(mut self, group: &str) -> Self {
        self.group = group.to_string();
        self
    }

    pub fn producer_type(mut self, ty: ProducerType) -> Self {
        self.ty = Some(ty);
        self
    }

    pub fn name_server_address(mut self, address: Vec<String>) -> Self {
        self.name_server_address = address;
        self
    }

    pub fn name_server_domain(mut self, domain: &str) -> Self {
        self.name_server_domain = Some(domain.to_string());
        self
    }

    pub fn instance_name(mut self, name: &str) -> Self {
        self.instance_name = Some(name.to_string());
        self
    }

    pub fn session_credentials(mut self, access_key: &str, secret_key: &str, ons_channel: &str) -> Self {
        self.session_credentials = Some((access_key.to_string(), secret_key.to_string(), ons_channel.to_string()));
        self
    }

    pub fn log(mut self, level: LogLevel, file_size: i64, file_num: i32, file_path: &str) -> Self {
        self.log_config = Some((level, file_num, file_size, file_path.to_string()));
        self
    }

    pub fn timeout(mut self, timeout: i32) -> Self {
        self.send_timeout = Some(timeout);
        self
    }

    pub fn compress_level(mut self, level: i32) -> Self {
        self.compress_level = Some(level);
        self
    }

    pub fn max_message_size(mut self, size: i32) -> Self {
        self.max_message_size = Some(size);
        self
    }

    pub fn trace(mut self, open: bool) -> Self {
        self.trace = open;
        self
    }
}
