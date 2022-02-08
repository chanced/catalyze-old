use super::{
    EnumDescriptor, ExtensionRange, FieldDescriptor, MessageOptions, OneOfDescriptor, ReservedRange,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Descriptor {
    /// tag: 1
    name: Option<String>,
    /// tag: 2
    fields: Vec<FieldDescriptor>,
    /// tag: 3
    nested_types: Vec<Descriptor>,
    /// tag: 4
    enums: Vec<EnumDescriptor>,
    /// tag: 5
    extension_ranges: Vec<ExtensionRange>,
    /// tag: 6
    extensions: Vec<FieldDescriptor>,

    /// tag: 7
    options: Option<MessageOptions>,
    /// tag: 8
    one_ofs: Vec<OneOfDescriptor>,
    /// tag: 9
    reserved_ranges: Vec<ReservedRange>,
    /// Reserved field names, which may not be used by fields in the same message.
    /// A given name may only be reserved once.
    ///
    /// tag: 10
    reserved_names: Vec<String>,
}
impl Descriptor {
    pub(crate) fn new(desc: &prost_types::DescriptorProto) -> Self {
        Descriptor {
            name: desc.name.clone(),
            fields: desc.field.iter().map(|f| FieldDescriptor::new(f)).collect(),
            nested_types: desc
                .nested_type
                .iter()
                .map(|n| Descriptor::new(n))
                .collect(),
            enums: desc
                .enum_type
                .iter()
                .map(|e| EnumDescriptor::new(e))
                .collect(),
            extension_ranges: desc
                .extension_range
                .iter()
                .map(|e| ExtensionRange::new(e))
                .collect(),
            extensions: desc
                .extension
                .iter()
                .map(|e| FieldDescriptor::new(e))
                .collect(),
            options: desc.options.map(|o| MessageOptions::new(&o)),
            one_ofs: desc
                .oneof_decl
                .iter()
                .map(|o| OneOfDescriptor::new(o))
                .collect(),
            reserved_ranges: desc
                .reserved_range
                .iter()
                .map(|r| ReservedRange::new(r))
                .collect(),
            reserved_names: desc.reserved_name.iter().map(|r| r.clone()).collect(),
        }
    }
}
