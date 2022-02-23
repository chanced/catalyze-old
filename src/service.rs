use std::{cell::RefCell, rc::Rc};

use prost_types::ServiceDescriptorProto;

use crate::{
    container::Container,
    iter::Iter,
    path::ServiceDescriptorPath,
    visit::{Accept, Visitor},
    FullyQualified, Method, Name, Node, NodeAtPath,
};

pub(crate) type ServiceList<'a, U> = Rc<RefCell<Vec<Rc<Service<'a, U>>>>>;

#[derive(Debug, Clone)]
pub struct Service<'a, U> {
    pub name: Name<U>,
    fqn: String,
    pub methods: Rc<RefCell<Vec<Rc<Method<'a, U>>>>>,
}

impl<'a, U> Service<'a, U> {
    pub(crate) fn new(
        desc: &'a ServiceDescriptorProto,
        container: Container<'a, U>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let fully_qualified_name = format!("{}.{}", container.fully_qualified_name(), desc.name());
        Rc::new(Service {
            name: Name::new(desc.name(), util.clone()),
            fqn: fully_qualified_name,
            methods: Rc::new(RefCell::new(Vec::with_capacity(desc.method.len()))),
        })
    }
    pub fn name(&self) -> Name<U> {
        self.name.clone()
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

impl<'a, U, V: Visitor<'a, U>> Accept<'a, U, V> for Rc<Service<'a, U>> {
    fn accept(&self, v: &mut V) -> Result<(), V::Error> {
        if v.done() {
            return Ok(());
        }
        v.visit_service(self.clone())?;
        for mth in self.methods.borrow().iter() {
            mth.accept(v)?;
        }
        Ok(())
    }
}

impl<'a, U> FullyQualified for Rc<Service<'a, U>> {
    fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
}
