use std::rc::Rc;

use crate::{EnumField, MessageField};

#[derive(Clone, Debug)]
/// Google provided, Well-Known Types
///
/// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf
pub enum WellKnownTypeField<'a, U> {
    /// Any contains an arbitrary serialized message along with a URL that
    /// describes the type of the serialized message.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Any
    Any(WktMessageField<'a, U>), // message
    /// Api is a light-weight descriptor for a protocol buffer service.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Api
    Api(WktMessageField<'a, U>),
    /// Wrapper message for bool.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.BoolValue
    BoolValue(WktMessageField<'a, U>),
    /// Wrapper message for bytes.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#bytesvalue
    BytesValue(WktMessageField<'a, U>),
    /// Wrapper message for double.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#doublevalue
    DoubleValue(WktMessageField<'a, U>),
    /// A Duration represents a signed, fixed-length span of time represented as
    /// a count of seconds and fractions of seconds at nanosecond resolution. It
    /// is independent of any calendar and concepts like "day" or "month". It is
    /// related to Timestamp in that the difference between two Timestamp values
    /// is a Duration and it can be added or subtracted from a Timestamp. Range
    /// is approximately +-10,000 years.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#duration
    Duration(WktMessageField<'a, U>),
    /// A generic empty message that you can re-use to avoid defining duplicated
    /// empty messages in your APIs. A typical example is to use it as the
    /// request or the response type of an API method. For Instance:
    ///
    /// ```
    /// service Foo {
    ///     rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
    /// }
    /// ```
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty
    Empty(WktMessageField<'a, U>),
    /// Enum type definition.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#enum
    Enum(WktMessageField<'a, U>),
    /// Enum value definition.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#enumvalue
    EnumValue(WktMessageField<'a, U>),
    /// A single field of a message type.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#field
    Field(WktMessageField<'a, U>),
    /// Whether a field is optional, required, or repeated.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#cardinality
    FieldCardinality(WktEnumField<'a, U>),
    /// Basic field types.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#kind
    FieldKind(WktEnumField<'a, U>),
    /// FieldMask represents a set of symbolic field paths, for example:
    /// ```
    /// paths: "f.a"
    /// paths: "f.b.d"
    /// ```
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask
    FieldMask(WktMessageField<'a, U>),
    /// Wrapper message for float.
    ///
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#floatvalue
    FloatValue(WktMessageField<'a, U>),
    Int32Value(WktMessageField<'a, U>),
    Int64Value(WktMessageField<'a, U>),
    ListValue(WktMessageField<'a, U>),
    Method(WktMessageField<'a, U>),
    Mixin(WktMessageField<'a, U>),
    NullValue(WktEnumField<'a, U>),
    Option(WktMessageField<'a, U>),
    SourceContext(WktMessageField<'a, U>),
    StringValue(WktMessageField<'a, U>),
    Struct(WktMessageField<'a, U>),
    Syntax(WktEnumField<'a, U>),
    Timestamp(WktMessageField<'a, U>),
    Type(WktMessageField<'a, U>),
    UInt32Value(WktMessageField<'a, U>),
    UInt64Value(WktMessageField<'a, U>),
    Value(WktMessageField<'a, U>),
}

#[derive(Clone, Debug)]
pub struct WktMessageField<'a, U> {
    message: Rc<MessageField<'a, U>>,
}
#[derive(Clone, Debug)]
pub struct WktEnumField<'a, U> {
    r#enum: Rc<EnumField<'a, U>>,
}
