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

pub trait Source {
    fn targets(&self) -> slice::Iter<String>;
    fn files(&self) -> slice::Iter<FileDescriptorProto>;
}

impl Source for prost_types::compiler::CodeGeneratorRequest {
    fn targets(&self) -> slice::Iter<String> {
        self.file_to_generate.iter()
    }

    fn files(&self) -> slice::Iter<FileDescriptorProto> {
        self.proto_file.iter()
    }
}

impl Source for InputSource {
    fn targets(&self) -> std::slice::Iter<String> {
        self.targets.iter()
    }

    fn files(&self) -> std::slice::Iter<prost_types::FileDescriptorProto> {
        self.file_descriptor_set.file.iter()
    }
}
