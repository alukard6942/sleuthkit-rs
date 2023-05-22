use libc::c_char;

use crate::bindings::*;
use crate::error::{Nullptr, TskError, TskResult};
use crate::fs::fs_info::FsInfo;
use crate::tchar::{AsTchar, Tchar};
use crate::vs::vs_info::VsInfo;
use std::ffi::{c_int, c_uint, CString};
use std::ops::Deref;
use std::os::unix::prelude::{AsFd, AsRawFd};
use std::usize;

pub struct ImgInfo(pub *mut TSK_IMG_INFO);

unsafe impl Send for ImgInfo {}
unsafe impl Sync for ImgInfo {}

impl std::fmt::Debug for ImgInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.itype().to_desc())
    }
}

impl Drop for ImgInfo {
    fn drop(&mut self) {
        // println!("droping img");
        unsafe {
            tsk_img_close(self.0);
        }
    }
}

impl Deref for ImgInfo {
    type Target = *mut TSK_IMG_INFO;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ImgInfo {
    // ekvivalent to open_sing(path, TSK_IMG_TYPE_ENUM::TSK_IMG_TYPE_DETECT, 0)
    pub fn new<T: AsRef<str>>(path: T) -> TskResult<Self> {
        let cs = CString::new(path.as_ref())?;
        let t: Tchar = cs.as_tchar();

        let ptr = unsafe { tsk_img_open_sing(*t, TSK_IMG_TYPE_ENUM::TSK_IMG_TYPE_DETECT, 0) };
        if ptr.is_null() {
            Err(Nullptr::ImgOpen)?;
        }
        Ok(ImgInfo(ptr))
    }

    pub fn open_sing<T: AsRef<str>>(
        path: T,
        img_type: TSK_IMG_TYPE_ENUM,
        ssize: usize,
    ) -> TskResult<Self> {
        let cs = CString::new(path.as_ref())?;
        let t: Tchar = cs.as_tchar();
        let ptr = unsafe { tsk_img_open_sing(*t, img_type, ssize as c_uint) };
        if ptr.is_null() {
            Err(Nullptr::ImgOpen)?;
        }
        Ok(ImgInfo(ptr))
    }

    pub fn open<T: AsRef<str>>(
        path: &[T],
        img_type: TSK_IMG_TYPE_ENUM,
        ssize: usize,
    ) -> TskResult<Self> {
        let mut ts: Vec<Tchar> = Vec::new();
        for p in path {
            let cs = CString::new(p.as_ref())?;
            let t: Tchar = cs.as_tchar();
            ts.push(t);
        }
        let ptr = unsafe {
            tsk_img_open(
                path.len() as c_int,
                ts.as_slice().as_ptr() as *const *const TSK_TCHAR,
                img_type,
                ssize as c_uint,
            )
        };
        if ptr.is_null() {
            Err(Nullptr::ImgOpen)?;
        }
        Ok(ImgInfo(ptr))
    }

    pub fn read_at(&self, offset: usize, buf: &mut [u8]) -> TskResult<usize> {
        let read = unsafe {
            tsk_img_read(
                self.0,
                offset as TSK_OFF_T,
                buf.as_mut_ptr() as *mut i8,
                buf.len(),
            )
        };

        if read < 0 {
            Err(TskError::Str("read_at"))
        } else {
            Ok(read as usize)
        }
    }

    pub fn print<T: AsFd>(f: T) {
        let fd = f.as_fd().as_raw_fd();
        let mode = "w";

        unsafe {
            let file = libc::fdopen(fd, mode.as_ptr() as *const c_char);
            tsk_img_type_print(file as *mut _IO_FILE);
            libc::fflush(file);
        }
    }

    pub fn itype(&self) -> TSK_IMG_TYPE_ENUM {
        unsafe { (*self.0).itype }
    }

    pub fn vs<'a>(&'a self) -> TskResult<VsInfo<'a>> {
        let ptr = unsafe { tsk_vs_open(self.0, 0, TSK_VS_TYPE_ENUM::TSK_VS_TYPE_DETECT) };
        if ptr.is_null() {
            Err(Nullptr::VsOpen)?
        }
        Ok(VsInfo::new(ptr))
    }

    pub fn vs_open<'a>(&'a self, addr: usize, vs_type: TSK_VS_TYPE_ENUM) -> TskResult<VsInfo<'a>> {
        let ptr = unsafe { tsk_vs_open(self.0, addr as TSK_DADDR_T, vs_type) };
        if ptr.is_null() {
            Err(Nullptr::VsOpen)?
        }
        Ok(VsInfo::new(ptr))
    }

    pub fn fs<'a>(&'a self) -> TskResult<FsInfo<'a>> {
        let ptr = unsafe { tsk_fs_open_img(self.0, 0, TSK_FS_TYPE_ENUM::TSK_FS_TYPE_DETECT) };

        if ptr.is_null() {
            Err(Nullptr::FsOpen)?
        }

        Ok(FsInfo::new(ptr))
    }
    pub fn fs_open<'a>(
        &'a self,
        offset: usize,
        fs_type: TSK_FS_TYPE_ENUM,
    ) -> TskResult<FsInfo<'a>> {
        let ptr = unsafe { tsk_fs_open_img(self.0, offset as TSK_OFF_T, fs_type) };

        if ptr.is_null() {
            Err(Nullptr::FsOpen)?
        }

        Ok(FsInfo::new(ptr))
    }
}

#[cfg(test)]
pub mod tests {

    use std::io;
    use super::*;

    pub fn new() -> ImgInfo {
        match ImgInfo::new("testData/test.iso") {
            Ok(it) => it,
            Err(err) => {
                println!("{}", err);
                panic!()
            }
        }
    }

    #[test]
    fn load_iso() {
        new();
    }

    #[test]
    fn itype() {
        let img = new();
        let _t = img.itype().to_name();
    }

    // #[test]
    // fn print() {
    //     let f = io::stdout();
    //     ImgInfo::print(&f);
    // }

    #[test]
    fn name() {
        println!("{:?}", new());
    }
}
