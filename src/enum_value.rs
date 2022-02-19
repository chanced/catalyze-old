use prost_types::EnumValueDescriptorProto;

use crate::{Name, Node};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValue<U> {
    pub name: Name<U>,
    pub fully_qualified_name: String,
    pub descriptor: EnumValueDescriptorProto,
}

impl<U> EnumValue<U> {
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<U>> {
        todo!()
    }
}
