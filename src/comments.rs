use crate::{iter::Iter, proto::Location, File, Package};

#[derive(Debug, Default)]
pub struct Comments<'a> {
    loc: Location<'a>,
}

impl<'a> Copy for Comments<'a> {}
impl<'a> Clone for Comments<'a> {
    fn clone(&self) -> Self {
        Comments { loc: self.loc }
    }
}
impl<'a> From<&Location<'a>> for Comments<'a> {
    fn from(loc: &Location<'a>) -> Self {
        Comments { loc: *loc }
    }
}

impl<'a> From<&'a prost_types::source_code_info::Location> for Comments<'a> {
    fn from(loc: &'a prost_types::source_code_info::Location) -> Self {
        Comments { loc: loc.into() }
    }
}

impl<'a> Comments<'a> {
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

impl<'a> From<Location<'a>> for Comments<'a> {
    fn from(loc: Location<'a>) -> Self {
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
    type Item = (File<'a, U>, Comments<'a>);

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

// impl<'a> CommentsIter<'a, U> {
//     pub fn len(&self) -> usize {
//         self.iter.len()
//     }
//     pub fn is_empty(&self) -> bool {
//         self.len() == 0
//     }
// }
// impl<'a> Iterator for CommentsIter<'a, U> {
//     type Item = Comments<'a>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next().map(Into::into)
//     }
// }
// impl<'a> From<SourceCodeInfo<'a, U>> for CommentsIter<'a, U> {
//     fn from(info: SourceCodeInfo<'a, U>) -> Self {
//         CommentsIter {
//             iter: info.info.location.iter(),
//             phantom: PhantomData,
//         }
//     }
// }
// impl<'a> From<&SourceCodeInfo<'a, U>> for CommentsIter<'a, U> {
//     fn from(info: &SourceCodeInfo<'a, U>) -> Self {
//         CommentsIter {
//             iter: info.info.location.iter(),
//             phantom: PhantomData,
//         }
//     }
// }
