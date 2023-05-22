use super::file;
use super::Dir;

#[derive(Debug)]
pub struct DirIter<'a> {
    pub count: usize,
    pub parent: &'a Dir<'a>,
}

impl Iterator for DirIter<'_> {
    type Item = file::File;
    fn next(&mut self) -> Option<Self::Item> {
        let e = self.parent.nth(self.count);
        self.count += 1;
        e
    }
}

impl<'a> IntoIterator for &'a Dir<'_> {
    type Item = file::File;

    type IntoIter = DirIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
