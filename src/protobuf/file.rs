pub struct FileDescriptorSet {
    pub files: Vec<FileDescriptor>,
}
/// Describes a complete .proto file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDescriptorProto {
    /// file name, relative to root of source tree
    name: Option<String>,
    /// e.g. "foo", "foo.bar", etc.
    package: Option<String>,
    /// Names of files imported by this file.
    dependency: Vec<String>,
    /// Indexes of the public imported files in the dependency list above.
    public_dependency: Vec<i32>,
    /// Indexes of the weak imported files in the dependency list.
    /// For Google-internal migration only. Do not use.
    weak_dependency: Vec<i32>,
    /// All top-level definitions in this file.
    message_types: Vec<DescriptorProto>,
    enum_types: Vec<EnumDescriptorProto>,
    services: Vec<ServiceDescriptorProto>,
    extensions: Vec<FieldDescriptorProto>,
    options: Option<FileOptions>,
    /// This field contains optional information about the original source code.
    /// You may safely remove this entire field without harming runtime
    /// functionality of the descriptors -- the information is needed only by
    /// development tools.
    source_code_info: Option<SourceCodeInfo>,
    /// The syntax of the proto file.
    /// The supported values are "proto2" and "proto3".
    syntax: Option<String>,
}
