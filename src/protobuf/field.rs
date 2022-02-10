use super::{Label, Opt};
use anyhow::bail;

pub struct Field {
    /// The field type.
    kind: FieldKind,
    /// The field cardinality.
    cardinality: Cardinality,
    /// The field number.
    number: i32,
    /// The field name.
    name: String,
    /// The field type URL, without the scheme, for message or enumeration
    /// types. Example: `"type.googleapis.com/google.protobuf.Timestamp"`.
    type_url: String,
    /// The index of the field type in `Type.oneofs`, for message or enumeration
    /// types. The first type has index 1; zero means the type is not in the list.
    one_of_index: i32,
    /// Whether to use alternative packed wire representation.
    packed: bool,
    /// The protocol buffer options.
    options: Vec<Opt>,
    /// The field JSON name.
    json_name: String,
    /// The string value of the default value of this field. Proto2 syntax only.
    default_value: String,
}

impl Field {
    pub(crate) fn new(field: prost_types::Field) -> Self {
        Self {
            kind: FieldKind::from(field.kind),
            cardinality: Cardinality::from(field.cardinality),
            number: field.number,
            name: field.name,
            type_url: field.type_url,
            one_of_index: field.oneof_index,
            packed: field.packed,
            options: field.options.iter().map(Opt::new).collect(),
            json_name: field.json_name,
            default_value: field.default_value,
        }
    }
    /// The field type.
    pub fn kind(&self) -> FieldKind {
        self.kind.clone()
    }
    /// The field cardinality.
    pub fn cardinality(&self) -> Cardinality {
        self.cardinality.clone()
    }
    /// The field number.
    pub fn number(&self) -> i32 {
        self.number.clone()
    }
    /// The field name.
    pub fn name(&self) -> String {
        self.name.clone()
    }
    /// The field type URL, without the scheme, for message or enumeration
    /// types. Example: `"type.googleapis.com/google.protobuf.Timestamp"`.
    pub fn type_url(&self) -> String {
        self.type_url.clone()
    }
    /// The index of the field type in `Type.oneofs`, for message or enumeration
    /// types. The first type has index 1; zero means the type is not in the list.
    pub fn one_of_index(&self) -> i32 {
        self.one_of_index.clone()
    }
    /// Whether to use alternative packed wire representation.
    pub fn packed(&self) -> bool {
        self.packed.clone()
    }
    /// The protocol buffer options.
    pub fn options(&self) -> Vec<Opt> {
        self.options.clone()
    }
    /// The field JSON name.
    pub fn json_name(&self) -> String {
        self.json_name.clone()
    }
    /// The string value of the default value of this field. Proto2 syntax only.
    pub fn default_value(&self) -> String {
        self.default_value.clone()
    }
}

/// Basic field types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FieldKind {
    /// Field type unknown.
    Unknown = 0,
    /// Field type double.
    Double = 1,
    /// Field type float.
    Float = 2,
    /// Field type int64.
    Int64 = 3,
    /// Field type uint64.
    Uint64 = 4,
    /// Field type int32.
    Int32 = 5,
    /// Field type fixed64.
    Fixed64 = 6,
    /// Field type fixed32.
    Fixed32 = 7,
    /// Field type bool.
    Bool = 8,
    /// Field type string.
    String = 9,
    /// Field type group. Proto2 syntax only, and deprecated.
    Group = 10,
    /// Field type message.
    Message = 11,
    /// Field type bytes.
    Bytes = 12,
    /// Field type uint32.
    Uint32 = 13,
    /// Field type enum.
    Enum = 14,
    /// Field type sfixed32.
    Sfixed32 = 15,
    /// Field type sfixed64.
    Sfixed64 = 16,
    /// Field type sint32.
    Sint32 = 17,
    /// Field type sint64.
    Sint64 = 18,
}

