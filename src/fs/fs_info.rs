use crate::bindings::*;
use crate::entry::{Dir, File};
use crate::error::{TskResult, TskError};
use crate::img::img_info::ImgInfo;
use crate::tchar::{AsTchar, Tchar};
use core::marker::PhantomData;
use std::ffi::CString;
use std::mem::forget;
use std::ops::Deref;
use std::ptr::null_mut;

use super::fs_block::FsBlock;

#[derive(Debug)]
pub struct FsInfo<'a>(*mut TSK_FS_INFO, PhantomData<&'a ImgInfo>);

impl Deref for FsInfo<'_> {
    type Target = *mut TSK_FS_INFO;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for FsInfo<'_> {
    fn drop(&mut self) {
        // println!("droping fs");
        unsafe {
            tsk_fs_close(self.0);
        }
    }
}

impl<'a> FsInfo<'a> {
    pub(crate) fn new(ptr: *mut TSK_FS_INFO) -> FsInfo<'a> {
        FsInfo(ptr, PhantomData)
    }

    pub fn block_get_flag(
        &'a self,
        addr: usize,
        flags: TSK_FS_BLOCK_FLAG_ENUM,
    ) -> TskResult<FsBlock<'a>> {
        let ptr = unsafe {
            tsk_fs_block_get_flag(self.0, null_mut() as *mut TSK_FS_BLOCK, addr as u64, flags)
        };
        if ptr.is_null() {
            TskError::get_err()?;
        }
            Ok(FsBlock::new(ptr))
    }

    pub fn block_get(&'a self, addr: usize) -> TskResult<FsBlock<'a>> {
        let ptr = unsafe { tsk_fs_block_get(self.0, null_mut() as *mut TSK_FS_BLOCK, addr as u64) };
        if ptr.is_null() {
            TskError::get_err()?;
        }
            Ok(FsBlock::new(ptr))
    }

    pub fn dir_open<T: AsRef<str>>(&self, path: T) -> TskResult<Dir> {
        let cs = CString::new(path.as_ref())?;
        let t: Tchar = cs.as_tchar();
        if t.is_empty() {
            TskError::get_err()?;
        }
        let ptr = unsafe { tsk_fs_dir_open(self.0, *t) };

        if ptr.is_null() {
            TskError::get_err()?;
        }

        Ok(Dir::new(ptr))
    }

    pub fn dir_open_root(&self) -> TskResult<Dir> {
        let ptr = unsafe { tsk_fs_dir_open_meta(self.0, (*self.0).root_inum) };

        if ptr.is_null() {
            TskError::get_err()?;
        }

        Ok(Dir::new(ptr))
    }

    // None for . and ..
    pub fn dir_open_meta(&self, addr: u64) -> Option<Dir> {
        let f = unsafe { tsk_fs_dir_open_meta(self.0, addr as TSK_INUM_T) };

        if f.is_null() {
            return None;
        }

        Some(Dir::new(f))
    }

    // &File or File? from the doc this is not obvious
    pub fn dir_open_from_file(&self, file: &File) -> Option<Dir> {
        let addr = unsafe { (**file.metadata().unwrap()).addr };

        self.dir_open_meta(addr)
    }

    pub fn file_open<T: AsRef<str>>(&self, path: T) -> TskResult<File> {
        let cs = CString::new(path.as_ref())?;
        let t: Tchar = cs.as_tchar();
        if t.is_empty() {
            TskError::get_err()?;
        }
        let ptr = unsafe { tsk_fs_file_open(self.0, null_mut(), *t) };

        if ptr.is_null() {
            TskError::get_err()?;
        }

        Ok(File::new(ptr))
    }
    pub fn file_open_replace<T: AsRef<str>>(&self, file: File, path: T) -> TskResult<File> {
        let cs = CString::new(path.as_ref())?;
        let t: Tchar = cs.as_tchar();
        if t.is_empty() {
            TskError::get_err()?;
        }
        let ptr = unsafe { tsk_fs_file_open(self.0, *file, *t) };

        // file is replaced freaing is handled on the libside
        forget(file);

        if ptr.is_null() {
            TskError::get_err()?;
        }

        Ok(File::new(ptr))
    }

    pub fn file_open_meta(&self, addr: u64) -> TskResult<File> {
        let ptr = unsafe { tsk_fs_file_open_meta(self.0, null_mut(), addr) };
        if ptr.is_null() {
            TskError::get_err()?;
        }
        Ok(File::new(ptr))
    }

    pub fn file_open_meta_replace(&self, file: File, addr: u64) -> TskResult<File> {
        let ptr = unsafe { tsk_fs_file_open_meta(self.0, *file, addr) };

        // file is replaced freaing is handled on the libside
        forget(file);

        if ptr.is_null() {
            TskError::get_err()?;
        }

        Ok(File::new(ptr))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::img::img_info;

    // must be a invalid code fs cannot outlive img
    // #[test]
    // fn open_seq() {
    //     let fs = img_info::tests::new().fs().unwrap();
    //
    //     fs.root().unwrap();
    // }

    #[test]
    fn open() {
        let img = img_info::tests::new();
        let fs = img.fs().unwrap();

        fs.dir_open_root().unwrap();
    }
}
