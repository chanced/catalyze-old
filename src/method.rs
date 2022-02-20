use std::{cell::RefCell, rc::Rc};

use crate::{Name, Node};

pub(crate) type MethodList<U> = Rc<RefCell<Vec<Rc<Method<U>>>>>;
pub(crate) fn new_method_list<U>(cap: usize) -> MethodList<U> {
    match cap {
        0 => Rc::new(RefCell::new(Vec::new())),
        cap => Rc::new(RefCell::new(Vec::with_capacity(cap))),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Method<U> {
    pub name: Name<U>,
    pub desc: prost_types::MethodDescriptorProto,
    pub fully_qualified_name: String,
}

impl<U> Method<U> {
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<U>> {
        todo!()
    }
}
