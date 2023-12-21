/// All supported "wire types" are listed in this enum.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Type {
    /// Variable-length integer
    Varint = 0,
    /// 32-bit field (e. g. `fixed64` or `double`)
    Fixed64 = 1,
    /// Length-delimited field
    LengthDelimited = 2,
    /// Groups are not supported in rust-protobuf
    StartGroup = 3,
    /// Groups are not supported in rust-protobuf
    EndGroup = 4,
    /// 32-bit field (e. g. `fixed32` or `float`)
    Fixed32 = 5,
}

impl WireType {
    /// Construct `WireType` from number, or return `None` if type is unknown.
    pub fn new(n: u32) -> Result<WireType, u32> {
        match n {
            0 => Ok(WireType::Varint),
            1 => Ok(WireType::Fixed64),
            2 => Ok(WireType::LengthDelimited),
            3 => Ok(WireType::StartGroup),
            4 => Ok(WireType::EndGroup),
            5 => Ok(WireType::Fixed32),
            _ => Err(n),
        }
    }

    #[doc(hidden)]
    pub fn for_type(field_type: field_descriptor_proto::Type) -> WireType {
        use field_descriptor_proto::Type;
        match field_type {
            Type::TYPE_INT32 => WireType::Varint,
            Type::TYPE_INT64 => WireType::Varint,
            Type::TYPE_UINT32 => WireType::Varint,
            Type::TYPE_UINT64 => WireType::Varint,
            Type::TYPE_SINT32 => WireType::Varint,
            Type::TYPE_SINT64 => WireType::Varint,
            Type::TYPE_BOOL => WireType::Varint,
            Type::TYPE_ENUM => WireType::Varint,
            Type::TYPE_FIXED32 => WireType::Fixed32,
            Type::TYPE_FIXED64 => WireType::Fixed64,
            Type::TYPE_SFIXED32 => WireType::Fixed32,
            Type::TYPE_SFIXED64 => WireType::Fixed64,
            Type::TYPE_FLOAT => WireType::Fixed32,
            Type::TYPE_DOUBLE => WireType::Fixed64,
            Type::TYPE_STRING => WireType::LengthDelimited,
            Type::TYPE_BYTES => WireType::LengthDelimited,
            Type::TYPE_MESSAGE => WireType::LengthDelimited,
            Type::TYPE_GROUP => WireType::LengthDelimited, // not true
        }
    }
}
