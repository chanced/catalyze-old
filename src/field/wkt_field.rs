use std::rc::Rc;

use crate::{name::Named, EnumField, MessageField};

#[derive(Clone, Debug)]
/// Google provided, Well-Known Types
///
/// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf
pub enum WellKnownTypeField<'a, U> {
    /// Any contains an arbitrary serialized message along with a URL that
    /// describes the type of the serialized message.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Any
    Any(Rc<WktMessageField<'a, U>>), // message
    /// Api is a light-weight descriptor for a protocol buffer service.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Api
    Api(Rc<WktMessageField<'a, U>>),
    /// Wrapper message for bool.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.BoolValue
    BoolValue(Rc<WktMessageField<'a, U>>),
    /// Wrapper message for bytes.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#bytesvalue
    BytesValue(Rc<WktMessageField<'a, U>>),
    /// Wrapper message for double.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#doublevalue
    DoubleValue(Rc<WktMessageField<'a, U>>),
    /// A Duration represents a signed, fixed-length span of time represented as
    /// a count of seconds and fractions of seconds at nanosecond resolution. It
    /// is independent of any calendar and concepts like "day" or "month". It is
    /// related to Timestamp in that the difference between two Timestamp values
    /// is a Duration and it can be added or subtracted from a Timestamp. Range
    /// is approximately +-10,000 years.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#duration
    Duration(Rc<WktMessageField<'a, U>>),
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
    Empty(Rc<WktMessageField<'a, U>>),
    /// Enum type definition.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#enum
    Enum(Rc<WktMessageField<'a, U>>),
    /// Enum value definition.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#enumvalue
    EnumValue(Rc<WktMessageField<'a, U>>),
    /// A single field of a message type.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#field
    Field(Rc<WktMessageField<'a, U>>),
    /// Whether a field is optional, required, or repeated.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#cardinality
    FieldCardinality(Rc<WktEnumField<'a, U>>),
    /// Basic field types.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#kind
    FieldKind(Rc<WktEnumField<'a, U>>),
    /// FieldMask represents a set of symbolic field paths, for example:
    /// ```
    /// paths: "f.a"
    /// paths: "f.b.d"
    /// ```
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask
    FieldMask(Rc<WktMessageField<'a, U>>),
    /// Wrapper message for float.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#floatvalue
    FloatValue(Rc<WktMessageField<'a, U>>),
    /// Wrapper message for int32.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#int32value
    Int32Value(Rc<WktMessageField<'a, U>>),
    /// Wrapper message for int64.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#int64value
    Int64Value(Rc<WktMessageField<'a, U>>),
    /// ListValue is a wrapper around a repeated field of values.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#listvalue
    ListValue(Rc<WktMessageField<'a, U>>),
    /// Method represents a method of an api.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#method
    Method(Rc<WktMessageField<'a, U>>),
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
    Mixin(Rc<WktMessageField<'a, U>>),
    /// NullValue is a singleton enumeration to represent the null value for the
    /// Value type union.
    NullValue(Rc<WktEnumField<'a, U>>),
    /// A protocol buffer option, which can be attached to a message, field,
    /// enumeration, etc.
    Option(Rc<WktMessageField<'a, U>>),
    /// SourceContext represents information about the source of a protobuf
    /// element, like the file in which it is defined.
    SourceContext(Rc<WktMessageField<'a, U>>),
    /// Wrapper message for string.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#stringvalue
    StringValue(Rc<WktMessageField<'a, U>>),
    Struct(Rc<WktMessageField<'a, U>>),
    Syntax(Rc<WktEnumField<'a, U>>),
    Timestamp(Rc<WktMessageField<'a, U>>),
    Type(Rc<WktMessageField<'a, U>>),
    UInt32Value(Rc<WktMessageField<'a, U>>),
    UInt64Value(Rc<WktMessageField<'a, U>>),
    Value(Rc<WktMessageField<'a, U>>),
}

impl<'a, U> Named<U> for WellKnownTypeField<'a, U> {
    fn name(&self) -> crate::Name<U> {
        match self {
            WellKnownTypeField::Any(wkt)
            | WellKnownTypeField::Api(wkt)
            | WellKnownTypeField::BoolValue(wkt)
            | WellKnownTypeField::BytesValue(wkt)
            | WellKnownTypeField::DoubleValue(wkt)
            | WellKnownTypeField::Duration(wkt)
            | WellKnownTypeField::Empty(wkt)
            | WellKnownTypeField::Enum(wkt)
            | WellKnownTypeField::EnumValue(wkt)
            | WellKnownTypeField::Field(wkt)
            | WellKnownTypeField::FieldMask(wkt)
            | WellKnownTypeField::FloatValue(wkt)
            | WellKnownTypeField::Int32Value(wkt)
            | WellKnownTypeField::Int64Value(wkt)
            | WellKnownTypeField::ListValue(wkt)
            | WellKnownTypeField::Method(wkt)
            | WellKnownTypeField::Mixin(wkt)
            | WellKnownTypeField::Option(wkt)
            | WellKnownTypeField::SourceContext(wkt)
            | WellKnownTypeField::StringValue(wkt)
            | WellKnownTypeField::Struct(wkt)
            | WellKnownTypeField::Timestamp(wkt)
            | WellKnownTypeField::Type(wkt)
            | WellKnownTypeField::UInt32Value(wkt)
            | WellKnownTypeField::UInt64Value(wkt)
            | WellKnownTypeField::Value(wkt) => wkt.name(),

            WellKnownTypeField::FieldCardinality(wkt)
            | WellKnownTypeField::FieldKind(wkt)
            | WellKnownTypeField::NullValue(wkt)
            | WellKnownTypeField::Syntax(wkt) => wkt.name(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WktMessageField<'a, U> {
    message: Rc<MessageField<'a, U>>,
}
#[derive(Clone, Debug)]
pub struct WktEnumField<'a, U> {
    r#enum: Rc<EnumField<'a, U>>,
}
