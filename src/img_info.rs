use crate::bindings::*;
use crate::error::{Nullptr, TskResult};
use crate::fs_info::FsInfo;
use crate::tchar::Tchar;
use crate::vs_info::{VsInfo, VsWrapper};
use std::ffi::CStr;
use std::fmt::Display;
use std::rc::Rc;

type CUint = ::std::os::raw::c_uint;

#[doc = " Flags for the partition type."]
#[repr(CUint)]
enum ImgType {
    #[doc = "< Use autodetection methods"]
    DETECT = 0,
    #[doc = "< DOS Partition table"]
    DOS,
    #[doc = "< BSD Partition table"]
    BSD,
    #[doc = "< Sun VTOC"]
    SUN,
    #[doc = "< Mac partition table"]
    MAC,
    #[doc = "< GPT partition table"]
    GPT,
    #[doc = "< APFS"]
    APFS,
    #[doc = "< fake partition table type for loaddb (for images that do not have a volume system)"]
    DBFILLER,
    #[doc = "< Unsupported"]
    UNSUPP,
}

#[derive(Debug)]
pub struct ImgInfo(*mut TSK_IMG_INFO);

impl Drop for ImgInfo {
    fn drop(&mut self) {
        // println!("droping img");
        unsafe {
            tsk_img_close(self.0);
        }
    }
}

impl ImgInfo {
    pub fn new<T: Into<Tchar> + Display + Clone>(path: T) -> TskResult<Self> {
        let tchar: Tchar = path.into();
        let ptr =
            unsafe { tsk_img_open_sing(tchar.inner, TSK_IMG_TYPE_ENUM_TSK_IMG_TYPE_DETECT, 0) };

        if ptr.is_null() {
            Err(Nullptr::ImgOpen)?;
        }

        Ok(ImgInfo(ptr))
    }


    pub fn open_sing<T: Into<Tchar>>(path: T) -> TskResult<Self> {
        let t = path.into();

        let ptr =
            unsafe { tsk_img_open_sing(t.inner, TSK_IMG_TYPE_ENUM_TSK_IMG_TYPE_DETECT, 0) };

        if ptr.is_null() {
            Err(Nullptr::ImgOpen)?;
        }

        Ok(ImgInfo(ptr))
    }


    pub fn desc(&self) -> String {
        let itype = unsafe { (*self.0).itype };

        let s = unsafe { CStr::from_ptr(tsk_img_type_todesc(itype)) };

        format!("{}", s.to_str().unwrap())
    }
    pub fn vs_open(&self) -> TskResult<VsInfo> {
        let ptr = unsafe { tsk_vs_open(self.0, 0, TSK_VS_TYPE_ENUM_TSK_VS_TYPE_DETECT) };

        if ptr.is_null() {
            Err(Nullptr::VsOpen)?
        }

        Ok(VsInfo {
            inner: Rc::new(VsWrapper { inner: ptr }),
        })
    }

    pub fn fs_open_img(&self) -> TskResult<FsInfo> {
        let ptr = unsafe { tsk_fs_open_img(self.0, 0, TSK_FS_TYPE_ENUM_TSK_FS_TYPE_DETECT) };

        if ptr.is_null() {
            Err(Nullptr::FsOpen)?
        }

        Ok(FsInfo(ptr))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::img_info::ImgInfo;

    pub fn new() -> ImgInfo {
        let arg = "testData/test.iso";
        ImgInfo::new(arg).unwrap()
    }

    #[test]
    fn load_iso() {
        new();
    }

    #[test]
    fn itype() {
        let img = new();
        let _t = img.desc();
    }
}
