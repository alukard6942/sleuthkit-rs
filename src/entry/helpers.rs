
use super::dir;
use super::file;

#[derive(Debug)]
pub struct DirIter<'a> {
    pub count: usize,
    pub parent: &'a dir::Dir,
}

#[derive(Debug)]
pub enum DirEntry {
    Dir(dir::Dir),
    File(file::File),
}

impl Iterator for DirIter<'_> {
    type Item = DirEntry;
    fn next(&mut self) -> Option<Self::Item> {
        let e = self.parent.nth(self.count);
        self.count += 1;
        e
    }
}
