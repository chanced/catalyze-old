mod artifact;
mod ast;
mod comments;
mod config;
mod container;
mod enum_;
mod enum_value;
mod error;
mod extension;
mod field;
mod file;
mod iter;
mod message;
mod method;
mod node;
mod oneof;
mod package;
mod proto;
mod service;
pub mod uninterpreted_option;

mod visit;
mod well_known_type;
use std::{collections::HashMap, fmt, path::PathBuf};

pub use artifact::*;
pub use ast::*;
pub use comments::*;
pub use config::*;
pub use enum_::*;
pub use enum_value::*;
pub use error::*;
pub use extension::*;
pub use field::*;
pub use file::*;
// pub use input::*;
pub use iter::*;
pub use message::*;
pub use method::*;
pub use node::*;
pub use oneof::*;
pub use package::*;
pub use proto::*;
pub use service::*;
pub use visit::*;
pub use well_known_type::*;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Scalar(Scalar),
    Enum(String),    // 14,
    Message(String), // 11,
    /// Group is not supported
    Group, //  = 10,
}
impl Type {
    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group)
    }
    pub fn is_scalar(&self) -> bool {
        matches!(self, Self::Scalar(_))
    }
    pub fn is_message(&self) -> bool {
        matches!(self, Self::Message(_))
    }
    pub fn is_enum(&self) -> bool {
        matches!(self, Self::Enum(_))
    }
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Scalar(s) => fmt::Display::fmt(s, f),
            Type::Enum(e) => fmt::Display::fmt(e, f),
            Type::Message(m) => fmt::Display::fmt(m, f),
            Type::Group => unreachable!("Group is not supported"),
        }
    }
}

