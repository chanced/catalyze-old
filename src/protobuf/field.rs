use super::{Label, Type};

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
    r#type: Option<Type>,
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
            r#type: Type::try_from(&desc.r#type).ok(),
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
    pub fn r#type(&self) -> Option<Type> {
        self.r#type.clone()
    }
    /// alias for `r#type()`
    ///
    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of `Type::Enum`, `Type::Message` or `Type::Group`.
    pub fn field_type(&self) -> Option<Type> {
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
