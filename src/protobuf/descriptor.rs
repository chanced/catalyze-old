use super::{
    EnumDescriptor, ExtensionRange, FieldDescriptor, MessageOptions, OneOfDescriptor, ReservedRange,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Descriptor {
    /// tag: 1
    pub name: Option<String>,
    /// tag: 2
    pub fields: Vec<FieldDescriptor>,
    /// tag: 3
    pub nested_types: Vec<Descriptor>,
    /// tag: 4
    pub enums: Vec<EnumDescriptor>,
    /// tag: 5
    pub extension_ranges: Vec<ExtensionRange>,
    /// tag: 6
    pub extensions: Vec<FieldDescriptor>,
    /// tag: 7
    pub options: Option<MessageOptions>,
    /// tag: 8
    pub one_ofs: Vec<OneOfDescriptor>,
    /// tag: 9
    pub reserved_ranges: Vec<ReservedRange>,
    /// Reserved field names, which may not be used by fields in the same message.
    /// A given name may only be reserved once.
    ///
    /// tag: 10
    pub reserved_names: Vec<String>,
}
impl Descriptor {
    pub(crate) fn new(desc: &prost_types::DescriptorProto) -> Self {
        Descriptor {
            name: desc.name.clone(),
            fields: desc.field.iter().map(FieldDescriptor::new).collect(),
            nested_types: desc.nested_type.iter().map(Descriptor::new).collect(),
            enums: desc.enum_type.iter().map(EnumDescriptor::new).collect(),
            extension_ranges: desc
                .extension_range
                .iter()
                .map(ExtensionRange::new)
                .collect(),
            extensions: desc.extension.iter().map(FieldDescriptor::new).collect(),
            options: MessageOptions::try_from(desc.options).ok(),
            one_ofs: desc.oneof_decl.iter().map(OneOfDescriptor::new).collect(),
            reserved_ranges: desc.reserved_range.iter().map(ReservedRange::new).collect(),
            reserved_names: desc.reserved_name.iter().map(|r| r.clone()).collect(),
        }
    }
}
