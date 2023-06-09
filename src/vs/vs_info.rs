use core::marker::PhantomData;
use std::ops::Deref;
use std::os::unix::prelude::{AsFd, AsRawFd};

use libc::c_char;

use crate::error::TskError;
use crate::{bindings::*, error::TskResult};

use super::vs_part::VsPart;

#[derive(Debug)]
pub struct VsInfo<'a>(*mut TSK_VS_INFO, PhantomData<&'a ()>);

impl<'a> VsInfo<'a> {
    pub(crate) fn new(ptr: *mut TSK_VS_INFO) -> VsInfo<'a> {
        VsInfo(ptr, PhantomData)
    }
}

impl Drop for VsInfo<'_> {
    fn drop(&mut self) {
        unsafe {
            tsk_vs_close(self.0);
        }
    }
}

impl Deref for VsInfo<'_> {
    type Target = *mut TSK_VS_INFO;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> VsInfo<'a> {
    pub fn part_get(&self, idx: TSK_PNUM_T) -> VsPart<'a> {
        let ptr = unsafe { tsk_vs_part_get(self.0, idx) };

        VsPart::new(ptr)
    }

    pub fn read_block(&self, addr: TSK_DADDR_T, buf: &mut [u8]) -> TskResult<usize> {
        let s =
            unsafe { tsk_vs_read_block(self.0, addr, buf.as_mut_ptr() as *mut i8, buf.len()) };

        if s < 0 {
            TskError::get_err()?;
        } 
            Ok(s as usize)
    }

    pub fn print<T: AsFd>(f: T) {
        let fd = f.as_fd().as_raw_fd();
        let mode = "w";

        unsafe {
            let file = libc::fdopen(fd, mode.as_ptr() as *const c_char);
            tsk_vs_type_print(file as *mut _IO_FILE);
            libc::fflush(file);
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::img::img_info;

    #[test]
    #[should_panic]
    pub fn new() {
        let img = img_info::tests::new();
        let _vs = img.vs().unwrap();
    }
}
