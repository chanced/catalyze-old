use std::{cell::RefCell, rc::Rc};

use prost_types::{DescriptorProto, FieldDescriptorProto};

use crate::{Name, Node};

#[derive(Debug, Clone, PartialEq)]
pub struct Field<'a, U> {
    pub name: Name<U>,
    pub desc: &'a FieldDescriptorProto,
    fully_qualified_name: String,
}

pub(crate) type FieldList<'a, U> = Rc<RefCell<Vec<Rc<Field<'a, U>>>>>;

impl<'a, U> Field<'a, U> {
    pub fn new(
        desc: &'a FieldDescriptorProto,
        msg: &'a DescriptorProto,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let field = Rc::new(Self {
            name: Name::new(desc.name(), util),
            desc,
            fully_qualified_name: msg.fully_qualified_name().to_owned(),
        });
        field
    }
}
