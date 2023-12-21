use std::rc::Rc;

use protobuf::descriptor::source_code_info::Location;

use crate::{file::File, iter::Iter, package::Package};

#[derive(Debug, Default, Clone)]
pub struct Comments {
    loc: Rc<Location>,
}

impl Comments {
    pub fn location(&self) -> Location {
        self.loc
    }
    pub fn is_empty(&self) -> bool {
        !self.loc.has_comments()
    }
    /// Returns any comment immediately preceding the node, without any
    /// whitespace between it and the comment.
    pub fn leading(&self) -> &str {
        self.loc.leading_comments()
    }
    /// Returns any comment immediately following the entity, without any
    /// whitespace between it and the comment. If the comment would be a leading
    /// comment for another entity, it won't be considered a trailing comment.
    pub fn trailing(&self) -> &str {
        self.loc.trailing_comments()
    }
    /// Returns each comment block or line above the
    /// entity but separated by whitespace.a
    pub fn leading_detached(&self) -> std::slice::Iter<'_, String> {
        self.loc.leading_detached_comments()
    }
}
#[derive(Debug, Clone)]
pub struct PackageComments {
    files: Iter<File>,
}
impl PackageComments {
    pub fn new(package: Package) -> Self {
        Self {
            files: package.files(),
        }
    }
}
impl Iterator for PackageComments {
    type Item = (File, Comments);

    fn next(&mut self) -> Option<Self::Item> {
        self.files
            .next()
            .filter(|file| !file.package_comments().is_empty())
            .map(|file| (file.clone(), file.package_comments()))
    }
}

impl From<Location> for Comments {
    fn from(loc: Location) -> Self {
        Comments { loc }
    }
}

// #[derive(Debug, Clone)]
// pub struct CommentsIter {
//     iter: std::slice::Iter<'a, protobuf::descriptor::source_code_info::Location>,
//     phantom: PhantomData<U>,
// }

// impl CommentsIter {
//     pub fn len(&self) -> usize {
//         self.iter.len()
//     }
//     pub fn is_empty(&self) -> bool {
//         self.len() == 0
//     }
// }
// impl Iterator for CommentsIter {
//     type Item = Comments;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next().map(Into::into)
//     }
// }
// impl From<SourceCodeInfo> for CommentsIter {
//     fn from(info: SourceCodeInfo) -> Self {
//         CommentsIter {
//             iter: info.info.location.iter(),
//             phantom: PhantomData,
//         }
//     }
// }
// impl From<&SourceCodeInfo> for CommentsIter {
//     fn from(info: &SourceCodeInfo) -> Self {
//         CommentsIter {
//             iter: info.info.location.iter(),
//             phantom: PhantomData,
//         }
//     }
// }
