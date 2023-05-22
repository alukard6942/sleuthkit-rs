use crate::bindings::*;
use crate::error::TskResult;
use crate::fs::fs_info::FsInfo;
use std::ffi::CStr;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Deref;

use super::file::File;
use super::helpers::*;


#[derive(Debug)]
pub struct Dir<'a>(*mut TSK_FS_DIR, PhantomData<&'a FsInfo<'a>>);

impl Drop for Dir<'_> {
    fn drop(&mut self) {
        // println!("droping dir");
        unsafe { tsk_fs_dir_close(self.0) }
    }
}

impl Deref for Dir<'_> {
    type Target = *mut TSK_FS_DIR;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Dir<'a> {
    pub(crate) fn new(ptr: *mut TSK_FS_DIR ) -> Dir<'a> {
        Dir(ptr, PhantomData)
    }

    pub fn get_name(&self, dx: usize) -> Result<&str, std::str::Utf8Error> {
        let tmp = unsafe {
            let ptr = *tsk_fs_dir_get_name(self.0, dx);
            CStr::from_ptr(ptr.name)
        };
        tmp.to_str()
    }

    pub fn get_size(&self) -> usize {
        unsafe { tsk_fs_dir_getsize(self.0) }
    }

    pub fn name(&self) -> TskResult<&str> {
        let s = unsafe {
            CStr::from_ptr({
                let inner = (*self.0).fs_file;
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

    pub fn iter(&'a self) -> DirIter<'a> {
        DirIter {
            count: 0,
            parent: self,
        }
    }

    pub fn get(&self, i: usize) -> Option<File> {
        let file = unsafe { tsk_fs_dir_get(self.0, i) };

        if file.is_null() {
            None
        } else {
            Some(File::new(file))
        }
    }
    #[inline]
    pub fn nth(&self, i: usize) -> Option<File> {
        self.get(i)
    }

}

impl Display for Dir<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = match self.name() {
            Ok(it) => it,
            Err(_) => return Err(std::fmt::Error),
        };

        write!(f, "{}", m)
    }
}


#[cfg(test)]
mod tests {

    use crate::img::img_info;

    use super::*;

    #[test]
    fn bla() {}


    #[test]
    fn name_of() {
        let img = img_info::tests::new();
        let fs = img.fs().unwrap();
        let root = fs.dir_open_root().unwrap();

        let name = root.get_name(0).unwrap();

        assert_eq!(name, ".");
    }

    #[test]
    #[should_panic]
    fn root_has_name() {
        let img = img_info::tests::new();
        let fs = img.fs().unwrap();
        let root = fs.dir_open_root().unwrap();

        let name = root.name().unwrap();

        assert_eq!(name, ".");
    }

    #[test]
    fn a_is_dir() {
        let img = img_info::tests::new();
        let fs = img.fs().unwrap();
        let root = fs.dir_open_root().unwrap();

        let a = root.nth(2).unwrap();

        assert!({ a.is_dir() });
    }

    #[test]
    fn iterator() {
        let img = img_info::tests::new();
        let fs = img.fs().unwrap();
        let root = fs.dir_open_root().unwrap();

        let mut buffer = String::new();

        for f in root.iter() {
            let name = f.name().unwrap();
            buffer += name;
        }

        assert_eq!(buffer, "...AGPL_3_0.TXT$OrphanFiles")
    }

    #[test]
    fn intoiter() {
        let img = img_info::tests::new();
        let fs = img.fs().unwrap();
        let root = fs.dir_open_root().unwrap();

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
