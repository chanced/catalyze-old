use anyhow::bail;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Scalar(Scalar),
    Enum,    //= 14,
    Message, //= 11,
    /// not supported
    Group, //  = 10,
}

impl TryFrom<Option<i32>> for Type {
    type Error = anyhow::Error;
    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => Type::try_from(v),
            None => bail!("Type is None"),
        }
    }
}

impl TryFrom<i32> for Type {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            10 => Ok(Type::Group),
            11 => Ok(Type::Message),
            14 => Ok(Type::Enum),
            v => {
                let s = Scalar::try_from(v);
                if let Ok(s) = s {
                    Ok(Type::Scalar(s))
                } else {
                    bail!("Unknown Type: {}", v);
                }
            }
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

const WELL_KNNOWN_TYPE_PACKAGE: &str = "google.protobuf";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WellKnownType {
    Enum(WellKnownEnum),
    Message(WellKnownMessage),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WellKnownMessage {
    /// Any contains an arbitrary serialized message along with a URL that
    /// describes the type of the serialized message.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Any
    Any,
    /// Api is a light-weight descriptor for a protocol buffer service.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Api
    Api,
    /// Wrapper message for bool.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.BoolValue
    BoolValue,
    /// Wrapper message for bytes.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#bytesvalue
    BytesValue,
    /// Wrapper message for double.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#doublevalue
    DoubleValue,
    /// A Duration represents a signed, fixed-length span of time represented as
    /// a count of seconds and fractions of seconds at nanosecond resolution. It
    /// is independent of any calendar and concepts like "day" or "month". It is
    /// related to Timestamp in that the difference between two Timestamp values
    /// is a Duration and it can be added or subtracted from a Timestamp. Range
    /// is approximately +-10,000 years.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#duration
    Duration,
    /// A generic empty message that you can re-use to avoid defining duplicated
    /// empty messages in your APIs. A typical example is to use it as the
    /// request or the response type of an API method. For Instance:
    ///
    /// ```protobuf
    /// service Foo {
    ///     rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
    /// }
    /// ```
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty
    Empty,
    /// Enum type definition.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#enum
    Enum,
    /// Enum value definition.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#enumvalue
    EnumValue,
    /// A single field of a message type.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#field
    Field,
    FieldKind,
    /// FieldMask represents a set of symbolic field paths, for example:
    /// ```protobuf
    /// paths: "f.a"
    /// paths: "f.b.d"
    /// ```
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask
    FieldMask,
    /// Wrapper message for float.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#floatvalue
    FloatValue,
    /// Wrapper message for int32.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#int32value
    Int32Value,
    /// Wrapper message for int64.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#int64value
    Int64Value,
    /// ListValue is a wrapper around a repeated field of values.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#listvalue
    ListValue,
    /// Method represents a method of an api.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#method
    Method,
    /// Declares an API to be included in this API. The including API must
    /// redeclare all the methods from the included API, but documentation and
    /// options are inherited as follows:
    ///
    /// If after comment and whitespace stripping, the documentation string of
    /// the redeclared method is empty, it will be inherited from the original
    /// method.
    ///
    /// Each annotation belonging to the service config (http, visibility) which
    /// is not set in the redeclared method will be inherited.
    ///
    /// If an http annotation is inherited, the path pattern will be modified as
    /// follows. Any version prefix will be replaced by the version of the
    /// including API plus the root path if specified.
    ///
    /// Example of a simple mixin:
    /// ```protobuf
    //     package google.acl.v1;
    /// service AccessControl {
    ///   // Get the underlying ACL object.
    ///   rpc GetAcl(GetAclRequest) returns (Acl) {
    ///     option (google.api.http).get = "/v1/{resource=**}:getAcl";
    ///   }
    /// }
    ///
    /// package google.storage.v2;
    /// service Storage {
    ///   //       rpc GetAcl(GetAclRequest) returns (Acl);
    ///
    ///   // Get a data record.
    ///   rpc GetData(GetDataRequest) returns (Data) {
    ///     option (google.api.http).get = "/v2/{resource=**}";
    ///   }
    /// }
    /// ```
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Mixin
    Mixin,
    /// A protocol buffer option, which can be attached to a message, field,
    /// enumeration, etc.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#option
    Option,
    /// SourceContext represents information about the source of a protobuf
    /// element, like the file in which it is defined.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#sourcecontext
    SourceContext,
    /// Wrapper message for string.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#stringvalue
    StringValue,
    /// Struct represents a structured data value, consisting of fields which
    /// map to dynamically typed values. In some languages, Struct might be
    /// supported by a native representation. For example, in scripting
    /// languages like JS a struct is represented as an object. The details of
    /// that representation are described together with the proto support for
    /// the language.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct
    Struct,
    /// A Timestamp represents a point in time independent of any time zone or
    /// calendar, represented as seconds and fractions of seconds at nanosecond
    /// resolution in UTC Epoch time. It is encoded using the Proleptic
    /// Gregorian Calendar which extends the Gregorian calendar backwards to
    /// year one. It is encoded assuming all minutes are 60 seconds long, i.e.
    /// leap seconds are "smeared" so that no leap second table is needed for
    /// interpretation. Range is from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59.999999999Z. By restricting to that range, we ensure
    /// that we can convert to and from RFC 3339 date strings. See
    /// https://www.ietf.org/rfc/rfc3339.txt.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#timestamp
    Timestamp,
    /// A protocol buffer message type.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#type
    Type,
    /// Wrapper message for uint32.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#uint32value
    UInt32Value,
    /// Wrapper message for uint64.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#uint64value
    UInt64Value,
    /// Value represents a dynamically typed value which can be either null, a
    /// number, a string, a boolean, a recursive struct value, or a list of
    /// values. A producer of value is expected to set one of that variants,
    /// absence of any variant indicates an error.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#value
    Value,
}

impl std::str::FromStr for WellKnownType {
    type Err = anyhow::Error;

    fn from_str(_s: &str) -> ::std::result::Result<WellKnownType, Self::Err> {
        //TODO: this has changed as I need to be able to identify nested enums
        todo!();

        // match s {
        //     "Any" => Ok(WellKnownType::Any),
        //     "Duration" => Ok(WellKnownType::Duration),
        //     "Empty" => Ok(WellKnownType::Empty),
        //     "Struct" => Ok(WellKnownType::Struct),
        //     "Timestamp" => Ok(WellKnownType::Timestamp),
        //     "Value" => Ok(WellKnownType::Value),
        //     "ListValue" => Ok(WellKnownType::List),
        //     "DoubleValue" => Ok(WellKnownType::Double),
        //     "FloatValue" => Ok(WellKnownType::Float),
        //     "Int64Value" => Ok(WellKnownType::Int64),
        //     "UInt64Value" => Ok(WellKnownType::Uint64),
        //     "Int32Value" => Ok(WellKnownType::Int32),
        //     "UInt32Value" => Ok(WellKnownType::Uint32),
        //     "BoolValue" => Ok(WellKnownType::Bool),
        //     "StringValue" => Ok(WellKnownType::String),
        //     "BytesValue" => Ok(WellKnownType::Bytes),
        //     _ => bail!("Unknown WellKnownType"),
        // }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WellKnownEnum {
    /// Whether a field is optional, required, or repeated.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#cardinality
    FieldCardinality,
    /// Basic field types.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#kind
    FieldKind,

    /// NullValue is a singleton enumeration to represent the null value for the
    /// Value type union.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#nullvalue
    NullValue,
    /// The syntax in which a protocol buffer element is defined.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#syntax
    Syntax,
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

/// Paths for nodes in a FileDescriptorProto

/// TODO: This could/should be turned into a tree of nested enums, where leaves end in something like Node(i32)

#[derive(Clone, PartialEq, Eq, Copy)]
#[repr(i32)]
pub enum FileDescriptorPath {
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

impl TryFrom<Option<&i32>> for FileDescriptorPath {
    type Error = anyhow::Error;
    fn try_from(v: Option<&i32>) -> Result<Self, Self::Error> {
        match v {
            Some(v) => Self::try_from(*v),
            None => bail!("value is empty and can not be converted to FileDescriptorPath"),
        }
    }
}

impl TryFrom<i32> for FileDescriptorPath {
    type Error = anyhow::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == FileDescriptorPath::Name as i32 => Ok(FileDescriptorPath::Name),
            x if x == FileDescriptorPath::Package as i32 => Ok(FileDescriptorPath::Package),
            x if x == FileDescriptorPath::Dependency as i32 => Ok(FileDescriptorPath::Dependency),
            x if x == FileDescriptorPath::PublicDependency as i32 => {
                Ok(FileDescriptorPath::PublicDependency)
            }
            x if x == FileDescriptorPath::WeakDependency as i32 => {
                Ok(FileDescriptorPath::WeakDependency)
            }
            x if x == FileDescriptorPath::MessageType as i32 => Ok(FileDescriptorPath::MessageType),
            x if x == FileDescriptorPath::EnumType as i32 => Ok(FileDescriptorPath::EnumType),
            x if x == FileDescriptorPath::Service as i32 => Ok(FileDescriptorPath::Service),
            x if x == FileDescriptorPath::Extension as i32 => Ok(FileDescriptorPath::Extension),
            x if x == FileDescriptorPath::Options as i32 => Ok(FileDescriptorPath::Options),
            x if x == FileDescriptorPath::SourceCodeInfo as i32 => {
                Ok(FileDescriptorPath::SourceCodeInfo)
            }
            x if x == FileDescriptorPath::Syntax as i32 => Ok(FileDescriptorPath::Syntax),
            _ => bail!("invalid FileDescriptorPath: {}", v),
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
        *other == *self as i32
    }
}
/// Paths for nodes in a DescriptorProto
#[derive(Clone, PartialEq, Eq, Copy)]
pub enum DescriptorPath {
    /// DescriptorProto.field
    Field = 2,
    /// DescriptorProto.nested_type
    NestedType = 3,
    /// DescriptorProto.enum_type
    EnumType = 4,
    /// DescriptorProto.oneof_decl
    OneofDecl = 8,
}

impl TryFrom<i32> for DescriptorPath {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == DescriptorPath::Field as i32 => Ok(DescriptorPath::Field),
            x if x == DescriptorPath::NestedType as i32 => Ok(DescriptorPath::NestedType),
            x if x == DescriptorPath::EnumType as i32 => Ok(DescriptorPath::EnumType),
            x if x == DescriptorPath::OneofDecl as i32 => Ok(DescriptorPath::OneofDecl),
            _ => Err(()),
        }
    }
}

impl PartialEq<i32> for DescriptorPath {
    fn eq(&self, other: &i32) -> bool {
        *other as i32 == *self
    }
}
impl PartialEq<DescriptorPath> for i32 {
    fn eq(&self, other: &DescriptorPath) -> bool {
        *other == *self as i32
    }
}

/// Paths for nodes in an EnumDescriptorProto

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnumDescriptorPath {
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
        *other == *self as i32
    }
}

impl TryFrom<i32> for EnumDescriptorPath {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == EnumDescriptorPath::Value as i32 => Ok(EnumDescriptorPath::Value),
            _ => Err(()),
        }
    }
}

// Paths for nodes in an ServiceDescriptorProto
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ServiceDescriptorPath {
    /// ServiceDescriptorProto.method
    Method = 2,
    Mixin = 6,
}
impl TryFrom<i32> for ServiceDescriptorPath {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == EnumDescriptorPath::Value as i32 => Ok(ServiceDescriptorPath::Method),
            _ => Err(()),
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
