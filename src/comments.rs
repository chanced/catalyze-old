use crate::{iter::Iter, File, Package};

#[derive(Debug)]
pub struct Comments<'a, U> {
    loc: Location<'a, U>,
}

impl<'a, U> Default for Comments<'a, U> {
    fn default() -> Self {
        Comments {
            loc: Location::default(),
        }
    }
}

impl<'a, U> Copy for Comments<'a, U> {}
impl<'a, U> Clone for Comments<'a, U> {
    fn clone(&self) -> Self {
        Comments { loc: self.loc }
    }
}
impl<'a, U> From<&Location<'a, U>> for Comments<'a, U> {
    fn from(loc: &Location<'a, U>) -> Self {
        Comments { loc: *loc }
    }
}

impl<'a, U> From<&'a prost_types::source_code_info::Location> for Comments<'a, U> {
    fn from(loc: &'a prost_types::source_code_info::Location) -> Self {
        Comments { loc: loc.into() }
    }
}

impl<'a, U> Comments<'a, U> {
    /// Returns any comment immediately preceding the node, without any
    /// whitespace between it and the comment.
    pub fn leading(&self) -> &'a str {
        self.loc.leading_comments()
    }
    pub fn location(&self) -> Location<'a, U> {
        self.loc
    }
    pub fn is_empty(&self) -> bool {
        !self.loc.has_comments()
    }
    /// Returns each comment block or line above the
    /// entity but separated by whitespace.a
    pub fn leading_detached(&self) -> std::slice::Iter<'a, String> {
        self.loc.leading_detached_comments()
    }
    /// Returns any comment immediately following the entity, without any
    /// whitespace between it and the comment. If the comment would be a leading
    /// comment for another entity, it won't be considered a trailing comment.
    pub fn trailing(&self) -> &'a str {
        self.loc.trailing_comments()
    }
}

impl<'a, U> From<Location<'a, U>> for Comments<'a, U> {
    fn from(loc: Location<'a, U>) -> Self {
        Comments { loc }
    }
}

#[derive(Debug)]
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
        self.files
            .next()
            .filter(|file| !file.package_comments().is_empty())
            .map(|file| (file.clone(), file.package_comments()))
    }
}
