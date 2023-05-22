use super::fs_info::FsInfo;
use crate::bindings::*;
use crate::error::TskResult;
use std::ffi::c_uchar;
use std::marker::PhantomData;

pub struct FsAttr<'a>(*mut TSK_FS_ATTR, PhantomData<&'a FsInfo<'a>>);

impl<'a> FsAttr<'a> {
    pub(crate) fn new(ptr: *mut TSK_FS_ATTR) -> FsAttr<'a> {
        FsAttr(ptr, PhantomData)
    }

    pub fn attr_read_at(
        &self,
        offset: usize,
        buf: &mut [u8],
        flags: TSK_FS_FILE_READ_FLAG_ENUM,
    ) -> TskResult<usize> {
        let size = unsafe {
            tsk_fs_attr_read(
                self.0,
                offset as i64,
                buf.as_mut_ptr() as *mut i8,
                buf.len(),
                flags,
            )
        };

        if size < 0 {
            Err("attr_read_at")?
        } else {
            Ok(size as usize)
        }
    }
}