use protobuf::descriptor::field_descriptor_proto::Type as FieldDescriptorType;
impl From<FieldDescriptorType> for Type {
    fn from(t: FieldDescriptorType) -> Self {
        match t.type_() {
            FieldDescriptorType::TYPE_DOUBLE => Type::Scalar(Scalar::Double),
            FieldDescriptorType::TYPE_FLOAT => Type::Scalar(Scalar::Float),
            FieldDescriptorType::TYPE_INT64 => Type::Scalar(Scalar::Int64),
            FieldDescriptorType::TYPE_UINT64 => Type::Scalar(Scalar::Uint64),
            FieldDescriptorType::TYPE_INT32 => Type::Scalar(Scalar::Int32),
            FieldDescriptorType::TYPE_FIXED64 => Type::Scalar(Scalar::Fixed64),
            FieldDescriptorType::TYPE_FIXED32 => Type::Scalar(Scalar::Fixed32),
            FieldDescriptorType::TYPE_BOOL => Type::Scalar(Scalar::Bool),
            FieldDescriptorType::TYPE_STRING => Type::Scalar(Scalar::String),
            FieldDescriptorType::TYPE_BYTES => Type::Scalar(Scalar::Bytes),
            FieldDescriptorType::TYPE_UINT32 => Type::Scalar(Scalar::Uint32),
            FieldDescriptorType::TYPE_SFIXED32 => Type::Scalar(Scalar::Sfixed32),
            FieldDescriptorType::TYPE_SFIXED64 => Type::Scalar(Scalar::Sfixed64),
            FieldDescriptorType::TYPE_SINT32 => Type::Scalar(Scalar::Sint32),
            FieldDescriptorType::TYPE_SINT64 => Type::Scalar(Scalar::Sint64),
            FieldDescriptorType::TYPE_ENUM => Type::Enum(t.type_name()),
            FieldDescriptorType::TYPE_MESSAGE => Type::Message(t.type_name()),
            FieldDescriptorType::TYPE_GROUP => Type::Group,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scalar {
    Double = 1,
    Float = 2,
    /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT64 if
    /// negative values are likely.
    Int64 = 3,
    Uint64 = 4,
    /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT32 if
    /// negative values are likely.
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    Bool = 8,
    String = 9,
    /// New in version 2.
    Bytes = 12,
    Uint32 = 13,
    Enum = 14,
    Sfixed32 = 15,
    Sfixed64 = 16,
    /// Uses ZigZag encoding.
    Sint32 = 17,
    /// Uses ZigZag encoding.
    Sint64 = 18,
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Scalar::Double => "double",
            Scalar::Float => "float",
            Scalar::Int64 => "int64",
            Scalar::Uint64 => "uint64",
            Scalar::Int32 => "int32",
            Scalar::Fixed64 => "fixed64",
            Scalar::Fixed32 => "fixed32",
            Scalar::Bool => "bool",
            Scalar::String => "string",
            Scalar::Bytes => "bytes",
            Scalar::Uint32 => "uint32",
            Scalar::Enum => "enum",
            Scalar::Sfixed32 => "sfixed32",
            Scalar::Sfixed64 => "sfixed64",
            Scalar::Sint32 => "sint32",
            Scalar::Sint64 => "sint64",
        };
        write!(f, "{}", s)
    }
}
impl TryFrom<i32> for Scalar {
    type Error = crate::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Scalar::Double),
            2 => Ok(Scalar::Float),
            3 => Ok(Scalar::Int64),
            4 => Ok(Scalar::Uint64),
            5 => Ok(Scalar::Int32),
            6 => Ok(Scalar::Fixed64),
            7 => Ok(Scalar::Fixed32),
            8 => Ok(Scalar::Bool),
            9 => Ok(Scalar::String),
            12 => Ok(Scalar::Bytes),
            13 => Ok(Scalar::Uint32),
            14 => Ok(Scalar::Enum),
            15 => Ok(Scalar::Sfixed32),
            16 => Ok(Scalar::Sfixed64),
            17 => Ok(Scalar::Sint32),
            18 => Ok(Scalar::Sint64),
            v => Err(Error::invalid_scalar(value)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum CType {
    /// Default mode.
    String = 0,
    Cord = 1,
    StringPiece = 2,
}

impl From<protobuf::descriptor::field_options::CType> for CType {
    fn from(t: protobuf::descriptor::field_options::CType) -> Self {
        match t {
            protobuf::descriptor::field_options::CType::STRING => CType::String,
            protobuf::descriptor::field_options::CType::CORD => CType::Cord,
            protobuf::descriptor::field_options::CType::STRING_PIECE => CType::StringPiece,
        }
    }
}

impl TryFrom<i32> for CType {
    type Error = crate::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CType::String),
            1 => Ok(CType::Cord),
            2 => Ok(CType::StringPiece),
            _ => Err(Error::invalid_c_type(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum JsType {
    /// Use the default type.
    Normal = 0,
    /// Use JavaScript strings.
    String = 1,
    /// Use JavaScript numbers.
    Number = 2,
}

impl From<protobuf::descriptor::field_options::JSType> for JsType {
    fn from(value: protobuf::descriptor::field_options::JSType) -> Self {
        match value {
            protobuf::descriptor::field_options::JSType::JS_NORMAL => JsType::Normal,
            protobuf::descriptor::field_options::JSType::JS_STRING => JsType::String,
            protobuf::descriptor::field_options::JSType::JS_NUMBER => JsType::Number,
        }
    }
}

impl TryFrom<i32> for JsType {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(JsType::Normal),
            1 => Ok(JsType::String),
            2 => Ok(JsType::Number),
            _ => Err(value),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Label {
    Required = 1,
    Optional = 2,
    Repeated = 3,
}

impl From<protobuf::descriptor::field_descriptor_proto::Label> for Label {
    fn from(value: protobuf::descriptor::field_descriptor_proto::Label) -> Self {
        match value {
            protobuf::descriptor::field_descriptor_proto::Label::LABEL_REQUIRED => Label::Required,
            protobuf::descriptor::field_descriptor_proto::Label::LABEL_OPTIONAL => Label::Optional,
            protobuf::descriptor::field_descriptor_proto::Label::LABEL_REPEATED => Label::Repeated,
        }
    }
}

impl TryFrom<i32> for Label {
    type Error = crate::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Label::Required),
            2 => Ok(Label::Optional),
            3 => Ok(Label::Repeated),
            _ => Err(Error::invalid_label(value)),
        }
    }
}

/// Generated classes can be optimized for speed or code size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum OptimizeMode {
    /// Generate complete code for parsing, serialization,
    Speed = 1,
    /// etc.
    ///
    /// Use ReflectionOps to implement these methods.
    CodeSize = 2,
    /// Generate code using MessageLite and the lite runtime.
    LiteRuntime = 3,
}

impl From<protobuf::descriptor::file_options::OptimizeMode> for OptimizeMode {
    fn from(value: protobuf::descriptor::file_options::OptimizeMode) -> Self {
        match value {
            protobuf::descriptor::file_options::OptimizeMode::SPEED => OptimizeMode::Speed,
            protobuf::descriptor::file_options::OptimizeMode::CODE_SIZE => OptimizeMode::CodeSize,
            protobuf::descriptor::file_options::OptimizeMode::LITE_RUNTIME => {
                OptimizeMode::LiteRuntime
            }
        }
    }
}

impl TryFrom<i32> for OptimizeMode {
    type Error = crate::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OptimizeMode::Speed),
            2 => Ok(OptimizeMode::CodeSize),
            3 => Ok(OptimizeMode::LiteRuntime),
            _ => Err(Error::invalid_optimize_mode(value)),
        }
    }
}

/// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
/// or neither? HTTP based RPC implementation may choose GET verb for safe
/// methods, and PUT verb for idempotent methods instead of the default POST.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum IdempotencyLevel {
    IdempotencyUnknown = 0,
    /// implies idempotent
    NoSideEffects = 1,
    /// idempotent, but may have side effects
    Idempotent = 2,
}
impl From<protobuf::descriptor::method_options::IdempotencyLevel> for IdempotencyLevel {
    fn from(value: protobuf::descriptor::method_options::IdempotencyLevel) -> Self {
        match value {
            protobuf::descriptor::method_options::IdempotencyLevel::IDEMPOTENT => {
                IdempotencyLevel::Idempotent
            }
            protobuf::descriptor::method_options::IdempotencyLevel::NO_SIDE_EFFECTS => {
                IdempotencyLevel::NoSideEffects
            }
            protobuf::descriptor::method_options::IdempotencyLevel::IDEMPOTENCY_UNKNOWN => {
                IdempotencyLevel::IdempotencyUnknown
            }
        }
    }
}
impl TryFrom<i32> for IdempotencyLevel {
    type Error = crate::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(IdempotencyLevel::IdempotencyUnknown),
            1 => Ok(IdempotencyLevel::NoSideEffects),
            2 => Ok(IdempotencyLevel::Idempotent),
            _ => Err(Error::invalid_idempotency_level(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Syntax {
    Proto2,
    Proto3,
}

impl Syntax {
    pub fn supports_required_prefix(&self) -> bool {
        match self {
            Syntax::Proto2 => true,
            Syntax::Proto3 => false,
        }
    }
    pub fn is_proto2(&self) -> bool {
        match self {
            Syntax::Proto2 => true,
            Syntax::Proto3 => false,
        }
    }
    pub fn is_proto3(&self) -> bool {
        match self {
            Syntax::Proto2 => false,
            Syntax::Proto3 => true,
        }
    }
}

impl TryFrom<String> for Syntax {
    type Error = crate::Error;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        match &*v.to_lowercase() {
            "proto2" => Ok(Syntax::Proto2),
            "proto3" => Ok(Syntax::Proto3),
            "" => Ok(Syntax::Proto2),
            _ => Err(Error::invalid_syntax(v)),
        }
    }
}

impl ToString for Syntax {
    fn to_string(&self) -> String {
        match self {
            Syntax::Proto2 => "proto2",
            Syntax::Proto3 => "proto3",
        }
        .to_string()
    }
}
impl From<&str> for Syntax {
    fn from(v: &str) -> Self {
        match v.to_lowercase().as_str() {
            "proto2" => Syntax::Proto2,
            "proto3" => Syntax::Proto3,
            _ => Syntax::Proto2,
        }
    }
}
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
    type Error = crate::Error;
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
    type Error = crate::Error;

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
    type Error = crate::Error;

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
    type Error = crate::Error;

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
