use std::{marker::PhantomData, ops::Deref};
use super::fs_info::FsInfo;
use crate::bindings::*;


#[derive(Debug)]
pub struct FsBlock<'a>(*mut TSK_FS_BLOCK, PhantomData<&'a FsInfo<'a>>);

impl Deref for FsBlock<'_> {
    type Target = *mut TSK_FS_BLOCK;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for FsBlock<'_> {
    fn drop(&mut self) {
        unsafe {
            tsk_fs_block_free(self.0);
        }
    }
}

impl<'a> FsBlock<'a> {
    pub fn new(ptr: *mut TSK_FS_BLOCK) -> FsBlock<'a> {
        FsBlock(ptr, PhantomData)
    }
}
