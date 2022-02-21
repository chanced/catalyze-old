use std::{collections::VecDeque, rc::Rc, slice};

use prost_types::FileDescriptorProto;

/// InputSource is composed of a `FileDescriptorSet` and a list of paths for
/// targeted proto files. The `FileDescriptorSet` must contain the entire tree
/// of dependencies.
///
/// In order to generate a `FileDescriptorSet` which contains all transitive
/// imports with protoc, you should both the `--inlucde-imports` and
/// `--include-source-info` flags.
///
/// Example:
/// ```bash
/// protoc --include_imports --include_source_info --proto_path=src/protos --descriptor_set_out=output.bin src/address_book/person.proto
/// ```
pub struct InputSource {
    pub file_descriptor_set: prost_types::FileDescriptorSet,
    pub targets: Vec<String>,
}

pub trait Source<'a> {
    fn targets(&self) -> &[String];
    fn files(&self) -> &[FileDescriptorProto];
}

impl<'a> Source<'a> for prost_types::compiler::CodeGeneratorRequest {
    fn targets(&self) -> &[String] {
        &self.file_to_generate
    }

    fn files(&self) -> &[FileDescriptorProto] {
        &self.proto_file
    }
}

impl<'a> Source<'a> for InputSource {
    fn targets(&self) -> &[String] {
        &self.targets
    }

    fn files(&self) -> &[FileDescriptorProto] {
        &self.file_descriptor_set.file
    }
}
