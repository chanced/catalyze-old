use anyhow::bail;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    // 0 is reserved for errors.
    // Order is weird for historical reasons.
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
    /// Tag-delimited aggregate.
    /// Group type is deprecated and not supported in proto3. However, Proto3
    /// implementations should still be able to parse the group wire format and
    /// treat group fields as unknown fields.
    Group = 10,
    /// Length-delimited aggregate.
    Message = 11,
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
impl TryFrom<&i32> for Type {
    type Error = anyhow::Error;
    fn try_from(value: &i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Type::Double),
            2 => Ok(Type::Float),
            3 => Ok(Type::Int64),
            4 => Ok(Type::Uint64),
            5 => Ok(Type::Int32),
            6 => Ok(Type::Fixed64),
            7 => Ok(Type::Fixed32),
            8 => Ok(Type::Bool),
            9 => Ok(Type::String),
            10 => Ok(Type::Group),
            11 => Ok(Type::Message),
            12 => Ok(Type::Bytes),
            13 => Ok(Type::Uint32),
            14 => Ok(Type::Enum),
            15 => Ok(Type::Sfixed32),
            16 => Ok(Type::Sfixed64),
            17 => Ok(Type::Sint32),
            18 => Ok(Type::Sint64),
            _ => bail!("invalid value for Type: {}", value),
        }
    }
}
impl TryFrom<i32> for Type {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&Option<i32>> for Type {
    type Error = anyhow::Error;
    fn try_from(value: &Option<i32>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => Self::try_from(v),
            None => bail!("value is none"),
        }
    }
}
