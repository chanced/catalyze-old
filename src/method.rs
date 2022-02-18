use crate::{Name, Node};

#[derive(Debug, Clone, PartialEq)]
pub struct Method<U> {
    pub name: Name<U>,
    pub desc: prost_types::MethodDescriptorProto,
    pub fully_qualified_name: String,
}

impl<U> Method<U> {
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<U>> {
        todo!()
    }
}