impl From<i32> for FieldKind {
    fn from(i: i32) -> Self {
        match i {
            1 => FieldKind::Double,
            2 => FieldKind::Float,
            3 => FieldKind::Int64,
            4 => FieldKind::Uint64,
            5 => FieldKind::Int32,
            6 => FieldKind::Fixed64,
            7 => FieldKind::Fixed32,
            8 => FieldKind::Bool,
            9 => FieldKind::String,
            10 => FieldKind::Group,
            11 => FieldKind::Message,
            12 => FieldKind::Bytes,
            _ => FieldKind::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Cardinality {
    /// For fields with unknown cardinality.
    Unknown = 0,
    /// For optional fields.
    Optional = 1,
    /// For required fields. Proto2 syntax only.
    Required = 2,
    /// For repeated fields.
    Repeated = 3,
}

impl From<i32> for Cardinality {
    fn from(i: i32) -> Self {
        match i {
            1 => Cardinality::Optional,
            2 => Cardinality::Required,
            3 => Cardinality::Repeated,
            _ => Cardinality::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CType {
    /// Default mode.
    String = 0,
    Cord = 1,
    StringPiece = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JsType {
    /// Use the default type.
    Normal = 0,
    /// Use JavaScript strings.
    String = 1,
    /// Use JavaScript numbers.
    Number = 2,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldType {
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
impl TryFrom<&i32> for FieldType {
    type Error = anyhow::Error;
    fn try_from(value: &i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(FieldType::Double),
            2 => Ok(FieldType::Float),
            3 => Ok(FieldType::Int64),
            4 => Ok(FieldType::Uint64),
            5 => Ok(FieldType::Int32),
            6 => Ok(FieldType::Fixed64),
            7 => Ok(FieldType::Fixed32),
            8 => Ok(FieldType::Bool),
            9 => Ok(FieldType::String),
            10 => Ok(FieldType::Group),
            11 => Ok(FieldType::Message),
            12 => Ok(FieldType::Bytes),
            13 => Ok(FieldType::Uint32),
            14 => Ok(FieldType::Enum),
            15 => Ok(FieldType::Sfixed32),
            16 => Ok(FieldType::Sfixed64),
            17 => Ok(FieldType::Sint32),
            18 => Ok(FieldType::Sint64),
            _ => bail!("invalid value for Type: {}", value),
        }
    }
}

impl TryFrom<i32> for FieldType {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&Option<i32>> for FieldType {
    type Error = anyhow::Error;
    fn try_from(value: &Option<i32>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => Self::try_from(v),
            None => bail!("value is none"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FieldDescriptor {
    /// tag: 1
    name: Option<String>,
    /// tag: 3
    number: Option<i32>,
    /// tag: 4
    label: Option<Label>,
    /// tag: 5
    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of `Type::Enum`, `Type::Message` or `Type::Group`.
    r#type: Option<FieldType>,
    /// tag: 6
    /// For message and enum types, this is the name of the type.  If the name
    /// starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
    /// rules are used to find the type (i.e. first the nested types within this
    /// message are searched, then within the parent, on up to the root
    /// namespace).
    type_name: Option<String>,
    /// tag: 2
    /// For extensions, this is the name of the type being extended.  It is
    /// resolved in the same manner as type_name.
    extendee: Option<String>,
    /// tag: 7
    /// For numeric types, contains the original text representation of the value.
    ///
    /// For booleans, `"true"` or `"false"`.
    ///
    /// For strings, contains the default text contents (not escaped in any way).
    ///
    /// For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
    default_value: Option<String>,
    /// If set, gives the index of a oneof in the containing type's oneof_decl
    /// list.  This field is a member of that oneof.
    oneof_index: Option<i32>,
}

impl FieldDescriptor {
    pub(crate) fn new(desc: &prost_types::FieldDescriptorProto) -> Self {
        FieldDescriptor {
            name: desc.name.clone(),
            number: desc.number.clone(),
            label: Label::try_from(&desc.label).ok(),
            r#type: FieldType::try_from(&desc.r#type).ok(),
            type_name: desc.type_name.clone(),
            extendee: desc.extendee.clone(),
            default_value: desc.default_value.clone(),
            oneof_index: desc.oneof_index.clone(),
        }
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }
    pub fn number(&self) -> Option<i32> {
        self.number.clone()
    }
    pub fn label(&self) -> Option<Label> {
        self.label.clone()
    }
    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of `Type::Enum`, `Type::Message` or `Type::Group`.
    pub fn r#type(&self) -> Option<FieldType> {
        self.r#type.clone()
    }
    /// alias for `r#type()`
    ///
    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of `Type::Enum`, `Type::Message` or `Type::Group`.
    pub fn field_type(&self) -> Option<FieldType> {
        self.r#type()
    }
    /// For message and enum types, this is the name of the type.  If the name
    /// starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
    /// rules are used to find the type (i.e. first the nested types within this
    /// message are searched, then within the parent, on up to the root
    /// namespace).
    pub fn type_name(&self) -> Option<String> {
        self.type_name.clone()
    }
    /// For extensions, this is the name of the type being extended.  It is
    /// resolved in the same manner as type_name.
    pub fn extendee(&self) -> Option<String> {
        self.extendee.clone()
    }
    /// For numeric types, contains the original text representation of the value.
    ///
    /// For booleans, `"true"` or `"false"`.
    ///
    /// For strings, contains the default text contents (not escaped in any way).
    ///
    /// For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
    pub fn default_value(&self) -> Option<String> {
        self.default_value.clone()
    }
    pub fn oneof_index(&self) -> Option<i32> {
        self.oneof_index.clone()
    }
}
