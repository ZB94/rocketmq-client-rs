use std::ffi::CStr;
use std::os::raw::c_char;

use rocketmq_client_sys::GetLatestErrorMessage;

#[inline]
pub fn from_c_str(s: *const c_char) -> Option<String> {
    if s.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(s).to_string_lossy().trim_matches('\0').to_string() }
        )
    }
}


pub fn get_last_error() -> String {
    from_c_str(unsafe { GetLatestErrorMessage() }).unwrap_or_default()
}
