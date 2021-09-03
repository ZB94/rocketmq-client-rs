use std::ffi::CStr;
use std::os::raw::c_char;

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
