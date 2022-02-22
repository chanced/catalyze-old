use std::{cell::RefCell, rc::Rc};

use crate::{
    visit::{Accept, Visitor},
    Name, Node, NodeAtPath,
};

pub(crate) type MethodList<'a, U> = Rc<RefCell<Vec<Rc<Method<'a, U>>>>>;
pub(crate) fn new_method_list<'a, U>(cap: usize) -> MethodList<'a, U> {
    match cap {
        0 => Rc::new(RefCell::new(Vec::new())),
        cap => Rc::new(RefCell::new(Vec::with_capacity(cap))),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Method<'a, U> {
    pub name: Name<U>,
    pub desc: &'a prost_types::MethodDescriptorProto,
    pub fully_qualified_name: String,
}

impl<'a, U, V: Visitor<'a, U>> Accept<'a, U, V> for Rc<Method<'a, U>> {
    fn accept(&self, v: &mut V) -> Result<(), V::Error> {
        if v.done() {
            return Ok(());
        }
        v.visit_method(self.clone())
    }
}

impl<'a, U> NodeAtPath<'a, U> for Rc<Method<'a, U>> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            Some(Node::Method(self.clone()))
        } else {
            None
        }
    }
}
