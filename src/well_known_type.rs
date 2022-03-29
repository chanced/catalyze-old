use anyhow::bail;

pub const WELL_KNNOWN_TYPE_PACKAGE: &str = "google.protobuf";
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WellKnownType {
    Enum(WellKnownEnum),
    Message(WellKnownMessage),
}

impl std::str::FromStr for WellKnownType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> ::std::result::Result<WellKnownType, Self::Err> {
        match s {
            "Any" => Ok(WellKnownType::Message(WellKnownMessage::Any)),
            "Api" => Ok(WellKnownType::Message(WellKnownMessage::Api)),
            "BoolValue" => Ok(WellKnownType::Message(WellKnownMessage::BoolValue)),
            "BytesValue" => Ok(WellKnownType::Message(WellKnownMessage::BytesValue)),
            "DoubleValue" => Ok(WellKnownType::Message(WellKnownMessage::DoubleValue)),
            "Duration" => Ok(WellKnownType::Message(WellKnownMessage::Duration)),
            "Empty" => Ok(WellKnownType::Message(WellKnownMessage::Empty)),
            "Enum" => Ok(WellKnownType::Message(WellKnownMessage::Enum)),
            "EnumValue" => Ok(WellKnownType::Message(WellKnownMessage::EnumValue)),
            "Field" => Ok(WellKnownType::Message(WellKnownMessage::Field)),
            "FieldKind" => Ok(WellKnownType::Message(WellKnownMessage::FieldKind)),
            "FieldMask" => Ok(WellKnownType::Message(WellKnownMessage::FieldMask)),
            "FloatValue" => Ok(WellKnownType::Message(WellKnownMessage::FloatValue)),
            "Int32Value" => Ok(WellKnownType::Message(WellKnownMessage::Int32Value)),
            "Int64Value" => Ok(WellKnownType::Message(WellKnownMessage::Int64Value)),
            "ListValue" => Ok(WellKnownType::Message(WellKnownMessage::ListValue)),
            "Method" => Ok(WellKnownType::Message(WellKnownMessage::Method)),
            "FieldCardinality" => Ok(WellKnownType::Enum(WellKnownEnum::FieldCardinality)),
            "NullValue" => Ok(WellKnownType::Enum(WellKnownEnum::NullValue)),
            "Syntax" => Ok(WellKnownType::Enum(WellKnownEnum::Syntax)),
            _ => bail!("Not a Well-Known Type: {}", s),
        }
    }
}

impl From<WellKnownEnum> for WellKnownType {
    fn from(e: WellKnownEnum) -> Self {
        WellKnownType::Enum(e)
    }
}

