use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::{FullyQualified, Name, Node, NodeAtPath};

pub(crate) type MethodList<'a, U> = Rc<RefCell<Vec<Method<'a, U>>>>;

#[derive(Debug, Clone)]
struct MethodDetail<'a, U> {
    name: Name<U>,
    desc: &'a prost_types::MethodDescriptorProto,
    fqn: String,
}

#[derive(Debug)]
pub struct Method<'a, U>(Rc<MethodDetail<'a, U>>);

impl<'a, U> Method<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
}

impl<'a, U> Clone for Method<'a, U> {
    fn clone(&self) -> Self {
        Method(self.0.clone())
    }
}

impl<'a, U> Into<Node<'a, U>> for Method<'a, U> {
    fn into(self) -> Node<'a, U> {
        Node::Method(self)
    }
}

impl<'a, U> Deref for Method<'a, U> {
    type Target = Node<'a, U>;

    fn deref(&self) -> &Self::Target {
        &Node::Method(self.clone())
    }
}

impl<'a, U> NodeAtPath<'a, U> for Method<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<&Node<'a, U>> {
        if path.is_empty() {
            Some(self)
        } else {
            None
        }
    }
}

impl<'a, U> FullyQualified for Method<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
}
