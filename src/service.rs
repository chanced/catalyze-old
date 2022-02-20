use std::{cell::RefCell, rc::Rc};

use prost_types::ServiceDescriptorProto;

use crate::{
    container::Container, iter::Iter, path::ServiceDescriptorPath, Method, Name, Node, NodeAtPath,
};

pub(crate) type ServiceList<U> = Rc<RefCell<Vec<Rc<Service<U>>>>>;

#[derive(Debug, Clone)]
pub struct Service<U> {
    pub name: Name<U>,
    pub fully_qualified_name: String,
    pub methods: Rc<RefCell<Vec<Rc<Method<U>>>>>,
}

impl<U> Service<U> {
    pub fn new(
        desc: ServiceDescriptorProto,
        container: Container<U>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let fully_qualified_name = format!("{}.{}", container.fully_qualified_name(), desc.name());
        Rc::new(Service {
            name: Name::new(desc.name(), util.clone()),
            fully_qualified_name,
            methods: Rc::new(RefCell::new(Vec::with_capacity(desc.method.len()))),
        })
    }
    pub fn methods(&self) -> Iter<Method<U>> {
        Iter::from(&self.methods)
    }
}

impl<U> NodeAtPath<U> for Rc<Service<U>> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<U>> {
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
