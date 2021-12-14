use std::ffi::CString;
use std::sync::atomic::Ordering;

use rocketmq_client_sys::*;

use crate::consumer::pull::error::PullConsumerError;
use crate::consumer::pull::PullConsumer;

pub struct PullConsumerBuilder {
    group: String,
    address: Vec<String>,
    domain: Option<String>,
    session_credentials: Option<(String, String, String)>,
}


impl PullConsumerBuilder {
    pub fn new(group: &str) -> Self {
        Self {
            group: group.to_string(),
            address: vec![],
            domain: None,
            session_credentials: None,
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

    pub fn session_credentials(mut self, access_key: &str, secret_key: &str, channel: &str) -> Self {
        self.session_credentials = Some((access_key.to_string(), secret_key.to_string(), channel.to_string()));
        self
    }

    pub fn start(self) -> Result<PullConsumer, PullConsumerError> {
        let group = CString::new(self.group.as_str())?;
        let c = PullConsumer::from_ptr(unsafe {
            CreatePullConsumer(group.as_ptr())
        });
        let ptr = c.ptr.load(Ordering::Relaxed);

        let address = CString::new(self.address.join(";").as_str())?;
        unsafe { SetPullConsumerNameServerAddress(ptr, address.as_ptr()) };

        if let Some(domain) = self.domain {
            let domain = CString::new(domain.as_str())?;
            unsafe { SetPullConsumerNameServerDomain(ptr, domain.as_ptr()) };
        }

        if let Some((access_key, secret_key, channel)) = self.session_credentials {
            let access_key = CString::new(access_key.as_str())?;
            let secret_key = CString::new(secret_key.as_str())?;
            let channel = CString::new(channel.as_str())?;
            unsafe {
                SetPullConsumerSessionCredentials(
                    ptr,
                    access_key.as_ptr(),
                    secret_key.as_ptr(),
                    channel.as_ptr(),
                )
            };
        }

        PullConsumerError::check(unsafe { StartPullConsumer(ptr) })?;
        Ok(c)
    }
}
