use std::{cell::RefCell, rc::Rc};

use crate::{Name, Node};
use prost_types::EnumValueDescriptorProto;

pub(crate) type EnumValueList<U> = Rc<RefCell<Vec<Rc<EnumValue<U>>>>>;

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
