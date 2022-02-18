use crate::{Name, Node};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValue<U> {
    pub name: Name<U>,
    pub fully_qualified_name: String,
}

impl<U> EnumValue<U> {
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<U>> {
        todo!()
    }
}
