use anyhow::bail;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type<'a> {
    Scalar(Scalar),
    Enum(&'a str),    //= 14,
    Message(&'a str), //= 11,
    /// not supported
    Group, //  = 10,
}
impl<'a> Type<'a> {
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
impl<'a> From<&'a prost_types::FieldDescriptorProto> for Type<'a> {
    fn from(fd: &'a prost_types::FieldDescriptorProto) -> Self {
        let t = fd.r#type();
        match t {
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
            prost_types::field_descriptor_proto::Type::Sfixed32 => Type::Scalar(Scalar::Sfixed32),
            prost_types::field_descriptor_proto::Type::Sfixed64 => Type::Scalar(Scalar::Sfixed64),
            prost_types::field_descriptor_proto::Type::Sint32 => Type::Scalar(Scalar::Sint32),
            prost_types::field_descriptor_proto::Type::Sint64 => Type::Scalar(Scalar::Sint64),
            prost_types::field_descriptor_proto::Type::Enum => Type::Enum(fd.type_name()),
            prost_types::field_descriptor_proto::Type::Message => Type::Message(fd.type_name()),
            prost_types::field_descriptor_proto::Type::Group => Type::Group,
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

impl TryFrom<i32> for Scalar {
    type Error = anyhow::Error;
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
            v => bail!("Unknown Scalar: {}", v),
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
    fn from(t: prost_types::field_options::CType) -> Self {
        match t {
            prost_types::field_options::CType::String => CType::String,
            prost_types::field_options::CType::Cord => CType::Cord,
            prost_types::field_options::CType::StringPiece => CType::StringPiece,
        }
    }
}

impl TryFrom<Option<i32>> for CType {
    type Error = anyhow::Error;

    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => CType::try_from(v),
            None => bail!("CType is None"),
        }
    }
}
impl TryFrom<i32> for CType {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CType::String),
            1 => Ok(CType::Cord),
            2 => Ok(CType::StringPiece),
            _ => bail!("invalid CType: {}", value),
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
    fn from(value: prost_types::field_options::JsType) -> Self {
        match value {
            prost_types::field_options::JsType::JsNormal => JsType::JsNormal,
            prost_types::field_options::JsType::JsString => JsType::JsString,
            prost_types::field_options::JsType::JsNumber => JsType::JsNumber,
        }
    }
}

impl TryFrom<Option<i32>> for JsType {
    type Error = anyhow::Error;

    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => JsType::try_from(v),
            None => bail!("JsType is None"),
        }
    }
}
impl TryFrom<i32> for JsType {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(JsType::JsNormal),
            1 => Ok(JsType::JsString),
            2 => Ok(JsType::JsNumber),
            _ => bail!("invalid JsType {}", value),
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

impl From<prost_types::field_descriptor_proto::Label> for Label {
    fn from(value: prost_types::field_descriptor_proto::Label) -> Self {
        match value {
            prost_types::field_descriptor_proto::Label::Required => Label::Required,
            prost_types::field_descriptor_proto::Label::Optional => Label::Optional,
            prost_types::field_descriptor_proto::Label::Repeated => Label::Repeated,
        }
    }
}

impl TryFrom<Option<i32>> for Label {
    type Error = anyhow::Error;

    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => Label::try_from(v),
            None => bail!("Label is None"),
        }
    }
}
impl TryFrom<i32> for Label {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Label::Required),
            2 => Ok(Label::Optional),
            3 => Ok(Label::Repeated),
            _ => bail!("invalid Label {}", value),
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
    fn from(value: prost_types::file_options::OptimizeMode) -> Self {
        match value {
            prost_types::file_options::OptimizeMode::Speed => OptimizeMode::Speed,
            prost_types::file_options::OptimizeMode::CodeSize => OptimizeMode::CodeSize,
            prost_types::file_options::OptimizeMode::LiteRuntime => OptimizeMode::LiteRuntime,
        }
    }
}

impl TryFrom<Option<i32>> for OptimizeMode {
    type Error = anyhow::Error;
    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => OptimizeMode::try_from(v),
            None => bail!("OptimizeMode cannot be None"),
        }
    }
}
impl TryFrom<i32> for OptimizeMode {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OptimizeMode::Speed),
            2 => Ok(OptimizeMode::CodeSize),
            3 => Ok(OptimizeMode::LiteRuntime),
            _ => bail!("OptimizeMode cannot be {}", value),
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
    fn from(value: prost_types::method_options::IdempotencyLevel) -> Self {
        match value {
            prost_types::method_options::IdempotencyLevel::Idempotent => {
                IdempotencyLevel::Idempotent
            }
            prost_types::method_options::IdempotencyLevel::NoSideEffects => {
                IdempotencyLevel::NoSideEffects
            }
            prost_types::method_options::IdempotencyLevel::IdempotencyUnknown => {
                IdempotencyLevel::IdempotencyUnknown
            }
        }
    }
}
impl TryFrom<Option<i32>> for IdempotencyLevel {
    type Error = anyhow::Error;

    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => IdempotencyLevel::try_from(v),
            None => bail!("IdempotencyLevel can not be None"),
        }
    }
}
impl TryFrom<i32> for IdempotencyLevel {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(IdempotencyLevel::IdempotencyUnknown),
            1 => Ok(IdempotencyLevel::NoSideEffects),
            2 => Ok(IdempotencyLevel::Idempotent),
            _ => bail!("IdempotencyLevel cannot be {}", value),
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
    type Error = anyhow::Error;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        match v.as_str() {
            "proto2" => Ok(Syntax::Proto2),
            "proto3" => Ok(Syntax::Proto3),
            "" => Ok(Syntax::Proto2),
            _ => bail!("invalid syntax: {}", v),
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
