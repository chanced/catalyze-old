use crate::{lang::Lang, Name};
#[derive(Debug, Clone, PartialEq)]
pub struct Field<L: Lang> {
    name: Name<L>,
    desc: prost_types::FieldDescriptorProto,
}
