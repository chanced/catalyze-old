pub enum Value {
    /// 32-bit unknown (e. g. `fixed32` or `float`)
    Fixed32(u32),
    /// 64-bit unknown (e. g. `fixed64` or `double`)
    Fixed64(u64),
    /// Varint unknown (e. g. `int32` or `bool`)
    Varint(u64),
    /// Length-delimited unknown (e. g. `message` or `string`)
    LengthDelimited(Vec<u8>),
}

impl Value {
    /// Wire type for this unknown
    pub fn wire_type(&self) -> WireType {
        match *self {
            Value::Fixed32(_) => WireType::Fixed32,
            Value::Fixed64(_) => WireType::Fixed64,
            Value::Varint(_) => WireType::Varint,
            Value::LengthDelimited(_) => WireType::LengthDelimited,
        }
    }

    /// As ref
    pub fn get_ref<'s>(&'s self) -> UnknownValueRef<'s> {
        match *self {
            Value::Fixed32(fixed32) => UnknownValueRef::Fixed32(fixed32),
            Value::Fixed64(fixed64) => UnknownValueRef::Fixed64(fixed64),
            Value::Varint(varint) => UnknownValueRef::Varint(varint),
            Value::LengthDelimited(ref bytes) => UnknownValueRef::LengthDelimited(&bytes),
        }
    }

    /// Construct unknown value from `int64` value.
    pub fn int32(i: i32) -> Value {
        Value::int64(i as i64)
    }

    /// Construct unknown value from `int64` value.
    pub fn int64(i: i64) -> Value {
        Value::Varint(i as u64)
    }

    /// Construct unknown value from `sint32` value.
    pub fn sint32(i: i32) -> Value {
        Value::Varint(encode_zig_zag_32(i) as u64)
    }

    /// Construct unknown value from `sint64` value.
    pub fn sint64(i: i64) -> Value {
        Value::Varint(encode_zig_zag_64(i))
    }

    /// Construct unknown value from `float` value.
    pub fn float(f: f32) -> Value {
        Value::Fixed32(f.to_bits())
    }

    /// Construct unknown value from `double` value.
    pub fn double(f: f64) -> Value {
        Value::Fixed64(f.to_bits())
    }

    /// Construct unknown value from `sfixed32` value.
    pub fn sfixed32(i: i32) -> Value {
        Value::Fixed32(i as u32)
    }

    /// Construct unknown value from `sfixed64` value.
    pub fn sfixed64(i: i64) -> Value {
        Value::Fixed64(i as u64)
    }
}

/// Field unknown values.
///
/// See [`UnknownFields`](crate::UnknownFields) for explanations.
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash)]
struct Values {
    /// 32-bit unknowns
    fixed32: Vec<u32>,
    /// 64-bit unknowns
    fixed64: Vec<u64>,
    /// Varint unknowns
    varint: Vec<u64>,
    /// Length-delimited unknowns
    length_delimited: Vec<Vec<u8>>,
}
