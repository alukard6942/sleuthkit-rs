
use crate::bindigs::*;

#[derive(Debug)]
pub struct Tchar {
    pub inner:  *const TSK_TCHAR,
    buff: String,
}

use std::str;
use std::string::String;

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

// use std::ptr;
// impl From<Tchar> for String {
//     fn from(tchar : Tchar) -> String {
//         let v =  tchar.inner as *const u8;
//         let len: usize = unsafe {
//
//             let mut i: isize = 0;
//             loop { 
//                 let c = *v.offset(i);
//                 if c == 0 { break; }
//                 i += 1;
//             };
//
//             i as usize
//         };
//
//         let mut bff = Vec::with_capacity(len);
//         unsafe {
//             ptr::copy(v, bff.as_mut_ptr(), len);
//             bff.set_len(len);
//         }
//
//         String::from_utf8(bff).unwrap()
//     }
// }
