use crate::{iter::Iter, File, Package};

use super::Comments;

pub struct PackageComments<'a, U> {
    files: Iter<File<'a, U>>,
}
impl<'a, U> PackageComments<'a, U> {
    pub fn new(package: Package<'a, U>) -> Self {
        Self {
            files: package.files(),
        }
    }
}
impl<'a, U> Iterator for PackageComments<'a, U> {
    type Item = (File<'a, U>, Comments<'a, U>);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(file) = self.files.next() {
            if let Some(comments) = file.package_comments() {
                return Some((file, comments));
            }
        }
        None
    }
}
