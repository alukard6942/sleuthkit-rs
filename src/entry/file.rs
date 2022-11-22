use super::{dir::Dir, DirWrapper};
use crate::{bindings::*, fs_info::FsWrapper};
use std::{ffi::CStr, fmt::Display, rc::Rc};

#[derive(Debug)]
pub struct File {
    pub inner: *mut TSK_FS_FILE,
    pub parent: Rc<FsWrapper>,
}

impl File {
    pub fn name(&self) -> Result<&str, std::str::Utf8Error> {
        let s = unsafe { CStr::from_ptr((*(*self.inner).name).name) };

        s.to_str()
    }

    pub fn is_file(&self) -> bool {
        let meta = unsafe {
            // the field name is type but that happens to be reserved by rust
            (*(*self.inner).meta).type_
        };

        // lol no cast from u32 to bool pathetic
        (meta & TSK_FS_META_TYPE_ENUM_TSK_FS_META_TYPE_DIR) == 0
    }

    /*
     ** Is this string a "." or ".."
     */
    pub fn is_dot(&self) -> bool {
        unsafe {
            let ptr = (*(*self.inner).name).name;
            if ptr.is_null() {
                return false;
            }

            // viz macro
            (*ptr == '.' as i8)
                && (((*ptr.add(1) == '.' as i8) && (*ptr.add(2) == '\0' as i8))
                    || (*ptr.add(1) == '\0' as i8))
        }
    }

    pub fn is_dir(&self) -> bool {
        !self.is_file()
    }

    pub fn is_subdir(&self) -> bool {
        self.is_dir() && !self.is_dot()
    }

    pub fn to_subdir(&self) -> Option<Dir> {
        if !self.is_subdir() {
            return None;
        }

        let faddr = unsafe { (*(*self.inner).meta).addr };

        let f = unsafe { tsk_fs_dir_open_meta(self.parent.inner, faddr) };

        if f.is_null() {
            return None;
        }

        Some(Dir {
            inner: Rc::new(DirWrapper {
                inner: f,
                parent: Rc::clone(&self.parent),
            }),
        })
    }

    pub fn to_dir(mut self) -> Option<Dir> {
        if self.is_file() {
            return None;
        }

        let faddr = unsafe { (*(*self.inner).meta).addr };

        let f = unsafe { tsk_fs_dir_open_meta(self.parent.inner, faddr) };

        if f.is_null() {
            return None;
        }

        self.inner = 0 as *mut TSK_FS_FILE;

        Some(Dir {
            inner: Rc::new(DirWrapper {
                inner: f,
                parent: Rc::clone(&self.parent),
            }),
        })
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = match self.name() {
            Ok(it) => it,
            Err(_) => return Err(std::fmt::Error),
        };

        write!(f, "{}", m)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            if !self.inner.is_null() {
                // println!("droping file");
                tsk_fs_file_close(self.inner);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{entry::Dir, img_info};

    #[test]
    fn todir() {
        let root = {
            let img = img_info::tests::new();
            let fs = img.fs_info().unwrap();
            let root = fs.root().unwrap();
            root
        };

        println!("no droping jet");

        for f in &root {
            println!("{:?}", f.name());
            if let Some(d) = f.to_dir() {
                println!("{:?}", d.name());
            }
        }
    }
}
