use crate::bindings::*;
use crate::entry::Dir;
use crate::error::{Nullptr, TskResult};
use crate::img_info::ImgWrapper;
use crate::tchar::Tchar;
use std::fmt::Display;
use std::rc::Rc;

use crate::entry::DirWrapper;

#[derive(Debug)]
pub struct FsWrapper {
    pub inner: *mut TSK_FS_INFO,
    pub parent: Rc<ImgWrapper>,
}

#[derive(Debug)]
pub struct FsInfo {
    pub inner: Rc<FsWrapper>,
}

impl FsInfo {
    pub fn open_dir<T: Into<Tchar> + Display + Clone>(&self, path: T) -> TskResult<Dir> {
        let t: Tchar = path.into();
        let ptr = unsafe { tsk_fs_dir_open(self.inner.inner, t.inner) };

        if ptr.is_null() {
            Err(Nullptr::DirOpen)?;
        }

        Ok(Dir {
            inner: Rc::new(DirWrapper {
                inner: ptr,
                parent: self.inner.clone(),
            }),
        })
    }

    pub fn root(&self) -> TskResult<Dir> {
        let ptr = unsafe { tsk_fs_dir_open_meta(self.inner.inner, (*self.inner.inner).root_inum) };

        if ptr.is_null() {
            Err(Nullptr::DirOpen)?;
        }

        Ok(Dir {
            inner: Rc::new(DirWrapper {
                inner: ptr,
                parent: self.inner.clone(),
            }),
        })
    }
}

impl Drop for FsWrapper {
    fn drop(&mut self) {
        // println!("droping fs");
        unsafe {
            tsk_fs_close(self.inner);
        }
    }
}

#[cfg(test)]
pub mod tests {

    use super::FsInfo;
    use crate::{entry::Dir, img_info};

    pub fn new() -> FsInfo {
        let img = img_info::tests::new();

        img.fs_info().unwrap()
    }

    pub fn root() -> Dir {
        let img = img_info::tests::new();
        let fs = img.fs_info().unwrap();

        fs.root().unwrap()
    }

    #[test]
    fn open() {
        let f = new();

        f.root().unwrap();
    }

    #[test]
    fn openroot() {
        root();
    }
}
