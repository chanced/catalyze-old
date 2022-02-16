use std::{cell::RefCell, rc::Rc};

use crate::Name;
pub(crate) type FieldList<L> = Rc<RefCell<Vec<Rc<Field<L>>>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Field<L> {
    pub name: Name<L>,
    desc: prost_types::FieldDescriptorProto,
}
