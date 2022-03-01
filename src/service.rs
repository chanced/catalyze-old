use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use prost_types::ServiceDescriptorProto;

use crate::{
    container::Container,
    iter::Iter,
    name::Named,
    path::ServiceDescriptorPath,
    traits::{Downgrade, Upgrade},
    FullyQualified, Method, Name, Node, NodeAtPath,
};

pub(crate) type ServiceList<'a, U> = Rc<RefCell<Vec<Service<'a, U>>>>;

#[derive(Debug, Clone)]
struct ServiceDetail<'a, U> {
    name: Name<U>,
    fqn: String,
    methods: Rc<RefCell<Vec<Method<'a, U>>>>,
}

#[derive(Debug)]
pub struct Service<'a, U>(Rc<ServiceDetail<'a, U>>);

impl<'a, U> Service<'a, U> {
    pub(crate) fn new(
        desc: &'a ServiceDescriptorProto,
        container: Container<'a, U>,
        util: Rc<U>,
    ) -> Self {
        let fully_qualified_name = format!("{}.{}", container.fully_qualified_name(), desc.name());
        Service(Rc::new(ServiceDetail {
            name: Name::new(desc.name(), util.clone()),
            fqn: fully_qualified_name,
            methods: Rc::new(RefCell::new(Vec::with_capacity(desc.method.len()))),
        }))
    }

    pub fn methods(&self) -> Iter<Method<'a, U>> {
        Iter::from(&self.0.methods)
    }
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
}

impl<'a, U> Clone for Service<'a, U> {
    fn clone(&self) -> Self {
        Service(self.0.clone())
    }
}

impl<'a, U> Downgrade for Service<'a, U> {
    type Output = WeakService<'a, U>;

    fn downgrade(self) -> Self::Output {
        WeakService(Rc::downgrade(&self.0))
    }
}

impl<'a, U> NodeAtPath<'a, U> for Service<'a, U> {
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
                    self.0.methods.borrow().get(next).cloned().map(Node::Method)
                }
                _ => None,
            })
    }
}

impl<'a, U> FullyQualified for Service<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
    }
}
impl<'a, U> Named<U> for Service<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
}

#[derive(Debug)]
pub(crate) struct WeakService<'a, U>(Weak<ServiceDetail<'a, U>>);

impl<'a, U> Clone for WeakService<'a, U> {
    fn clone(&self) -> Self {
        WeakService(self.0.clone())
    }
}

impl<'a, U> Upgrade for WeakService<'a, U> {
    type Output = Service<'a, U>;

    fn upgrade(self) -> Self::Output {
        Service(self.0.upgrade().expect("Service was dropped"))
    }
}
