
use crate::bindigs::*;


#[derive(Debug)]
pub struct VsInfo {
    pub inner: *mut TSK_VS_INFO,
}

impl Drop for VsInfo {
    fn drop (&mut self) {
        unsafe {
            tsk_vs_close(self.inner);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::img_info;
    use super::VsInfo;

    pub fn new () -> VsInfo {
        let img = img_info::tests::new();
        img.vs_info().unwrap()
    }

    #[test]
    fn open () {
        // new();
    }
}
