/**
 * File: fs_info.rs
 * Author: alukard <alukard6942@github>
 * Date: 23.10.2022
 * Last Modified Date: 23.10.2022
 */

use std::fmt::Display;
use crate::entry::Dir;
use crate::bindings::*;
use crate::error::{DResult, Nullptr};
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
            Err(Nullptr::DirOpen)?;
        }

        Ok(Dir { inner: ptr, })
    }

    pub fn root(&self) -> DResult<Dir> {
        let ptr = unsafe {
            tsk_fs_dir_open_meta(self.inner, 0)
        };

        if ptr.is_null() {
            Err(Nullptr::DirOpen)?;
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

    use crate::{entry::Dir, img_info};
    use super::FsInfo;

    pub fn new () -> FsInfo {
        let img = img_info::tests::new();

        img.fs_info().unwrap()
    }

    pub fn root () -> Dir {
        let img = img_info::tests::new();
        let fs = img.fs_info().unwrap();

        fs.root().unwrap()
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
