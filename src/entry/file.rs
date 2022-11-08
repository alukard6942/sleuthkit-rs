/**
 * File: file.rs
 * Author: alukard <alukard6942@github>
 * Date: 25.10.2022
 * Last Modified Date: 25.10.2022
 */

use crate::bindings::*;
use std::{ffi::CStr, fmt::Display};


#[derive(Debug)]
pub struct File {
    pub inner: *mut TSK_FS_FILE,
}


impl File {

    pub fn name(&self) -> Result<&str, std::str::Utf8Error> {
        let s = unsafe {
            CStr::from_ptr(
                (*(*self.inner).name).name
            )
        };

        s.to_str()
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let m = match self.name() {
            Ok(it) => it,
            Err(_) => return Err(std::fmt::Error),
        };

        write!(f, "{}", m)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            tsk_fs_file_close(self.inner);
        }
    }
}
