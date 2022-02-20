use std::{cell::RefCell, rc::Rc};

use crate::{Name, Node};

#[derive(Debug, Clone, PartialEq)]
pub struct Field<U> {
    pub name: Name<U>,
    pub desc: prost_types::FieldDescriptorProto,
    pub fully_qualified_name: String,
}

pub(crate) type FieldList<U> = Rc<RefCell<Vec<Rc<Field<U>>>>>;

impl<U> Field<U> {
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<U>> {
        todo!()
    }
}
