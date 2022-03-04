#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Scalar(Scalar),
    Enum,
    Message,
    /// not supported
    Group,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scalar {
    /// 0 is reserved for errors.
    /// Order is weird for historical reasons.
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
impl From<prost_types::field_descriptor_proto::Type> for Type {
    fn from(typ: prost_types::field_descriptor_proto::Type) -> Self {
        match typ {
            prost_types::field_descriptor_proto::Type::Double => Type::Scalar(Scalar::Double),
            prost_types::field_descriptor_proto::Type::Float => Type::Scalar(Scalar::Float),
            prost_types::field_descriptor_proto::Type::Int64 => Type::Scalar(Scalar::Int64),
            prost_types::field_descriptor_proto::Type::Uint64 => Type::Scalar(Scalar::Uint64),
            prost_types::field_descriptor_proto::Type::Int32 => Type::Scalar(Scalar::Int32),
            prost_types::field_descriptor_proto::Type::Fixed64 => Type::Scalar(Scalar::Fixed64),
            prost_types::field_descriptor_proto::Type::Fixed32 => Type::Scalar(Scalar::Fixed32),
            prost_types::field_descriptor_proto::Type::Bool => Type::Scalar(Scalar::Bool),
            prost_types::field_descriptor_proto::Type::String => Type::Scalar(Scalar::String),
            prost_types::field_descriptor_proto::Type::Bytes => Type::Scalar(Scalar::Bytes),
            prost_types::field_descriptor_proto::Type::Uint32 => Type::Scalar(Scalar::Uint32),
            prost_types::field_descriptor_proto::Type::Enum => Type::Enum,
            prost_types::field_descriptor_proto::Type::Sfixed32 => Type::Scalar(Scalar::Sfixed32),
            prost_types::field_descriptor_proto::Type::Sfixed64 => Type::Scalar(Scalar::Sfixed64),
            prost_types::field_descriptor_proto::Type::Sint32 => Type::Scalar(Scalar::Sint32),
            prost_types::field_descriptor_proto::Type::Sint64 => Type::Scalar(Scalar::Sint64),
            prost_types::field_descriptor_proto::Type::Group => Type::Group,
            prost_types::field_descriptor_proto::Type::Message => Type::Message,
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

impl From<prost_types::field_options::CType> for CType {
    fn from(c_type: prost_types::field_options::CType) -> Self {
        match c_type {
            prost_types::field_options::CType::String => CType::String,
            prost_types::field_options::CType::Cord => CType::Cord,
            prost_types::field_options::CType::StringPiece => CType::StringPiece,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JsType {
    /// Use the default type.
    JsNormal = 0,
    /// Use JavaScript strings.
    JsString = 1,
    /// Use JavaScript numbers.
    JsNumber = 2,
}
impl From<prost_types::field_options::JsType> for JsType {
    fn from(js_type: prost_types::field_options::JsType) -> Self {
        match js_type {
            prost_types::field_options::JsType::JsNormal => JsType::JsNormal,
            prost_types::field_options::JsType::JsString => JsType::JsString,
            prost_types::field_options::JsType::JsNumber => JsType::JsNumber,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum Label {
    Required = 1,
    Optional = 2,
    Repeated = 3,
}
impl From<prost_types::field_descriptor_proto::Label> for Label {
    fn from(label: prost_types::field_descriptor_proto::Label) -> Self {
        match label {
            prost_types::field_descriptor_proto::Label::Optional => todo!(),
            prost_types::field_descriptor_proto::Label::Required => todo!(),
            prost_types::field_descriptor_proto::Label::Repeated => todo!(),
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
impl From<prost_types::file_options::OptimizeMode> for OptimizeMode {
    fn from(optimize_mode: prost_types::file_options::OptimizeMode) -> Self {
        match optimize_mode {
            prost_types::file_options::OptimizeMode::Speed => OptimizeMode::Speed,
            prost_types::file_options::OptimizeMode::CodeSize => OptimizeMode::CodeSize,
            prost_types::file_options::OptimizeMode::LiteRuntime => OptimizeMode::LiteRuntime,
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

impl From<prost_types::method_options::IdempotencyLevel> for IdempotencyLevel {
    fn from(idempotency_level: prost_types::method_options::IdempotencyLevel) -> Self {
        match idempotency_level {
            prost_types::method_options::IdempotencyLevel::IdempotencyUnknown => {
                IdempotencyLevel::IdempotencyUnknown
            }
            prost_types::method_options::IdempotencyLevel::NoSideEffects => {
                IdempotencyLevel::NoSideEffects
            }
            prost_types::method_options::IdempotencyLevel::Idempotent => {
                IdempotencyLevel::Idempotent
            }
        }
    }
}
