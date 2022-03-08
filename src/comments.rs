use std::marker::PhantomData;

use crate::{iter::Iter, proto::Location, File, Package};

#[derive(Debug)]
pub struct Comments<'a, U> {
    u: PhantomData<U>,
    loc: Location<'a>,
}

impl<'a, U> Default for Comments<'a, U> {
    fn default() -> Self {
        Comments {
            loc: Location::default(),
            u: PhantomData,
        }
    }
}

impl<'a, U> Copy for Comments<'a, U> {}
impl<'a, U> Clone for Comments<'a, U> {
    fn clone(&self) -> Self {
        Comments {
            loc: self.loc,
            u: PhantomData,
        }
    }
}
impl<'a, U> From<&Location<'a>> for Comments<'a, U> {
    fn from(loc: &Location<'a>) -> Self {
        Comments {
            loc: *loc,
            u: PhantomData,
        }
    }
}

impl<'a, U> From<&'a prost_types::source_code_info::Location> for Comments<'a, U> {
    fn from(loc: &'a prost_types::source_code_info::Location) -> Self {
        Comments {
            loc: loc.into(),
            u: PhantomData,
        }
    }
}

impl<'a, U> Comments<'a, U> {
    /// Returns any comment immediately preceding the node, without any
    /// whitespace between it and the comment.
    pub fn leading(&self) -> &'a str {
        self.loc.leading_comments()
    }
    pub fn location(&self) -> Location<'a> {
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

impl<'a, U> From<Location<'a>> for Comments<'a, U> {
    fn from(loc: Location<'a>) -> Self {
        Comments {
            loc,
            u: PhantomData,
        }
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

// #[derive(Debug)]
// pub struct CommentsIter<'a, U> {
//     iter: std::slice::Iter<'a, prost_types::source_code_info::Location>,
//     phantom: PhantomData<U>,
// }

// impl<'a, U> CommentsIter<'a, U> {
//     pub fn len(&self) -> usize {
//         self.iter.len()
//     }
//     pub fn is_empty(&self) -> bool {
//         self.len() == 0
//     }
// }
// impl<'a, U> Iterator for CommentsIter<'a, U> {
//     type Item = Comments<'a, U>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next().map(Into::into)
//     }
// }
// impl<'a, U> From<SourceCodeInfo<'a, U>> for CommentsIter<'a, U> {
//     fn from(info: SourceCodeInfo<'a, U>) -> Self {
//         CommentsIter {
//             iter: info.info.location.iter(),
//             phantom: PhantomData,
//         }
//     }
// }
// impl<'a, U> From<&SourceCodeInfo<'a, U>> for CommentsIter<'a, U> {
//     fn from(info: &SourceCodeInfo<'a, U>) -> Self {
//         CommentsIter {
//             iter: info.info.location.iter(),
//             phantom: PhantomData,
//         }
//     }
// }
