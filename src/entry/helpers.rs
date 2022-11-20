use super::dir;
use super::file;

#[derive(Debug)]
pub struct DirIter<'a> {
    pub count: usize,
    pub parent: &'a dir::Dir,
}

impl Iterator for DirIter<'_> {
    type Item = file::File;
    fn next(&mut self) -> Option<Self::Item> {
        let e = self.parent.nth(self.count);
        self.count += 1;
        e
    }
}
