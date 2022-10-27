/**
 * File: img_info.rs
 * Author: alukard <alukard6942@github>
 * Date: 23.10.2022
 * Last Modified Date: 23.10.2022
 */

use crate::bindigs::*;
use crate::fs_info::FsInfo;
use crate::tchar::Tchar;
use crate::vs_info::VsInfo;
use std::fmt::Display;
use crate::error::{DResult, Nullptr};


#[derive(Debug)]
pub struct ImgInfo {
    pub inner: *mut TSK_IMG_INFO,
}

impl ImgInfo {
    pub fn new<T: Into<Tchar> + Display + Clone>(path: T) -> DResult<Self> {
        let tchar : Tchar = path.clone().into();
        let ptr = unsafe {
            tsk_img_open_sing(
                tchar.inner,
                TSK_IMG_TYPE_ENUM_TSK_IMG_TYPE_DETECT,
                0,
            )
        };

        if ptr.is_null() {
            Err(Nullptr::ImgOpen)?;
        }

        Ok(ImgInfo { inner: ptr, })
    }

    pub fn itype(&self) -> String {
        let itype = unsafe {
          (*self.inner).itype  
        };

        format!("{}", itype)
    }
    pub fn vs_info(&self) -> DResult<VsInfo> {
        let ptr =  unsafe {
            tsk_vs_open(self.inner , 0, TSK_VS_TYPE_ENUM_TSK_VS_TYPE_DETECT)
        };

        if ptr.is_null() {
            Err(Nullptr::VsOpen)?
        }


        Ok(VsInfo { inner : ptr, })
    }

    pub fn fs_info(&self) -> DResult<FsInfo> {
        let ptr =  unsafe {
            tsk_fs_open_img(self.inner, 0, TSK_FS_TYPE_ENUM_TSK_FS_TYPE_DETECT)
        };

        if ptr.is_null() {
            Err(Nullptr::FsOpen)?
        }

        Ok(FsInfo { inner : ptr, })
    }
}

impl Drop for ImgInfo {
    fn drop(&mut self) {
        unsafe {
            tsk_img_close(self.inner);
        }
    }
}


#[cfg(test)]
pub mod tests {
    use crate::img_info::ImgInfo;

    pub fn new() -> ImgInfo{
        let arg = "./test.iso";
        ImgInfo::new(arg).unwrap()
    }

    #[test]
    fn load_iso() {
        new();
    }

    #[test]
    fn itype() {
        let img = new();
        let _t = img.itype();
    }
}
