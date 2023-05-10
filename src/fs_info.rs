use crate::bindings::*;
use crate::entry::Dir;
use crate::error::{Nullptr, TskResult};
use crate::tchar::Tchar;
use std::fmt::Display;
use std::rc::Rc;

use crate::entry::DirWrapper;

#[derive(Debug)]
struct FsInfo(*mut TSK_FS_INFO);

impl Drop for FsInfo {
    fn drop(&mut self) {
        // println!("droping fs");
        unsafe {
            tsk_fs_close(self.0);
        }
    }
}


impl FsInfo {

    pub fn open_dir<T: Into<Tchar> + Display + Clone>(&self, path: T) -> TskResult<Dir> {
        let t: Tchar = path.into();
        if t.is_empty() {
            return Err(crate::error::TskError::Str("empty path"));
        }
        let ptr = unsafe { tsk_fs_dir_open(self.0, t.inner) };

        if ptr.is_null() {
            Err(Nullptr::DirOpen)?;
        }

        Ok(Dir {
            inner: Rc::new(DirWrapper {
                inner: ptr,
                parent: self.0.clone(),
                file: None,
            }),
        })
    }

    pub fn root(&self) -> TskResult<Dir> {
        let ptr = unsafe { tsk_fs_dir_open_meta(self.0, (*self.0).root_inum) };

        if ptr.is_null() {
            Err(Nullptr::DirOpen)?;
        }

        Ok(Dir {
            inner: Rc::new(DirWrapper {
                inner: ptr,
                parent: self.clone(),
                file: None,
            }),
        })
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
