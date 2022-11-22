#[derive(Debug)]
pub enum Nullptr {
    ImgOpen,
    VsOpen,
    FsOpen,
    DirOpen,
    FileOpen,
}

use std::ffi::CStr;
use std::fmt::Display;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum TskError {
    Nullptr(Nullptr),
    Dynamic(Box<dyn Error>),
    Msg(String),
    Cstr(Utf8Error),
}

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

impl From<&str> for TskError {
    fn from(t: &str) -> Self {
        TskError::Msg(t.to_string())
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
pub type DResult<_T> = Result<_T, TskError>;
