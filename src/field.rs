use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use prost_types::{DescriptorProto, FieldDescriptorProto};

use crate::{
    container::WeakContainer,
    fmt_fqn,
    util::Generic,
    visit::{self, Accept, Visitor},
    FullyQualified, Message, Name, Node, NodeAtPath,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Field<'a, U> {
    pub name: Name<U>,
    pub desc: &'a FieldDescriptorProto,
    fqn: String,
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
            fqn: fmt_fqn(msg.as_ref(), desc.name()),
        });
        field
    }
    pub fn name(&self) -> Name<U> {
        self.name.clone()
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

impl<'a, U, V: Visitor<'a, U>> Accept<'a, U, V> for Rc<Field<'a, U>> {
    fn accept(&self, visitor: &mut V) -> Result<(), V::Error> {
        visitor.visit_field(self.clone())
    }
}

impl<'a, U> FullyQualified for Field<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
}
