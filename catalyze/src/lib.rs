mod artifact;
mod ast;
mod comments;
mod config;
mod enum_;
mod error;
mod extension;
mod field;
mod file;
mod generator;
mod iter;
mod message;
mod method;
mod node;
mod oneof;
mod package;
mod proto;
pub mod range;
pub mod reflect_value;
pub mod service;
pub mod uninterpreted_option;
pub mod unknown;
pub mod well_known;

use std::{collections::HashMap, convert::Infallible, fmt, path::PathBuf};

use artifact::Artifact;
pub use ast::Ast;
use error::Error;
use file::File;

// #[derive(Clone, Debug)]
pub enum Source {
    CodeGeneratorRequest(protobuf::plugin::CodeGeneratorRequest),
    FileDescriptorSet(protobuf::descriptor::FileDescriptorSet),
}

pub trait Generate {
    type Error;
    fn generate(
        self,
        targets: HashMap<String, File>,
        ast: Ast,
    ) -> Result<Vec<Artifact>, Self::Error>;
}

#[derive(Clone, Debug)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: Option<u32>,
    pub prerelease: Option<String>,
    pub build_metadata: Option<String>,
    pub prefix: Option<String>,
}

pub trait Input {
    type Parameter;
    fn files(&self) -> &[protobuf::descriptor::FileDescriptorProto];
    fn protoc_version(&self) -> Option<Version>;
}

pub struct Generator {
    artifacts: Vec<Artifact>,
    targets: Vec<PathBuf>,
    files: Vec<protobuf::descriptor::FileDescriptorProto>,
}

/// The `Standalone` [`Input`] reads a `FileDescriptorSet`, typically from the
/// contents of a saved output from `protoc`, and generates based on a list of
/// target (proto) files. The output is saved to disk at the specified output
/// path.
pub struct Standalone<P> {
    file_descriptor_set: protobuf::descriptor::FileDescriptorProto,
    output_path: PathBuf,
    parameter: P,
}

// impl<P> Standalone<P> {
//     pub fn new<R, O>(input: R, output_path: O, parameter: P) -> Result<Self, Error>
//     where
//         R: Read,
//         O: AsRef<Path>,
//     {
//         Self {
//             output_path: output_path.as_ref().to_path_buf(),
//             parameter,
//         }
//     }
// }

// /// ProtocPlugins reads a `CodeGeneratorRequest` from an implementation of
// /// `Read` (typically `stdin`) and writes a `CodeGeneratorResponse` to
// /// `stdout`.
// ///
// /// The input (`stdin`) and output (`stdout`) can be configured, using any
// /// reader and writer respectively.
// pub struct ProtocPlugin {}

// pub struct Generator<W = ProtocPlugin>
// where
//     W: Workflow,
//     I: Read,
// {
//     output_path: Option<PathBuf>,
//     modules: Vec<Box<dyn Module>>,
//     targets: Vec<PathBuf>,
//     workflow: W,
//     parsed_input: Input,
//     param_mutators: Vec<ParamMutatorFn>,
// }

/// Paths for nodes in a FileDescriptorProto
#[derive(Clone, PartialEq, Eq, Copy)]
#[repr(i32)]
pub(crate) enum FileDescriptorPath {
    /// file name, relative to root of source tree
    Name = 1,
    /// FileDescriptorProto.package
    Package = 2,
    /// Names of files imported by this file.
    Dependency = 3,

    /// Indexes of the public imported files in the dependency list above.
    PublicDependency = 10,

    /// Indexes of the weak imported files in the dependency list.
    /// For Google-internal migration only. Do not use.
    WeakDependency = 11,

    // All top-level definitions in this file.
    MessageType = 4,
    /// FileDescriptorProto.enum_type
    EnumType = 5,
    /// FileDescriptorProto.service
    Service = 6,
    /// FileDescriptorProto.extension
    Extension = 7,

    Options = 8,
    /// This field contains optional information about the original source code.
    /// You may safely remove this entire field without harming runtime
    /// functionality of the descriptors -- the information is needed only by
    /// development tools.
    SourceCodeInfo = 9,

    /// FileDescriptorProto.syntax
    Syntax = 12,
}

