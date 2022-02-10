#[derive(Clone, Debug, PartialEq)]
pub struct SourceCodeInfo {
    /// A Location identifies a piece of source code in a .proto file which
    /// corresponds to a particular definition.  This information is intended
    /// to be useful to IDEs, code indexers, documentation generators, and similar
    /// tools.
    ///
    /// For example, say we have a file like:
    ///   message Foo {
    ///     optional string foo = 1;
    ///   }
    /// Let's look at just the field definition:
    ///   optional string foo = 1;
    ///   ^       ^^     ^^  ^  ^^^
    ///   a       bc     de  f  ghi
    /// We have the following locations:
    ///   span   path               represents
    ///   [a,i)  [ 4, 0, 2, 0 ]     The whole field definition.
    ///   [a,b)  [ 4, 0, 2, 0, 4 ]  The label (optional).
    ///   [c,d)  [ 4, 0, 2, 0, 5 ]  The type (string).
    ///   [e,f)  [ 4, 0, 2, 0, 1 ]  The name (foo).
    ///   [g,h)  [ 4, 0, 2, 0, 3 ]  The number (1).
    ///
    /// Notes:
    /// - A location may refer to a repeated field itself (i.e. not to any
    ///   particular index within it).  This is used whenever a set of elements are
    ///   logically enclosed in a single code segment.  For example, an entire
    ///   extend block (possibly containing multiple extension definitions) will
    ///   have an outer location whose path refers to the "extensions" repeated
    ///   field without an index.
    /// - Multiple locations may have the same path.  This happens when a single
    ///   logical declaration is spread out across multiple places.  The most
    ///   obvious example is the "extend" block again -- there may be multiple
    ///   extend blocks in the same scope, each of which will have the same path.
    /// - A location's span is not always a subset of its parent's span.  For
    ///   example, the "extendee" of an extension declaration appears at the
    ///   beginning of the "extend" block and is shared by all extensions within
    ///   the block.
    /// - Just because a location's span is a subset of some other location's span
    ///   does not mean that it is a descendant.  For example, a "group" defines
    ///   both a type and a field in a single declaration.  Thus, the locations
    ///   corresponding to the type and field and their components will overlap.
    /// - Code which tries to interpret locations should probably be designed to
    ///   ignore those that it doesn't understand, as more types of locations could
    ///   be recorded in the future.
    location: Vec<Location>,
}

impl SourceCodeInfo {
    pub(crate) fn new(sci: prost_types::SourceCodeInfo) -> Self {
        Self {
            location: sci.location.iter().map(Location::new).collect(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Location {
    // tag 1
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
    path: Vec<i32>,
    /// tag 2
    span: Vec<i32>,
    leading_comments: Option<String>,
    trailing_comments: Option<String>,
    leading_detached_comments: Vec<String>,
}

impl Location {
    fn new(loc: &prost_types::source_code_info::Location) -> Self {
        Location {
            leading_comments: loc.leading_comments,
            trailing_comments: loc.trailing_comments,
            leading_detached_comments: loc.leading_detached_comments,
            path: loc.path,
            span: loc.span,
        }
    }
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
    pub fn path(&self) -> Vec<i32> {
        self.path.clone()
    }
    /// If this SourceCodeInfo represents a complete declaration, these are any
    /// comments appearing before and after the declaration which appear to be
    /// attached to the declaration.
    ///
    /// A series of line comments appearing on consecutive lines, with no other
    /// tokens appearing on those lines, will be treated as a single comment.
    ///
    /// leading_detached_comments will keep paragraphs of comments that appear
    /// before (but not connected to) the current element. Each paragraph,
    /// separated by empty lines, will be one comment element in the repeated
    /// field.
    ///
    /// Only the comment content is provided; comment markers (e.g. //) are
    /// stripped out.  For block comments, leading whitespace and an asterisk
    /// will be stripped from the beginning of each line other than the first.
    /// Newlines are included in the output.
    ///
    /// Examples:
    ///
    ///   optional int32 foo = 1;  // Comment attached to foo.
    ///   // Comment attached to bar.
    ///   optional int32 bar = 2;
    ///
    ///   optional string baz = 3;
    ///   // Comment attached to baz.
    ///   // Another line attached to baz.
    ///
    ///   // Comment attached to qux.
    ///   //
    ///   // Another line attached to qux.
    ///   optional double qux = 4;
    ///
    ///   // Detached comment for corge. This is not leading or trailing comments
    ///   // to qux or corge because there are blank lines separating it from
    ///   // both.
    ///
    ///   // Detached comment for corge paragraph 2.
    ///
    ///   optional string corge = 5;
    ///   /* Block comment attached
    ///    * to corge.  Leading asterisks
    ///    * will be removed. */
    ///   /* Block comment attached to
    ///    * grault. */
    ///   optional int32 grault = 6;
    ///
    ///   // ignored detached comments.
    pub fn leading_comments(&self) -> Option<String> {
        self.leading_comments.clone()
    }
    pub fn trailing_comments(&self) -> Option<String> {
        self.trailing_comments.clone()
    }
    /// leading_detached_comments will keep paragraphs of comments that appear
    /// before (but not connected to) the current element. Each paragraph,
    /// separated by empty lines, will be one comment element in the repeated
    /// field.
    pub fn leading_detached_comments(&self) -> Vec<String> {
        self.leading_detached_comments.clone()
    }
}
