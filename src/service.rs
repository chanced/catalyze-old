use std::{cell::RefCell, rc::Rc};

use prost_types::ServiceDescriptorProto;

use crate::{
    container::Container, iter::Iter, path::ServiceDescriptorPath, Method, Name, Node, NodeAtPath,
};

pub(crate) type ServiceList<'a, U> = Rc<RefCell<Vec<Rc<Service<'a, U>>>>>;

#[derive(Debug, Clone)]
pub struct Service<'a, U> {
    pub name: Name<U>,
    pub fully_qualified_name: String,
    pub methods: Rc<RefCell<Vec<Rc<Method<'a, U>>>>>,
}

impl<'a, U> Service<'a, U> {
    pub fn new(
        desc: &'a ServiceDescriptorProto,
        container: Container<'a, U>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let fully_qualified_name = format!("{}.{}", container.fully_qualified_name(), desc.name());
        Rc::new(Service {
            name: Name::new(desc.name(), util.clone()),
            fully_qualified_name,
            methods: Rc::new(RefCell::new(Vec::with_capacity(desc.method.len()))),
        })
    }
    pub fn methods(&self) -> Iter<Method<'a, U>> {
        Iter::from(&self.methods)
    }
}

impl<'a, U> NodeAtPath<'a, U> for Rc<Service<'a, U>> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            return Some(Node::Service(self.clone()));
        }
        if path.len() % 2 == 1 {
            return None;
        }
        let next = path[1] as usize;
        ServiceDescriptorPath::try_from(path[0])
            .ok()
            .and_then(|p| match p {
                ServiceDescriptorPath::Method => {
                    self.methods.borrow().get(next).cloned().map(Node::Method)
                }
                _ => None,
            })
    }
}
