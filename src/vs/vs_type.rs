
use std::{ffi::CStr, fmt::Display};
use crate::{bindings::*, tchar::Tchar};

impl Display for TSK_VS_TYPE_ENUM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_name())
    }
}

impl TSK_VS_TYPE_ENUM {
    pub fn supported() -> Self {
        unsafe { tsk_vs_type_supported() }
    }

    pub fn type_toid<T: Into<Tchar>>(str: T) -> Self {
        unsafe { tsk_vs_type_toid(*str.into()) }
    }

    pub fn to_name<'a>(self) -> &'a str {
        unsafe {
            let s = tsk_vs_type_toname(self);
            CStr::from_ptr(s).to_str().unwrap()
        }
    }

    pub fn to_desc<'a>(self) -> &'a str {
        unsafe {
            let s = tsk_vs_type_todesc(self);
            CStr::from_ptr(s).to_str().unwrap()
        }
    }
}
