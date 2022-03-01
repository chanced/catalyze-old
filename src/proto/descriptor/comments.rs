use std::slice::Iter;

use petgraph::visit::Walker;

use crate::util::Util;

/// Comments associated to entities in the source code.
#[derive(Debug)]
pub struct Comments<'a, U> {
    loc: &'a prost_types::source_code_info::Location,
    util: Util<U>,
}

impl<'a, U> Comments<'a, U> {
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

    /// Returns any comment immediately preceding the node, without any
    /// whitespace between it and the comment.
    pub fn leading_comments(&self) -> &'a str {
        self.loc.leading_comments()
    }

    /// Returns each comment block or line above the
    /// entity but separated by whitespace.a
    pub fn leading_detached_comments(&self) -> Iter<'a, String> {
        self.loc.leading_detached_comments.iter()
    }
    /// Returns any comment immediately following the entity, without any
    /// whitespace between it and the comment. If the comment would be a leading
    /// comment for another entity, it won't be considered a trailing comment.
    pub fn trailing_comments(&self) -> &'a str {
        self.loc.trailing_comments()
    }
}

impl<'a, U> Clone for Comments<'a, U> {
    fn clone(&self) -> Self {
        Comments {
            loc: self.loc,
            util: self.util.clone(),
        }
    }
}
