/**
 * File: fs_info.rs
 * Author: alukard <alukard6942@github>
 * Date: 23.10.2022
 * Last Modified Date: 23.10.2022
 */

use std::fmt::Display;
use crate::bindigs::TSK_FS_INFO;
use crate::dir::Dir;
use crate::bindigs::*;
use crate::error::DResult;
use crate::tchar::Tchar;


#[derive(Debug)]
pub struct FsInfo {
    pub inner: *mut TSK_FS_INFO
}

impl FsInfo {
    pub fn open_dir<T: Into<Tchar> + Display + Clone>(&self, path: T) -> DResult<Dir> {
        let t: Tchar = path.into();
        let ptr = unsafe {
            tsk_fs_dir_open(self.inner, t.inner)
        };
        
        if ptr.is_null() {
            Err("null ptr")?;
        }

        Ok(Dir { inner: ptr, })
    }
}

impl Drop for FsInfo {
    fn drop(&mut self) {
        unsafe {
            tsk_fs_close(self.inner);
        }
    }

}


#[cfg(test)]
mod tests {

    use crate::{img_info, dir::Dir};
    use super::FsInfo;

    pub fn new () -> FsInfo {
        let img = img_info::tests::new();
        img.fs_info().unwrap()
    }

    pub fn root () -> Dir {
        let fs = new();
        fs.open_dir("/").unwrap()
    }

    #[test]
    fn open () {
        new();
    }
    
    #[test]
    fn openroot () {
        root();
    }
}
