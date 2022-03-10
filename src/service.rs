use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    iter::Iter,
    proto::{path::ServiceDescriptorPath, ServiceDescriptor},
    Comments, File, FullyQualified, Method, Name, Node, NodeAtPath, Nodes, Package, WeakFile,
};

pub(crate) type ServiceList<'a, U> = Rc<RefCell<Vec<Service<'a, U>>>>;

#[derive(Debug, Clone)]
struct ServiceDetail<'a, U> {
    name: Name<U>,
    fqn: String,
    methods: Rc<RefCell<Vec<Method<'a, U>>>>,
    comments: RefCell<Comments<'a, U>>,
    file: WeakFile<'a, U>,
    util: Rc<U>,
}

#[derive(Debug)]
pub struct Service<'a, U>(Rc<ServiceDetail<'a, U>>);

impl<'a, U> Service<'a, U> {
    pub(crate) fn new(desc: ServiceDescriptor<'a>, file: File<'a, U>) -> Self {
        let util = file.util();
        let fully_qualified_name = format!("{}.{}", file.fully_qualified_name(), desc.name());
        Service(Rc::new(ServiceDetail {
            name: Name::new(desc.name(), util.clone()),
            fqn: fully_qualified_name,
            methods: Rc::new(RefCell::new(Vec::with_capacity(desc.methods().len()))),
            comments: RefCell::new(Comments::default()),
            file: file.clone().into(),
            util: file.util(),
        }))
    }

    pub fn comments(&self) -> Comments<'a, U> {
        *self.0.comments.borrow()
    }

    pub fn file(&self) -> File<'a, U> {
        self.0.file.clone().into()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.file().package()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.comments.replace(comments);
    }
    pub fn methods(&self) -> Iter<Method<'a, U>> {
        Iter::from(&self.0.methods)
    }
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
    fn downgrade(&self) -> WeakService<'a, U> {
        WeakService(Rc::downgrade(&self.0))
    }

    pub fn nodes(&self) -> Nodes<'a, U> {
        Nodes::new(vec![self.methods().into()])
    }
    pub fn util(&self) -> Rc<U> {
        self.0.util.clone()
    }
}

impl<'a, U> Clone for Service<'a, U> {
    fn clone(&self) -> Self {
        Service(self.0.clone())
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
impl<'a, U> From<WeakService<'a, U>> for Service<'a, U> {
    fn from(weak: WeakService<'a, U>) -> Self {
        weak.upgrade()
    }
}
impl<'a, U> FullyQualified for Service<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}

#[derive(Debug)]
pub(crate) struct WeakService<'a, U>(Weak<ServiceDetail<'a, U>>);

impl<'a, U> WeakService<'a, U> {
    fn upgrade(&self) -> Service<'a, U> {
        Service(self.0.upgrade().expect("Failed to upgrade WeakService"))
    }
}

impl<'a, U> From<&Service<'a, U>> for WeakService<'a, U> {
    fn from(service: &Service<'a, U>) -> Self {
        service.downgrade()
    }
}
impl<'a, U> From<Service<'a, U>> for WeakService<'a, U> {
    fn from(service: Service<'a, U>) -> Self {
        service.downgrade()
    }
}
impl<'a, U> Clone for WeakService<'a, U> {
    fn clone(&self) -> Self {
        WeakService(self.0.clone())
    }
}
