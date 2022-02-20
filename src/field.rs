use std::{cell::RefCell, rc::Rc};

use prost_types::FieldDescriptorProto;

use crate::{Name, Node};

#[derive(Debug, Clone, PartialEq)]
pub struct Field<U> {
    pub name: Name<U>,
    pub desc: FieldDescriptorProto,
    pub fully_qualified_name: String,
}

pub(crate) type FieldList<U> = Rc<RefCell<Vec<Rc<Field<U>>>>>;

impl<U> Field<U> {
    pub fn new(desc: FieldDescriptorProto)
}
