use std::marker::PhantomData;

use crate::iter::CommentsIter;

use super::{iter::LocationIter, FileDescriptorPath};

lazy_static! {
    static ref DEFAULT_LOCATION: prost_types::source_code_info::Location =
        prost_types::source_code_info::Location::default();
    static ref DEFAULT_SOURCE_CODE_INFO: prost_types::SourceCodeInfo =
        prost_types::SourceCodeInfo::default();
}

#[derive(Debug)]
pub struct SourceCodeInfo<'a, U> {
    pub(crate) info: &'a prost_types::SourceCodeInfo,
    phantom: PhantomData<U>,
}

impl<'a, U> SourceCodeInfo<'a, U> {
    pub fn iter(&self) -> LocationIter<'a, U> {
        self.into()
    }
    pub fn len(&self) -> usize {
        self.info.location.len()
    }
    pub fn is_empty(&self) -> bool {
        self.info.location.is_empty()
    }
}

impl<'a, U> From<&'a prost_types::SourceCodeInfo> for SourceCodeInfo<'a, U> {
    fn from(info: &'a prost_types::SourceCodeInfo) -> Self {
        SourceCodeInfo {
            info,
            phantom: PhantomData,
        }
    }
}

impl<'a, U> From<Option<&'a prost_types::SourceCodeInfo>> for SourceCodeInfo<'a, U> {
    fn from(info: Option<&'a prost_types::SourceCodeInfo>) -> Self {
        SourceCodeInfo {
            info: info.unwrap_or(&DEFAULT_SOURCE_CODE_INFO),
            phantom: PhantomData,
        }
    }
}

impl<'a, U> IntoIterator for SourceCodeInfo<'a, U> {
    type Item = Location<'a, U>;
    type IntoIter = LocationIter<'a, U>;

    fn into_iter(self) -> Self::IntoIter {
        LocationIter::from(&self)
    }
}
impl<'a, U> Copy for SourceCodeInfo<'a, U> {}
impl<'a, U> Clone for SourceCodeInfo<'a, U> {
    fn clone(&self) -> Self {
        SourceCodeInfo {
            info: self.info,
            phantom: PhantomData,
        }
    }
}
impl<'a, U> SourceCodeInfo<'a, U> {
    pub fn comments(&self) -> CommentsIter<'a, U> {
        self.into()
    }
}

impl<'a, U> Default for Location<'a, U> {
    fn default() -> Self {
        Location {
            loc: &DEFAULT_LOCATION,
            u: PhantomData,
        }
    }
}

/// Comments associated to entities in the source code.
#[derive(Debug)]
pub struct Location<'a, U> {
    loc: &'a prost_types::source_code_info::Location,
    u: PhantomData<U>,
}
impl<'a, U> From<&'a prost_types::source_code_info::Location> for Location<'a, U> {
    fn from(loc: &'a prost_types::source_code_info::Location) -> Self {
        Self {
            loc,
            u: PhantomData,
        }
    }
}

impl<'a, U> Location<'a, U> {
    /// Identifies which part of the FileDescriptorProto was defined at this
    /// location.
    ///
    /// Each element is a field number or an index.  They form a path from
    /// the root FileDescriptorProto to the place where the definition.  For
    /// example, this path:
    ///   [ 4, 3, 2, 7, 1 ]
    /// refers to:
    ///   file.message_type(3)  // 4, 3
    ///       .field(7)         // 2, 7
    ///       .name()           // 1
    /// This is because FileDescriptorProto.message_type has field number 4:
    ///   repeated DescriptorProto message_type = 4;
    /// and DescriptorProto.field has field number 2:
    ///   repeated FieldDescriptorProto field = 2;
    /// and FieldDescriptorProto.name has field number 1:
    ///   optional string name = 1;
    ///
    /// Thus, the above path gives the location of a field name.  If we removed
    /// the last element:
    ///   [ 4, 3, 2, 7 ]
    /// this path refers to the whole field declaration (from the beginning
    /// of the label to the terminating semicolon).
    pub fn path(&self) -> &'a [i32] {
        &self.loc.path
    }
    /// Always has exactly three or four elements: start line, start column,
    /// end line (optional, otherwise assumed same as start line), end column.
    /// These are packed into a single field for efficiency.  Note that line
    /// and column numbers are zero-based -- typically you will want to add
    /// 1 to each before displaying to a user
    pub fn span(&self) -> &'a [i32] {
        &self.loc.span
    }

    /// Returns any comment immediately preceding the node, without anyElsewhere
    /// whitespace between it and the comment.
    pub fn leading_comments(&self) -> &'a str {
        self.loc.leading_comments()
    }

    /// Returns each comment block or line above the
    /// entity but separated by whitespace.a
    pub fn leading_detached_comments(&self) -> std::slice::Iter<'a, String> {
        self.loc.leading_detached_comments.iter()
    }
    /// Returns any comment immediately following the entity, without any
    /// whitespace between it and the comment. If the comment would be a leading
    /// comment for another entity, it won't be considered a trailing comment.
    pub fn trailing_comments(&self) -> &'a str {
        self.loc.trailing_comments()
    }

    pub fn is_file_syntax_location(&self) -> bool {
        self.path().len() == 1 && FileDescriptorPath::Syntax == self.path()[0]
    }

    pub fn is_file_package_location(&self) -> bool {
        self.path().len() == 1 && FileDescriptorPath::Package == self.path()[0]
    }

    pub fn file_descriptor_path(&self) -> Result<FileDescriptorPath, anyhow::Error> {
        FileDescriptorPath::try_from(self.path().get(0))
    }

    pub fn has_comments(&self) -> bool {
        !self.leading_comments().is_empty()
            || self.leading_detached_comments().count() > 0
            || !self.trailing_comments().is_empty()
    }
}
impl<'a, U> Copy for Location<'a, U> {}
impl<'a, U> Clone for Location<'a, U> {
    fn clone(&self) -> Self {
        Location {
            loc: self.loc,
            u: PhantomData {},
        }
    }
}
