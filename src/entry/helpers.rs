use super::file;
use super::Dir;

#[derive(Debug)]
pub struct DirIter {
    pub count: usize,
    pub parent: Dir,
}

impl Iterator for DirIter {
    type Item = file::File;
    fn next(&mut self) -> Option<Self::Item> {
        let e = self.parent.nth(self.count);
        self.count += 1;
        e
    }
}

impl IntoIterator for &Dir {
    type Item = file::File;

    type IntoIter = DirIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
