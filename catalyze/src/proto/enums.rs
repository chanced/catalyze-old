use std::fmt;

use crate::Error;
use protobuf::descriptor::field_descriptor_proto::Type as FieldDescriptorType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    type Error = crate::Error<'static>;

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
