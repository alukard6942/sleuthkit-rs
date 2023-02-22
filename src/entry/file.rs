use super::{dir::Dir, DirWrapper};
use crate::{bindings::*, error::TskResult, fs_info::FsWrapper};
use std::{ffi::CStr, fmt::Display, rc::Rc, usize};

#[derive(Debug)]
pub struct FileWrapper {
    pub inner: *mut TSK_FS_FILE,
}

#[derive(Debug)]
pub struct File {
    pub inner: Rc<FileWrapper>,
    pub parent: Rc<FsWrapper>,
}

impl File {
    pub fn contents(&self) -> Vec<u8> {
        // read file kilobyte by kilobyte
        // todo: optimalizacion this seams wrong
        let step = 1024;
        let mut size = 0;
        let mut buffer = Vec::with_capacity(step as usize);

        loop {
            let read = unsafe {
                tsk_fs_file_read(
                    self.inner.inner,
                    size as i64,
                    (buffer.as_mut_ptr() as *mut i8).add(size as usize),
                    step as usize,
                    TSK_FS_FILE_READ_FLAG_ENUM_TSK_FS_FILE_READ_FLAG_NONE,
                )
            };

            if read == -1 {
                return Vec::new();
            }

            size += read;

            // basecase
            if read < step {
                unsafe {
                    buffer.set_len(size as usize);
                }
                break;
            }

            buffer.reserve(step as usize);
        }

        buffer
    }

    // todo: does this makes actualy sence? can this ever return none?
    pub fn bytes(&self, buffer: &mut Vec<u8>) -> Option<usize> {
        let size = unsafe {
            tsk_fs_file_read(
                self.inner.inner,
                0,
                buffer.as_mut_ptr() as *mut i8,
                buffer.capacity(),
                TSK_FS_FILE_READ_FLAG_ENUM_TSK_FS_FILE_READ_FLAG_NONE,
            )
        };
        if size < 0 {
            return None;
        }

        unsafe {
            buffer.set_len(size as usize);
        }

        Some(size as usize)
    }

    pub fn name(&self) -> TskResult<&str> {
        let s = unsafe {
            CStr::from_ptr({
                let inner = self.inner.inner;
                if inner.is_null() {
                    Err("fs_file")?
                }
                let name = (*inner).name;
                if name.is_null() {
                    Err("name is null")?
                }
                (*name).name
            })
        };

        Ok(s.to_str()?)
    }

    pub fn is_file(&self) -> bool {
        let meta = unsafe {
            // the field name is type but that happens to be reserved by rust
            (*(*self.inner.inner).meta).type_
        };

        // lol no cast from u32 to bool pathetic
        (meta & TSK_FS_META_TYPE_ENUM_TSK_FS_META_TYPE_DIR) == 0
    }

    /*
     ** Is this string a "." or ".."
     */
    pub fn is_dot(&self) -> bool {
        unsafe {
            let ptr = (*(*self.inner.inner).name).name;
            if ptr.is_null() {
                return false;
            }

            // viz macro from tsk
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

    // None for . and ..
    pub fn to_subdir(&self) -> Option<Dir> {
        if !self.is_subdir() {
            return None;
        }

        let faddr = unsafe { (*(*self.inner.inner).meta).addr };

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

    pub fn to_dir(&self) -> Option<Dir> {
        if !self.is_dir() {
            return None;
        }

        let faddr = unsafe { (*(*self.inner.inner).meta).addr };

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

impl Drop for FileWrapper {
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
    use std::string;

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

    #[test]
    pub fn bytestest() {
        let r = img_info::ImgInfo::new("testData/ntfs.img")
            .unwrap()
            .fs_info()
            .unwrap()
            .root()
            .unwrap();

        let file = {
            let mut file = None;
            for f in &r {
                if f.name().unwrap().contains("pdf") {
                    file = Some(f);
                    break;
                }
            }

            file.unwrap()
        };

        println!("file {:?}", file.name());

        let mut buffer = Vec::with_capacity(1024);
        let size = file.bytes(&mut buffer);
        println!("size {:?}", size);

        let pdfsig = ['%' as u8, 'P' as u8, 'D' as u8, 'F' as u8];
        let filesig = buffer.get(0..4).unwrap();

        println!("signacure {:?}", filesig);
        println!("pdfsingcr {:?}", pdfsig);

        assert_eq!(filesig, pdfsig);
    }
}
