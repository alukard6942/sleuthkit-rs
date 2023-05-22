use std::marker::PhantomData;
use std::ops::Deref;
use crate::img::img_info::ImgInfo;
use crate::{bindings::*, error::TskResult};


pub struct VsPart<'a>(*const TSK_VS_PART_INFO, PhantomData<&'a ImgInfo>);

impl Deref for VsPart<'_> {
    type Target = *const TSK_VS_PART_INFO;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> VsPart<'a> {
    pub(crate) fn new(ptr: *const TSK_VS_PART_INFO) -> VsPart<'a> {
        VsPart(ptr, PhantomData)
    }

    pub fn read_at(&self, offset: usize, buf: &mut [u8]) -> TskResult<usize> {
        let s = unsafe {
            tsk_vs_part_read(
                self.0,
                offset as i64,
                buf.as_mut_ptr() as *mut i8,
                buf.len(),
            )
        };

        if s < 0 {
            Err(crate::error::TskError::Str("vspart read_at"))
        } else {
            Ok(s as usize)
        }
    }

    pub fn read_block(&self, addr: TSK_DADDR_T, buf: &mut [u8]) -> TskResult<usize> {
        let s =
            unsafe { tsk_vs_part_read_block(self.0, addr, buf.as_mut_ptr() as *mut i8, buf.len()) };

        if s < 0 {
            Err(crate::error::TskError::Str("vspart read_at"))
        } else {
            Ok(s as usize)
        }
    }
}
