
use crate::bindings::*;
use std::str;
use std::string::String;


#[derive(Debug, Clone)]
pub struct Tchar {
    pub inner:  *const TSK_TCHAR,
    buff: String,
}

impl From<String> for Tchar {
   fn from(s: String) -> Tchar {
       Tchar {inner: s.as_ptr() as *const TSK_TCHAR, buff: s}
   }
}
impl From<&str> for Tchar {
   fn from(s: &str) -> Tchar {
       let buff = s.to_string();
       Tchar {inner: buff.as_ptr() as *const TSK_TCHAR, buff}
   }
}

impl From<Tchar> for String {
    fn from(val: Tchar) -> Self {
        val.buff
    }
}
