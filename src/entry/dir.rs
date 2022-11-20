/**
 * File: dir.rs
 * Author: alukard <alukard6942@github>
 * Date: 23.10.2022
 * Last Modified Date: 23.10.2022
 */

use crate::bindings::*;
use crate::error::DResult;
use std::ffi::CStr;
use std::fmt::Display;

use super::file::File;
use super::helpers::*;


#[derive(Debug)]
pub struct Dir {
    pub inner: *mut TSK_FS_DIR,
}

impl Dir {
    pub fn name_of(&self, dx: usize) -> Result<&str, std::str::Utf8Error> {
        let tmp = unsafe {
            let ptr = *tsk_fs_dir_get_name(self.inner, dx);
            CStr::from_ptr(ptr.name)
        };
        tmp.to_str()
    }

    pub fn name(&self) -> DResult<&str> {
        let s = unsafe {
            CStr::from_ptr({
                let inner = (*self.inner).fs_file;
                if inner.is_null() {Err("fs_file")?}
                let name = (*inner).name;
                if name.is_null() {Err("name")?}
                (*name).name
            })
        };

        Ok(s.to_str()?)
    }

    pub fn iter(&self) -> DirIter<'_> {
        DirIter { count: 0, parent: self }
    }

    pub fn nth(&self, i: usize) -> Option<File> {
        let file = unsafe {
            tsk_fs_dir_get(self.inner, i)
        };

        if file.is_null() {
            None
        }
        else{
            Some(File {
                inner: file,
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

impl Drop for Dir {
    fn drop(&mut self) {
        unsafe {
            tsk_fs_dir_close(self.inner)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::img_info;

    #[test]
    fn bla() {
    }

    #[test]
    fn name_of(){
        let img = img_info::tests::new();
        let fs = img.fs_info().unwrap();
        let root = fs.root().unwrap();

        let name = root.name_of(0).unwrap();

        assert_eq!(name, ".");
    }

    #[test]
    #[should_panic]
    fn root_has_name(){
        let img = img_info::tests::new();
        let fs = img.fs_info().unwrap();
        let root = fs.root().unwrap();

        let name = root.name().unwrap();

        assert_eq!(name, ".");
    }

    #[test]
    fn a_is_dir(){
        let img = img_info::tests::new();
        let fs = img.fs_info().unwrap();
        let root = fs.root().unwrap();

        let a = root.nth(2).unwrap();

        assert!({ 
            a.is_dir()
        });
    }

    #[test]
    fn iterator() {
        let img = img_info::tests::new();
        let fs = img.fs_info().unwrap();
        let root = fs.root().unwrap();

        let mut buffer = String::new();

        for f in root.iter() {
            let name = f.name().unwrap();
            buffer += name;
        }

        assert_eq!(buffer, "...AGPL_3_0.TXT$OrphanFiles")
    }
}

