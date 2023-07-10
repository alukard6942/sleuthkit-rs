use crate::bindings::tsk_version_get_str;
use std::ffi::{CStr, CString};


fn version_str() -> String {

    let cst = unsafe {
        CStr::from_ptr( tsk_version_get_str())
    };

    cst.to_string_lossy().to_string()
}
