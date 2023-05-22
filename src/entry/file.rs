use super::dir::Dir;
use crate::{
    bindings::*,
    error::{TskError, TskResult},
    fs::fs_info::FsInfo,
};
use std::{
    ffi::CStr,
    fmt::Display,
    io::{Error, ErrorKind},
    marker::PhantomData,
    ops::Deref,
    usize,
};

#[derive(Default)]
pub struct MetaTime {
    pub crate_time: u64,
    pub last_modified_time: u64,
    pub last_acces_time: u64,
}

#[derive(Debug, Clone)]
pub struct File<'a>(*mut TSK_FS_FILE, PhantomData<&'a FsInfo<'a>>);

impl Drop for File<'_> {
    fn drop(&mut self) {
        unsafe {
            tsk_fs_file_close(self.0);
        }
    }
}

impl Deref for File<'_> {
    type Target = *mut TSK_FS_FILE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> File<'a> {
    pub(crate) fn new(ptr: *mut TSK_FS_FILE) -> File<'a> {
        File(ptr, PhantomData)
    }

    pub fn read_at(
        &self,
        offset: usize,
        buf: &mut [u8],
        flag: TSK_FS_FILE_READ_FLAG_ENUM,
    ) -> std::io::Result<usize> {
        let size = unsafe {
            let size = tsk_fs_file_read(
                self.0,
                offset as i64,
                buf.as_mut_ptr() as *mut i8,
                buf.len(),
                flag,
            ) as i64;

            if size < 0 {
                return Err(Error::new(ErrorKind::InvalidInput, "todo: better errors"));
            }
            size
        };

        Ok(size as usize)
    }

    pub fn read_type(
        &self,
        atype: TSK_FS_ATTR_TYPE_ENUM,
        id: u16,
        offset: usize,
        buf: &[u8],
        flag: TSK_FS_FILE_READ_FLAG_ENUM,
    ) -> TskResult<usize> {
        let size = unsafe {
            tsk_fs_file_read_type(
                self.0,
                atype,
                id,
                offset as i64,
                buf.as_mut_ptr() as *mut i8,
                buf.len(),
                flag,
            )
        };

        if size < 0 {
            Err("file type")?
        }
        Ok(size as usize)
    }

    pub(crate) fn metadata(&self) -> TskResult<*const TSK_FS_META> {
        Ok(unsafe {
            // the field name is type but that happens to be reserved by rust
            let meta = (*self.0).meta;
            if meta.is_null() {
                return Err(TskError::Nullptr(crate::error::Nullptr::Meta));
            }
            meta
        })
    }

    /* unix style rights string
     *  example: '-rw-rw-r--'
     * if cant construct will return: '----------'
     */
    pub fn rights(&self) -> String {
        let meta = match self.metadata() {
            Ok(it) => it,
            Err(_err) => return "-".repeat(12),
        };
        let mut b = Vec::with_capacity(12);
        unsafe {
            tsk_fs_meta_make_ls(meta, b.as_mut_ptr() as *mut i8, 12);
            b.set_len(12);
            String::from_utf8_unchecked(b)
        }
    }

    // todo: more stuff
    pub fn meta_time(&self) -> TskResult<MetaTime> {
        let meta = self.metadata()?;
        unsafe {
            Ok(MetaTime {
                crate_time: (*meta).atime as u64,
                last_modified_time: (*meta).ctime as u64,
                last_acces_time: (*meta).crtime as u64,
            })
        }
    }

    pub fn size(&self) -> usize {
        let meta = match self.metadata() {
            Ok(it) => it,
            Err(_err) => return 0,
        };
        let len = unsafe { (*meta).size } as usize;

        len
    }

    pub fn name(&self) -> TskResult<&str> {
        let s = unsafe {
            CStr::from_ptr({
                let name = (*self.0).name;
                if name.is_null() {
                    Err("name is null")?
                }
                (*name).name
            })
        };

        Ok(s.to_str()?)
    }

    pub fn is_file(&self) -> bool {
        let meta = match self.metadata() {
            Ok(it) => it,
            Err(_err) => return true,
        };
        let typ = unsafe { (*meta).type_ };

        // lol no cast from u32 to bool pathetic
        (typ & TSK_FS_META_TYPE_ENUM_TSK_FS_META_TYPE_DIR) == 0
    }

    /*
     ** Is this string a "." or ".."
     */
    pub fn is_dot(&self) -> bool {
        unsafe {
            let ptr = (*(*self.0).name).name;
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
}

impl Display for File<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = match self.name() {
            Ok(it) => it,
            Err(_) => return Err(std::fmt::Error),
        };

        write!(f, "{}", m)
    }
}

#[cfg(test)]
mod tests {
    use crate::{img::img_info::{self, ImgInfo}, bindings::TSK_FS_FILE_READ_FLAG_ENUM_TSK_FS_FILE_READ_FLAG_NONE};

    #[test]
    fn todir() {
        let img = img_info::tests::new();
        let fs = img.fs().unwrap();
        let root = fs.dir_open_root().unwrap();

        for f in &root {
            println!("{:?}", f.name());
            if let Some(d) = fs.dir_open_from_file(&f) {
                println!("{:?}", d.name());
            }
        }
    }

    #[test]
    fn contents_test() {
        let img = ImgInfo::new("testData/ntfs.img".to_string()).unwrap();
        let fs = img.fs().unwrap();
        let r = fs.dir_open_root().unwrap();

        for f in &r {
            // let c = f.contents();
            // println!("{}", String::from_utf8(c).unwrap());
            println!("{:?}", ());
        }
    }

    #[test]
    pub fn bytestest() {
        let img = ImgInfo::new("testData/ntfs.img".to_string()).unwrap();
        let fs = img.fs().unwrap();
        let r = fs.dir_open_root().unwrap();

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
        let size = file.read_at(0, &mut buffer, TSK_FS_FILE_READ_FLAG_ENUM_TSK_FS_FILE_READ_FLAG_NONE);
        println!("size {:?}", size);

        let pdfsig = ['%' as u8, 'P' as u8, 'D' as u8, 'F' as u8];
        let filesig = buffer.get(0..4).unwrap();

        println!("signacure {:?}", filesig);
        println!("pdfsingcr {:?}", pdfsig);

        assert_eq!(filesig, pdfsig);
    }
}
