use std::{mem::MaybeUninit, ops::Deref};

use crate::bindings::{TSK_FS_META, tsk_fs_meta_make_ls};

pub struct MetaData(pub(crate) *const TSK_FS_META);

impl Deref for MetaData {
    type Target = *const TSK_FS_META;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MetaData {
    /* unix style rights string
     *  example: '-rw-rw-r--' */
    pub fn make_ls(&self) -> String {
        let mut b = MaybeUninit::<[u8;12]>::uninit();
        unsafe {
            tsk_fs_meta_make_ls(self.0, b.as_mut_ptr() as *mut i8, 12);
            String::from_raw_parts(b.assume_init().as_mut_ptr(), 12, 12)
        }
    }
}
