use prost_types::FileDescriptorSet;
pub struct Soruce<'a> {
    pub files: &'a [&'a FileDescriptorSet],
    pub targets: Vec<String>,
}

// /// InputSource is composed of a `FileDescriptorSet` and a list of paths for
// /// targeted proto files. The `FileDescriptorSet` must contain the entire tree
// /// of dependencies.
// ///
// /// In order to generate a `FileDescriptorSet` which contains all transitive
// /// imports with protoc, you should both the `--inlucde-imports` and
// /// `--include-source-info` flags.
// ///
// /// Example:
// /// ```bash
// /// protoc --include_imports --include_source_info --proto_path=src/protos --descriptor_set_out=output.bin src/address_book/person.proto
// /// ```
// pub trait Source<'a> {
//     fn targets(&'a self) -> &'a [String];
//     fn files(&'a self) -> &'a [prost_types::FileDescriptorProto];
// }

// impl<'a> Source<'a> for prost_types::compiler::CodeGeneratorRequest {
//     fn targets(&'a self) -> &'a [String] {
//         &self.file_to_generate
//     }

//     fn files(&'a self) -> &'a [prost_types::FileDescriptorProto] {
//         &self.proto_file
//     }
// }

// pub struct FileDescriptorSetSource {
//     pub file_descriptor_set: prost_types::FileDescriptorSet,
//     pub targets: Vec<String>,
// }

// impl<'a> Source<'a> for FileDescriptorSetSource {
//     fn targets(&'a self) -> &'a [String] {
//         &self.targets
//     }

//     fn files(&'a self) -> &'a [prost_types::FileDescriptorProto] {
//         &self.file_descriptor_set.file
//     }
// }

// pub struct CodeGeneratorRequestSource {
//     pub code_generator_request: prost_types::compiler::CodeGeneratorRequest,
// }

// impl<'a> Source<'a> for CodeGeneratorRequestSource {
//     fn targets(&'a self) -> &'a [String] {
//         self.code_generator_request.targets()
//     }

//     fn files(&'a self) -> &'a [prost_types::FileDescriptorProto] {
//         self.code_generator_request
//     }
// }
