#[derive(Debug)]
pub enum Nullptr {
    ImgOpen,
    VsOpen,
    FsOpen,
    DirOpen,
    FileOpen,
    Meta,
}

use std::ffi::CStr;
use std::fmt::Display;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum TskError {
    Nullptr(Nullptr),
    Msg(String),
    Str(&'static str),
    Cstr(Utf8Error),
}

impl Error for TskError {}

impl From<Nullptr> for TskError {
    fn from(t: Nullptr) -> Self {
        TskError::Nullptr(t)
    }
}

impl From<String> for TskError {
    fn from(t: String) -> Self {
        TskError::Msg(t)
    }
}

impl From<&'static str> for TskError {
    fn from(t: &'static str) -> Self {
        TskError::Str(t)
    }
}

impl From<Utf8Error> for TskError {
    fn from(t: Utf8Error) -> Self {
        TskError::Cstr(t)
    }
}


impl Display for TskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::bindings::tsk_error_get;
        use std::fmt::Error;

        let s = unsafe { CStr::from_ptr(tsk_error_get()) };

        let st = s.to_str();

        if st.is_err() {
            return Err(Error);
        }

        write!(f, "{}", st.unwrap())
    }
}

use std::error::Error;
pub type TskResult<_T> = Result<_T, TskError>;
