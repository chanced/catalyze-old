use std::{cell::RefCell, rc::Rc};

use crate::{Method, Name, Node};

#[derive(Debug, Clone)]
pub struct Service<U> {
    pub name: Name<U>,
    pub fully_qualified_name: String,
    pub methods: Vec<Rc<Method<U>>>,
}

impl<U> Service<U> {
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<U>> {
        todo!()
    }
}
