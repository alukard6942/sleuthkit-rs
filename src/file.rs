/**
 * File: file.rs
 * Author: alukard <alukard6942@github>
 * Date: 25.10.2022
 * Last Modified Date: 25.10.2022
 */

use crate::bindigs::*;


struct File {
    pub inner: *mut TSK_FS_FILE,
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            tsk_fs_file_close(self.inner);
        }
    }
}