impl FileDescriptorPath {
    const NAME: i32 = Self::Name as i32;
    const PACKAGE: i32 = Self::Package as i32;
    const DEPENDENCY: i32 = Self::Dependency as i32;
    const PUBLIC_DEPENDENCY: i32 = Self::PublicDependency as i32;
    const WEAK_DEPENDENCY: i32 = Self::WeakDependency as i32;
    const MESSAGE_TYPE: i32 = Self::MessageType as i32;
    const ENUM_TYPE: i32 = Self::EnumType as i32;
    const SERVICE: i32 = Self::Service as i32;
    const EXTENSION: i32 = Self::Extension as i32;
    const OPTIONS: i32 = Self::Options as i32;
    const SOURCE_CODE_INFO: i32 = Self::SourceCodeInfo as i32;
    const SYNTAX: i32 = Self::Syntax as i32;
}
impl TryFrom<i32> for FileDescriptorPath {
    type Error = Error;
    fn try_from(path: i32) -> Result<Self, Self::Error> {
        match path {
            Self::NAME => Ok(Self::Name),
            Self::PACKAGE => Ok(Self::Package),
            Self::DEPENDENCY => Ok(Self::Dependency),
            Self::PUBLIC_DEPENDENCY => Ok(Self::PublicDependency),
            Self::WEAK_DEPENDENCY => Ok(Self::WeakDependency),
            Self::MESSAGE_TYPE => Ok(Self::MessageType),
            Self::ENUM_TYPE => Ok(Self::EnumType),
            Self::SERVICE => Ok(Self::Service),
            Self::EXTENSION => Ok(Self::Extension),
            Self::OPTIONS => Ok(Self::Options),
            Self::SOURCE_CODE_INFO => Ok(Self::SourceCodeInfo),
            Self::SYNTAX => Ok(Self::Syntax),
            _ => Err(Error::UnknownFileDecriptorPath { path }),
        }
    }
}

impl PartialEq<i32> for FileDescriptorPath {
    fn eq(&self, other: &i32) -> bool {
        *other == *self as i32
    }
}
impl PartialEq<FileDescriptorPath> for i32 {
    fn eq(&self, other: &FileDescriptorPath) -> bool {
        *other == *self
    }
}
/// Paths for nodes in a DescriptorProto
#[derive(Clone, PartialEq, Eq, Copy)]
pub(crate) enum DescriptorPath {
    /// DescriptorProto.field
    Field = 2,
    /// DescriptorProto.nested_type
    NestedType = 3,
    /// DescriptorProto.enum_type
    EnumType = 4,

    Extension = 6,

    /// DescriptorProto.oneof_decl
    OneofDecl = 8,
}
impl DescriptorPath {
    const FIELD: i32 = Self::Field as i32;
    const NESTED_TYPE: i32 = Self::NestedType as i32;
    const ENUM_TYPE: i32 = Self::EnumType as i32;
    const ONEOF_DECL: i32 = Self::OneofDecl as i32;
}
impl TryFrom<i32> for DescriptorPath {
    type Error = crate::error::Error;

    fn try_from(path: i32) -> Result<Self, Self::Error> {
        match path {
            Self::FIELD => Ok(DescriptorPath::Field),
            Self::NESTED_TYPE => Ok(DescriptorPath::NestedType),
            Self::ENUM_TYPE => Ok(DescriptorPath::EnumType),
            Self::ONEOF_DECL => Ok(DescriptorPath::OneofDecl),
            _ => Err(Error::UnknownDecriptorPath { path }),
        }
    }
}

impl PartialEq<i32> for DescriptorPath {
    fn eq(&self, other: &i32) -> bool {
        *other == *self
    }
}
impl PartialEq<DescriptorPath> for i32 {
    fn eq(&self, other: &DescriptorPath) -> bool {
        *other == *self
    }
}

/// Paths for nodes in an EnumDescriptorProto

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum EnumDescriptorPath {
    /// EnumDescriptorProto.Value
    Value = 2,
}
impl PartialEq<i32> for EnumDescriptorPath {
    fn eq(&self, other: &i32) -> bool {
        *other == *self as i32
    }
}
impl PartialEq<EnumDescriptorPath> for i32 {
    fn eq(&self, other: &EnumDescriptorPath) -> bool {
        *other == *self
    }
}
impl EnumDescriptorPath {
    const VALUE: i32 = Self::Value as i32;
}
impl TryFrom<i32> for EnumDescriptorPath {
    type Error = crate::error::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            Self::VALUE => Ok(Self::Value),
            _ => Err(()),
        }
    }
}

// Paths for nodes in an ServiceDescriptorProto
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ServiceDescriptorPath {
    /// ServiceDescriptorProto.method
    Method = 2,
    Mixin = 6,
}

impl ServiceDescriptorPath {
    const METHOD: i32 = Self::Method as i32;
    const MIXIN: i32 = Self::Mixin as i32;
}

impl TryFrom<i32> for ServiceDescriptorPath {
    type Error = crate::error::Error;

    fn try_from(path: i32) -> Result<Self, Self::Error> {
        match path {
            Self::METHOD => Ok(Self::Method),
            Self::MIXIN => Ok(Self::Mixin),
            _ => Err(Error::UnknownServiceDecriptorPath { path }),
        }
    }
}
impl PartialEq<i32> for ServiceDescriptorPath {
    fn eq(&self, other: &i32) -> bool {
        *other == *self as i32
    }
}
impl PartialEq<ServiceDescriptorPath> for i32 {
    fn eq(&self, other: &ServiceDescriptorPath) -> bool {
        *other as i32 == *self
    }
}
