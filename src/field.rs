use std::{cell::RefCell, rc::Rc};

use prost_types::FieldDescriptorProto;

use crate::{Message, Name, Node, NodeAtPath};

#[derive(Debug, Clone, PartialEq)]
pub struct Field<'a, U> {
    pub name: Name<U>,
    pub desc: &'a FieldDescriptorProto,
    pub fully_qualified_name: String,
}

pub(crate) type FieldList<'a, U> = Rc<RefCell<Vec<Rc<Field<'a, U>>>>>;

impl<'a, U> Field<'a, U> {
    pub fn new(
        desc: &'a FieldDescriptorProto,
        msg: Rc<Message<'a, U>>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let field = Rc::new(Self {
            name: Name::new(desc.name(), util),
            desc,
            fully_qualified_name: msg.fully_qualified_name.to_owned(),
        });
        field
    }
}

impl<'a, U> NodeAtPath<'a, U> for Rc<Field<'a, U>> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            Some(Node::Field(self.clone()))
        } else {
            None
        }
    }
}
