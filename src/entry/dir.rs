use crate::bindings::*;
use crate::error::DResult;
use crate::fs_info::FsWrapper;
use std::ffi::CStr;
use std::fmt::Display;
use std::rc::Rc;

use super::file::File;
use super::helpers::*;

#[derive(Debug, Clone)]
pub struct DirWrapper {
    pub inner: *mut TSK_FS_DIR,
    pub parent: Rc<FsWrapper>,
}

#[derive(Debug, Clone)]
pub struct Dir {
    pub inner: Rc<DirWrapper>,
}

impl Dir {
    pub fn name_of(&self, dx: usize) -> Result<&str, std::str::Utf8Error> {
        let tmp = unsafe {
            let ptr = *tsk_fs_dir_get_name(self.inner.inner, dx);
            CStr::from_ptr(ptr.name)
        };
        tmp.to_str()
    }

    pub fn name(&self) -> DResult<&str> {
        let s = unsafe {
            CStr::from_ptr({
                let inner = (*self.inner.inner).fs_file;
                if inner.is_null() {
                    Err("fs_file")?
                }
                let name = (*inner).name;
                if name.is_null() {
                    Err("name is null")?
                }
                (*name).name
            })
        };

        Ok(s.to_str()?)
    }

    pub fn iter(&self) -> DirIter {
        DirIter {
            count: 0,
            parent: (*self).clone(),
        }
    }

    pub fn nth(&self, i: usize) -> Option<File> {
        let file = unsafe { tsk_fs_dir_get(self.inner.inner, i) };

        if file.is_null() {
            None
        } else {
            Some(File {
                inner: file,
                parent: Rc::clone(&self.inner.parent),
            })
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = match self.name() {
            Ok(it) => it,
            Err(_) => return Err(std::fmt::Error),
        };

        write!(f, "{}", m)
    }
}

impl Drop for DirWrapper {
    fn drop(&mut self) {
        // println!("droping dir");
        unsafe { tsk_fs_dir_close(self.inner) }
    }
}

#[cfg(test)]
mod tests {
    use crate::img_info;

    use super::*;

    #[test]
    fn bla() {}

    fn root() -> Dir {
        let img = img_info::tests::new();
        let fs = img.fs_info().unwrap();
        let root = fs.root().unwrap();

        root
    }

    #[test]
    fn name_of() {
        let root = root();
        let name = root.name_of(0).unwrap();

        assert_eq!(name, ".");
    }

    #[test]
    #[should_panic]
    fn root_has_name() {
        let root = root();

        let name = root.name().unwrap();

        assert_eq!(name, ".");
    }

    #[test]
    fn a_is_dir() {
        let root = root();

        let a = root.nth(2).unwrap();

        assert!({ a.is_dir() });
    }

    #[test]
    fn iterator() {
        let root = root();

        let mut buffer = String::new();

        for f in root.iter() {
            let name = f.name().unwrap();
            buffer += name;
        }

        assert_eq!(buffer, "...AGPL_3_0.TXT$OrphanFiles")
    }

    #[test]
    fn intoiter() {
        let root = root();
        let mut buffer = String::new();

        for f in &root {
            let name = f.name().unwrap();
            buffer += name;
        }
        assert_eq!(buffer, "...AGPL_3_0.TXT$OrphanFiles");
        buffer.clear();

        for f in &root {
            let name = f.name().unwrap();
            buffer += name;
        }
        assert_eq!(buffer, "...AGPL_3_0.TXT$OrphanFiles")
    }
}
