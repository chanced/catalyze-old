use std::{collections::VecDeque, rc::Rc, slice};

use prost_types::FileDescriptorProto;

pub struct SourceFileIter {
    files: VecDeque<Rc<FileDescriptorProto>>,
}

impl From<&[FileDescriptorProto]> for SourceFileIter {
    fn from(files: &[FileDescriptorProto]) -> Self {
        Self {
            files: files.iter().cloned().map(Rc::new).collect(),
        }
    }
}
impl From<&[Rc<FileDescriptorProto>]> for SourceFileIter {
    fn from(files: &[Rc<FileDescriptorProto>]) -> Self {
        Self {
            files: files.iter().cloned().collect(),
        }
    }
}
impl From<&Vec<prost_types::FileDescriptorProto>> for SourceFileIter {
    fn from(files: &Vec<prost_types::FileDescriptorProto>) -> Self {
        Self {
            files: files.iter().cloned().map(Rc::new).collect(),
        }
    }
}

impl SourceFileIter {
    pub fn new(files: &[FileDescriptorProto]) -> Self {
        Self::from(files)
    }
}
impl Iterator for SourceFileIter {
    type Item = Rc<FileDescriptorProto>;

    fn next(&mut self) -> Option<Self::Item> {
        self.files.pop_front().clone()
    }
}
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
    fn files(&self) -> SourceFileIter;
}

impl Source for prost_types::compiler::CodeGeneratorRequest {
    fn targets(&self) -> std::slice::Iter<String> {
        self.file_to_generate.iter()
    }

    fn files(&self) -> SourceFileIter {
        SourceFileIter::from(&self.proto_file)
    }
}

impl Source for InputSource {
    fn targets(&self) -> std::slice::Iter<String> {
        self.targets.iter()
    }

    fn files(&self) -> SourceFileIter {
        SourceFileIter::new(&self.file_descriptor_set.file)
    }
}
