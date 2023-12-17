use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    iter::Iter,
    proto::{ServiceDescriptor, ServiceDescriptorPath},
    Comments, File, Method, Node, Nodes, Package, WeakFile,
};

#[derive(Debug, Clone)]
struct ServiceDetail {
    fqn: String,
    methods: Rc<RefCell<Vec<Method>>>,
    comments: RefCell<Comments>,
    file: WeakFile,
    descriptor: ServiceDescriptor,
}

#[derive(Debug, Clone)]
pub struct Service(Rc<ServiceDetail>);

impl Service {
    pub(crate) fn new(descriptor: ServiceDescriptor, file: File) -> Self {
        let fully_qualified_name = format!("{}.{}", file.fully_qualified_name(), descriptor.name());
        let svc = Service(Rc::new(ServiceDetail {
            fqn: fully_qualified_name,
            methods: Rc::new(RefCell::new(Vec::with_capacity(descriptor.methods().len()))),
            comments: RefCell::new(Comments::default()),
            file: file.clone().into(),
            descriptor,
        }));

        for method in descriptor.methods() {
            svc.0
                .methods
                .borrow_mut()
                .push(Method::new(method, svc.clone()));
        }
        svc
    }

    pub fn comments(&self) -> Comments {
        *self.0.comments.borrow()
    }

    pub fn file(&self) -> File {
        self.0.file.clone().into()
    }
    pub fn package(&self) -> Package {
        self.file().package()
    }
    pub(crate) fn set_comments(&self, comments: Comments) {
        self.0.comments.replace(comments);
    }
    pub fn methods(&self) -> Iter<Method> {
        Iter::from(&self.0.methods)
    }
    pub fn name(&self) -> &str {
        &self.0.name
    }
    fn downgrade(&self) -> WeakService {
        WeakService(Rc::downgrade(&self.0))
    }

    pub fn nodes(&self) -> Nodes {
        Nodes::new(vec![self.methods().into()])
    }

    pub fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node> {
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

    pub fn method(&self, name: &str) -> Option<Method> {
        self.methods().find(|m| m.name() == name)
    }
}

impl From<WeakService> for Service {
    fn from(weak: WeakService) -> Self {
        weak.upgrade()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WeakService(Weak<ServiceDetail>);

impl WeakService {
    fn upgrade(&self) -> Service {
        Service(self.0.upgrade().expect("Failed to upgrade WeakService"))
    }
}

impl From<&Service> for WeakService {
    fn from(service: &Service) -> Self {
        service.downgrade()
    }
}
impl From<Service> for WeakService {
    fn from(service: Service) -> Self {
        service.downgrade()
    }
}
