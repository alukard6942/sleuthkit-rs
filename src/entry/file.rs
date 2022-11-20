/**
 * File: file.rs
 * Author: alukard <alukard6942@github>
 * Date: 25.10.2022
 * Last Modified Date: 25.10.2022
 */

use crate::{bindings::*, error::DResult};
use std::{ffi::CStr, fmt::Display};
use super::dir::Dir;


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

    pub fn is_file(&self) -> bool {
        let meta = unsafe {
            // the field name is type but that happens to be reserved by rust
            (*(*self.inner).meta).type_
        };

        // lol no cast from u32 to bool pathetic
        (meta & TSK_FS_META_TYPE_ENUM_TSK_FS_META_TYPE_DIR ) == 0
    }

    /*
     ** Is this string a "." or ".."
     */
    pub fn is_dot(&self) -> bool {
        unsafe {
            let ptr = (*(*self.inner).name).name;
            if ptr.is_null() { return false }

            // viz macro
            (*ptr == '.' as i8) &&
                ( ((*ptr.add(1) =='.' as i8) && (*ptr.add(2) == '\0' as i8)) ||
                  (*ptr.add(1) =='\0' as i8))
        }
    }

    pub fn is_dir(&self) -> bool {
        ! self.is_file()
    }

    pub fn is_dir_not_dot(&self) -> bool {
        self.is_dir() && !self.is_dot()

    }

    pub fn to_dir(&self) -> Option<Dir> {

        unsafe {
            tsk_fs_dir_open_meta
        }



        None
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
