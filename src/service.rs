use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    iter::Iter,
    proto::{ServiceDescriptor, ServiceDescriptorPath},
    Comments, File, Method, Name, Node, Nodes, Package, WeakFile,
};

#[derive(Debug, Clone)]
struct ServiceDetail<'a> {
    name: Name,
    fqn: String,
    methods: Rc<RefCell<Vec<Method<'a>>>>,
    comments: RefCell<Comments<'a>>,
    file: WeakFile<'a>,
}

#[derive(Debug, Clone)]
pub struct Service<'a>(Rc<ServiceDetail<'a>>);

impl<'a> Service<'a> {
    pub(crate) fn new(desc: ServiceDescriptor<'a>, file: File<'a>) -> Self {
        let fully_qualified_name = format!("{}.{}", file.fully_qualified_name(), desc.name());
        let svc = Service(Rc::new(ServiceDetail {
            name: desc.name().into(),
            fqn: fully_qualified_name,
            methods: Rc::new(RefCell::new(Vec::with_capacity(desc.methods().len()))),
            comments: RefCell::new(Comments::default()),
            file: file.clone().into(),
        }));

        for method in desc.methods() {
            svc.0
                .methods
                .borrow_mut()
                .push(Method::new(method, svc.clone()));
        }
        svc
    }

    pub fn comments(&self) -> Comments<'a> {
        *self.0.comments.borrow()
    }

    pub fn file(&self) -> File<'a> {
        self.0.file.clone().into()
    }
    pub fn package(&self) -> Package<'a> {
        self.file().package()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.comments.replace(comments);
    }
    pub fn methods(&self) -> Iter<Method<'a>> {
        Iter::from(&self.0.methods)
    }
    pub fn name(&self) -> &Name {
        &self.0.name
    }
    fn downgrade(&self) -> WeakService<'a> {
        WeakService(Rc::downgrade(&self.0))
    }

    pub fn nodes(&self) -> Nodes<'a> {
        Nodes::new(vec![self.methods().into()])
    }

    pub fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a>> {
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

impl<'a> From<WeakService<'a>> for Service<'a> {
    fn from(weak: WeakService<'a>) -> Self {
        weak.upgrade()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WeakService<'a>(Weak<ServiceDetail<'a>>);

impl<'a> WeakService<'a> {
    fn upgrade(&self) -> Service<'a> {
        Service(self.0.upgrade().expect("Failed to upgrade WeakService"))
    }
}

impl<'a> From<&Service<'a>> for WeakService<'a> {
    fn from(service: &Service<'a>) -> Self {
        service.downgrade()
    }
}
impl<'a> From<Service<'a>> for WeakService<'a> {
    fn from(service: Service<'a>) -> Self {
        service.downgrade()
    }
}
