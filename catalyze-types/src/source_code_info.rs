use super::FileDescriptorPath;
// use super::Impl;
pub trait SourceCodeInfo<'a>:
    IntoIterator<Item = Self::Impl::Location, IntoIter = Self::Impl::LocationIter>
{
    type Impl: crate::Impl<'a>;
    fn iter(&self) -> Self::Impl::LocationIter {
        self.into_iter()
    }
    fn len(&self) -> usize {
        self.into_iter().len()
    }
    fn is_empty(&self) -> bool {
        self.into_iter().len() == 0
    }
}

/// Comments associated to entities in the source code.
pub trait Location<'a> {
    type Impl: crate::Impl<'a>;
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
    fn path(&self) -> &'a [i32];

    /// Always has exactly three or four elements: start line, start column,
    /// end line (optional, otherwise assumed same as start line), end column.
    /// These are packed into a single field for efficiency.  Note that line
    /// and column numbers are zero-based -- typically you will want to add
    /// 1 to each before displaying to a user
    fn span(&self) -> &'a [i32];

    /// Returns any comment immediately preceding the node, without anyElsewhere
    /// whitespace between it and the comment.
    fn leading_comments(&self) -> &'a str;

    /// Returns each comment block or line above the
    /// entity but separated by whitespace.a
    fn leading_detached_comments(&self) -> std::slice::Iter<'a, String>;
    /// Returns any comment immediately following the entity, without any
    /// whitespace between it and the comment. If the comment would be a leading
    /// comment for another entity, it won't be considered a trailing comment.
    fn trailing_comments(&self) -> &'a str;

    fn is_file_syntax_location(&self) -> bool {
        self.path().len() == 1 && FileDescriptorPath::Syntax == self.path()[0]
    }

    fn is_file_package_location(&self) -> bool {
        self.path().len() == 1 && FileDescriptorPath::Package == self.path()[0]
    }

    fn file_descriptor_path(&self) -> Result<FileDescriptorPath, anyhow::Error> {
        FileDescriptorPath::try_from(self.path().get(0))
    }

    fn has_comments(&self) -> bool {
        !self.leading_comments().is_empty()
            || self.leading_detached_comments().count() > 0
            || !self.trailing_comments().is_empty()
    }
}
