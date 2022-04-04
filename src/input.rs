use std::{cell::RefCell, rc::Rc};

use crate::Parameters;

#[derive(Clone, Debug)]
pub enum Source {
    CodeGeneratorRequest(protobuf::plugin::CodeGeneratorRequest),
    FileDescriptorSet(protobuf::descriptor::FileDescriptorSet),
}
impl Source {
    pub fn files(&self) -> &[protobuf::descriptor::FileDescriptorProto] {
        match self {
            Source::CodeGeneratorRequest(ref req) => req.proto_file.as_slice(),
            Source::FileDescriptorSet(ref fds) => fds.file.as_slice(),
        }
    }
}

impl Default for Source {
    fn default() -> Self {
        Source::CodeGeneratorRequest(protobuf::plugin::CodeGeneratorRequest::default())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Input {
    src: Source,
    targets: Vec<String>,
    protoc_version: Option<semver::Version>,
    parameters: Parameters,
}

type ParamsMutator = Rc<RefCell<dyn FnMut(&mut Parameters)>>;

impl Input {
    pub fn new(src: Source, output_path: Option<String>, targets: Vec<String>) -> Self {
        let protoc_version = match src {
            Source::CodeGeneratorRequest(ref req) => {
                parse_compiler_vers(req.compiler_version.as_ref())
            }
            _ => None,
        };
        let mut parameters = Parameters::new(match src {
            Source::CodeGeneratorRequest(ref req) => {
                if req.has_parameter() {
                    Some(req.parameter())
                } else {
                    None
                }
            }
            _ => None,
        });
        if let Some(op) = output_path {
            if !op.is_empty() {
                parameters.set_output_path(op);
            }
        }
        Self {
            parameters,
            protoc_version,
            src,
            targets,
        }
    }

    pub fn mutate(&mut self, mutators: &[ParamsMutator]) {
        for mutator in mutators {
            mutator.borrow_mut()(&mut self.parameters);
        }
    }

    pub fn files(&self) -> &[protobuf::descriptor::FileDescriptorProto] {
        self.src.files()
    }
    pub fn targets(&self) -> &[String] {
        match self.src {
            Source::CodeGeneratorRequest(ref req) => req.file_to_generate.as_slice(),
            _ => self.targets.as_slice(),
        }
    }
    pub fn protoc_version(&self) -> Option<&semver::Version> {
        self.protoc_version.as_ref()
    }
    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }
}

fn parse_compiler_vers(vers: Option<&protobuf::plugin::Version>) -> Option<semver::Version> {
    vers.map(|vers| {
        let suffix = vers.suffix();
        let pre = if !suffix.is_empty() {
            semver::Prerelease::new(suffix).unwrap_or(semver::Prerelease::EMPTY)
        } else {
            semver::Prerelease::EMPTY
        };
        let build = semver::BuildMetadata::EMPTY;
        semver::Version {
            major: vers.major().abs() as u64,
            minor: vers.minor().abs() as u64,
            patch: vers.patch().abs() as u64,
            pre,
            build,
        }
    })
}
