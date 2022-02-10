/// `SourceContext` represents information about the source of a
/// protobuf element, like the file in which it is defined.
pub struct SourceContext {
    /// The path-qualified name of the .proto file that contained the associated
    /// protobuf element.  For example: `"google/protobuf/source_context.proto"`.
    file_name: String,
}

impl SourceContext {
    pub(crate) fn new(sc: &prost_types::SourceContext) -> Self {
        SourceContext {
            file_name: sc.file_name.clone(),
        }
    }
    /// The path-qualified name of the .proto file that contained the associated
    /// protobuf element.  For example: `"google/protobuf/source_context.proto"`.
    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }
}
