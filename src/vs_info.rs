use std::rc::Rc;

use crate::bindings::*;

#[derive(Debug)]
pub struct VsWrapper {
    pub inner: *mut TSK_VS_INFO,
}

#[derive(Debug)]
pub struct VsInfo {
    pub inner: Rc<VsWrapper>,
}

impl Drop for VsInfo {
    fn drop(&mut self) {
        unsafe {
            tsk_vs_close(self.inner.inner);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::img_info;

    #[test]
    #[should_panic]
    pub fn new() {
        let img = img_info::tests::new();
        let _vs = img.vs_info().unwrap();
    }
}