impl From<WellKnownMessage> for WellKnownType {
    fn from(m: WellKnownMessage) -> Self {
        WellKnownType::Message(m)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WellKnownMessage {
    /// Any contains an arbitrary serialized message along with a URL that
    /// describes the type of the serialized message.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Any>
    Any,
    /// Api is a light-weight descriptor for a protocol buffer service.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Api>
    Api,
    /// Wrapper message for bool.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.BoolValue>
    BoolValue,
    /// Wrapper message for bytes.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#bytesvalue>
    BytesValue,
    /// Wrapper message for double.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#doublevalue>
    DoubleValue,
    /// A Duration represents a signed, fixed-length span of time represented as
    /// a count of seconds and fractions of seconds at nanosecond resolution. It
    /// is independent of any calendar and concepts like "day" or "month". It is
    /// related to Timestamp in that the difference between two Timestamp values
    /// is a Duration and it can be added or subtracted from a Timestamp. Range
    /// is approximately +-10,000 years.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#duration>
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
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty>
    Empty,
    /// Enum type definition.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#enum>
    Enum,
    /// Enum value definition.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#enumvalue>
    EnumValue,
    /// A single field of a message type.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#field>
    Field,
    FieldKind,
    /// FieldMask represents a set of symbolic field paths, for example:
    /// ```protobuf
    /// paths: "f.a"
    /// paths: "f.b.d"
    /// ```
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    FieldMask,
    /// Wrapper message for float.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#floatvalue>
    FloatValue,
    /// Wrapper message for int32.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#int32value>
    Int32Value,
    /// Wrapper message for int64.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#int64value>
    Int64Value,
    /// ListValue is a wrapper around a repeated field of values.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#listvalue>
    ListValue,
    /// Method represents a method of an api.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#method>
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
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Mixin>
    Mixin,
    /// A protocol buffer option, which can be attached to a message, field,
    /// enumeration, etc.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#option>
    Option,
    /// SourceContext represents information about the source of a protobuf
    /// element, like the file in which it is defined.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#sourcecontext>
    SourceContext,
    /// Wrapper message for string.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#stringvalue>
    StringValue,
    /// Struct represents a structured data value, consisting of fields which
    /// map to dynamically typed values. In some languages, Struct might be
    /// supported by a native representation. For example, in scripting
    /// languages like JS a struct is represented as an object. The details of
    /// that representation are described together with the proto support for
    /// the language.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct>
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
    /// <https://www.ietf.org/rfc/rfc3339.txt.>
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#timestamp>
    Timestamp,
    /// A protocol buffer message type.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#type>
    Type,
    /// Wrapper message for uint32.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#uint32value>
    UInt32Value,
    /// Wrapper message for uint64.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#uint64value>
    UInt64Value,
    /// Value represents a dynamically typed value which can be either null, a
    /// number, a string, a boolean, a recursive struct value, or a list of
    /// values. A producer of value is expected to set one of that variants,
    /// absence of any variant indicates an error.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#value>
    Value,
}

impl std::str::FromStr for WellKnownMessage {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> ::std::result::Result<WellKnownMessage, Self::Err> {
        match s {
            "Any" => Ok(WellKnownMessage::Any),
            "Api" => Ok(WellKnownMessage::Api),
            "BoolValue" => Ok(WellKnownMessage::BoolValue),
            "BytesValue" => Ok(WellKnownMessage::BytesValue),
            "DoubleValue" => Ok(WellKnownMessage::DoubleValue),
            "Duration" => Ok(WellKnownMessage::Duration),
            "Empty" => Ok(WellKnownMessage::Empty),
            "Enum" => Ok(WellKnownMessage::Enum),
            "EnumValue" => Ok(WellKnownMessage::EnumValue),
            "Field" => Ok(WellKnownMessage::Field),
            "FieldKind" => Ok(WellKnownMessage::FieldKind),
            "FieldMask" => Ok(WellKnownMessage::FieldMask),
            "FloatValue" => Ok(WellKnownMessage::FloatValue),
            "Int32Value" => Ok(WellKnownMessage::Int32Value),
            "Int64Value" => Ok(WellKnownMessage::Int64Value),
            "ListValue" => Ok(WellKnownMessage::ListValue),
            "Method" => Ok(WellKnownMessage::Method),
            "Mixin" => Ok(WellKnownMessage::Mixin),
            "Option" => Ok(WellKnownMessage::Option),
            "SourceContext" => Ok(WellKnownMessage::SourceContext),
            "StringValue" => Ok(WellKnownMessage::StringValue),
            "Struct" => Ok(WellKnownMessage::Struct),
            "Timestamp" => Ok(WellKnownMessage::Timestamp),
            "Type" => Ok(WellKnownMessage::Type),
            "UInt32Value" => Ok(WellKnownMessage::UInt32Value),
            "UInt64Value" => Ok(WellKnownMessage::UInt64Value),
            "Value" => Ok(WellKnownMessage::Value),
            _ => bail!("Not a Well-Known Message: {}", s),
        }
    }
}

impl std::str::FromStr for WellKnownEnum {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> ::std::result::Result<WellKnownEnum, Self::Err> {
        match s {
            "FieldCardinality" => Ok(WellKnownEnum::FieldCardinality),
            "FieldKind" => Ok(WellKnownEnum::FieldKind),
            "NullValue" => Ok(WellKnownEnum::NullValue),
            "Syntax" => Ok(WellKnownEnum::Syntax),
            _ => bail!("Not a Well-Known Type: {}", s),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WellKnownEnum {
    /// Whether a field is optional, required, or repeated.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#cardinality>
    FieldCardinality,
    /// Basic field types.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#kind>
    FieldKind,

    /// NullValue is a singleton enumeration to represent the null value for the
    /// Value type union.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#nullvalue>
    NullValue,
    /// The syntax in which a protocol buffer element is defined.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#syntax>
    Syntax,
}
