use crate::bindings::*;
use std::ffi::{CStr, CString};
use std::ops::Deref;
use std::string::String;

// since &str does not end witch 0, can be converted only from string
pub struct Tchar(*const TSK_TCHAR);

impl Tchar {
    pub fn is_empty(&self) -> bool {
        unsafe { *self.0.add(1) == 0 }
    }
}

impl Deref for Tchar {
    type Target = *const TSK_TCHAR;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait AsTchar{
    fn as_tchar(&self) -> Tchar;
}

impl AsTchar for CString {
    fn as_tchar(&self) -> Tchar {
        Tchar(self.as_ptr() as *const TSK_TCHAR)
    }
}

// impl AsTchar for &CStr {
//     fn as_tchar(&self) -> Tchar {
//         Tchar(self.as_ptr() as *const TSK_TCHAR)
//     }
// }


impl From<Tchar> for String {
    fn from(val: Tchar) -> Self {
        unsafe { CStr::from_ptr(val.0) }.to_str().unwrap().to_string()
    }
}
