use std::ffi::{CStr, CString, NulError};
use std::fmt::{Display, Debug};
use std::error::Error;
use std::str::Utf8Error;

use libc::c_char;

use crate::bindings::tsk_error_get;

pub type TskResult<_T> = Result<_T, TskError>;

pub struct TskError {
    str: *const c_char,
}

impl TskError {

    pub fn get_err() -> TskResult<()> {
        let ptr = unsafe {
            tsk_error_get()
        };

        if ptr.is_null() {
            Err(TskError{str: "uspecified error\0".as_ptr() as *const c_char})
        } else {
            Err(TskError{str: ptr})
        }
    }
}

impl Debug for TskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = unsafe {
            CStr::from_ptr(self.str).to_str().unwrap()
        };
        f.debug_struct("TskError").field("*str", &str).finish()
    }
}

impl Display for TskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = unsafe {
            CStr::from_ptr(self.str).to_str().unwrap()
        };
        write!(f, "{}", str)
    }
}

impl Error for TskError {}


impl From<Utf8Error> for TskError {
    fn from(_: Utf8Error) -> Self {
        let s: &'static str = "Utf8Error, error while converting strings\0";
        TskError{
            str: s.as_ptr() as *const c_char,
        }
    }
}

impl From<NulError> for TskError {
    fn from(_: NulError) -> Self {
        let s: &'static str = "Utf8Error, error while converting strings\0";
        TskError{
            str: s.as_ptr() as *const c_char,
        }
    }
}




