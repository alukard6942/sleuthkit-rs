/**
 * File: dir.rs
 * Author: alukard <alukard6942@github>
 * Date: 23.10.2022
 * Last Modified Date: 23.10.2022
 */

use crate::bindings::*;
use std::ffi::CStr;
use std::fmt::Display;

use super::file::File;
use super::helpers::*;
use super::file;


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

    pub fn name (&self) -> Result<&str, std::str::Utf8Error> {
        let s = unsafe {
            CStr::from_ptr(
                (*(*(*self.inner).fs_file).name).name
            )
        };

        s.to_str()
    }

    pub fn iter(&self) -> DirIter<'_> {
        DirIter { count: 0, parent: self }
    }

    pub fn nth(&self, i: usize) -> Option<DirEntry> {
        let file = unsafe {
            tsk_fs_dir_get(self.inner, i)
        };

        if file.is_null() {
            None
        }
        else{
            Some( DirEntry::File( file::File {
                inner: file,
            }))
        }
    }
}

impl IntoIterator for Dir {
    type Item;

    type IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
    use crate::entry::DirEntry;

    #[test]
    fn bla() {
    }

    #[test]
    fn name(){
        let img = img_info::tests::new();
        let fs = img.fs_info().unwrap();
        let root = fs.root().unwrap();

        let name = root.name_of(0).unwrap();

        assert_eq!(name, ".");
    }

    #[test]
    fn iterator() {
        let img = img_info::tests::new();
        let fs = img.fs_info().unwrap();
        let root = fs.root().unwrap();

        for f in root.iter() {

            if let DirEntry::File(i) = f  {
                println!("{:?}", i);
            }

        }

        assert_eq!(true, false)
    }
}

