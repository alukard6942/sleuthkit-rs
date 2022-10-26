/**
 * File: dir.rs
 * Author: alukard <alukard6942@github>
 * Date: 23.10.2022
 * Last Modified Date: 23.10.2022
 */

use crate::bindigs::*;


#[derive(Debug)]
pub struct Dir {
    pub inner: *mut TSK_FS_DIR,
}


impl Drop for Dir {
    fn drop(&mut self) {
        unsafe {
            tsk_fs_dir_close(self.inner)
        }
    }
}
