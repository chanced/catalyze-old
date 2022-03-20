use std::{cell::RefCell, rc::Rc};

use crate::{parameters, Parameters};

#[derive(Clone, Debug)]
pub enum Source {
    CodeGeneratorRequest(prost_types::compiler::CodeGeneratorRequest),
    FileDescriptorSet(prost_types::FileDescriptorSet),
}
impl Source {
    pub fn files(&self) -> &[prost_types::FileDescriptorProto] {
        match self {
            Source::CodeGeneratorRequest(ref req) => req.proto_file.as_slice(),
            Source::FileDescriptorSet(ref fds) => fds.file.as_slice(),
        }
    }
}

impl Default for Source {
    fn default() -> Self {
        Source::CodeGeneratorRequest(prost_types::compiler::CodeGeneratorRequest::default())
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
            Source::CodeGeneratorRequest(ref req) => req.parameter.as_deref(),
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

    pub fn files<'a>(&'a self) -> &'a [prost_types::FileDescriptorProto] {
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
}

fn parse_compiler_vers(vers: Option<&prost_types::compiler::Version>) -> Option<semver::Version> {
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
