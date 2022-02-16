use std::{cell::RefCell, rc::Rc};

use crate::Name;
pub(crate) type FieldList<U> = Rc<RefCell<Vec<Rc<Field<U>>>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Field<U> {
    pub name: Name<U>,
    desc: prost_types::FieldDescriptorProto,
}
