use crate::bindings::*;
use std::ffi::CStr;
use std::str;
use std::string::String;

pub struct Tchar(*const TSK_TCHAR);

// #[derive(Debug, Clone)]
// pub struct Tchar {
//     pub inner:  *const TSK_TCHAR,
//     buff: String,
// }

impl Tchar {
    pub fn is_empty(&self) -> bool {
        unsafe { *self.0.add(1) == 0 }
    }
}

impl<T: AsRef<str>> From<T> for Tchar {
    fn from(s: T) -> Tchar {
        Tchar(s.as_ref().as_ptr() as *const TSK_TCHAR)
    }
}

impl From<Tchar> for String {
    fn from(val: Tchar) -> Self {
        unsafe { CStr::from_ptr(val.0) }.to_str().unwrap().to_owned()
    }
}
