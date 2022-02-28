pub enum Type {
    Scalar(Scalar),
    Enum,
    Message,
    /// not supported
    Group,
}
#[derive(Debug, Clone, Copy)]
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
