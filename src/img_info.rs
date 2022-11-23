use crate::bindings::*;
use crate::error::{Nullptr, TskResult};
use crate::fs_info::{FsInfo, FsWrapper};
use crate::tchar::Tchar;
use crate::vs_info::VsInfo;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub struct ImgWrapper {
    pub inner: *mut TSK_IMG_INFO,
}

#[derive(Debug)]
pub struct ImgInfo {
    pub inner: Rc<ImgWrapper>,
}

impl ImgInfo {
    pub fn new<T: Into<Tchar> + Display + Clone>(path: T) -> TskResult<Self> {
        let tchar: Tchar = path.clone().into();
        let ptr =
            unsafe { tsk_img_open_sing(tchar.inner, TSK_IMG_TYPE_ENUM_TSK_IMG_TYPE_DETECT, 0) };

        if ptr.is_null() {
            Err(Nullptr::ImgOpen)?;
        }

        Ok(ImgInfo {
            inner: Rc::new(ImgWrapper { inner: ptr }),
        })
    }

    pub fn itype(&self) -> String {
        let itype = unsafe { (*self.inner.inner).itype };

        format!("{}", itype)
    }
    pub fn vs_info(&self) -> TskResult<VsInfo> {
        let ptr = unsafe { tsk_vs_open(self.inner.inner, 0, TSK_VS_TYPE_ENUM_TSK_VS_TYPE_DETECT) };

        if ptr.is_null() {
            Err(Nullptr::VsOpen)?
        }

        Ok(VsInfo { inner: ptr })
    }

    pub fn fs_info(&self) -> TskResult<FsInfo> {
        let ptr =
            unsafe { tsk_fs_open_img(self.inner.inner, 0, TSK_FS_TYPE_ENUM_TSK_FS_TYPE_DETECT) };

        if ptr.is_null() {
            Err(Nullptr::FsOpen)?
        }

        Ok(FsInfo {
            inner: Rc::new(FsWrapper {
                inner: ptr,
                parent: Rc::clone(&self.inner),
            }),
        })
    }
}

impl Drop for ImgWrapper {
    fn drop(&mut self) {
        // println!("droping img");
        unsafe {
            tsk_img_close(self.inner);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::img_info::ImgInfo;

    pub fn new() -> ImgInfo {
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
