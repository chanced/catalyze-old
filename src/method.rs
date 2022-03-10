use std::{cell::RefCell, rc::Rc};

use crate::proto::MethodDescriptor;
use crate::{
    Comments, File, FullyQualified, Name, Node, NodeAtPath, Nodes, Package, Service, WeakService,
};
pub(crate) type MethodList<'a, U> = Rc<RefCell<Vec<Method<'a, U>>>>;

#[derive(Debug, Clone)]
struct MethodDetail<'a, U> {
    name: Name<U>,
    desc: MethodDescriptor<'a>,
    fqn: String,
    comments: RefCell<Comments<'a, U>>,
    service: WeakService<'a, U>,
    util: RefCell<Rc<U>>,
}

#[derive(Debug)]
pub struct Method<'a, U>(Rc<MethodDetail<'a, U>>);

impl<'a, U> Method<'a, U> {
    pub(crate) fn new(descriptor: prost_types::MethodDescriptorProto, svc: Service<'a, U>) -> Self {
        todo!()
    }

    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
    pub fn descriptor(&self) -> MethodDescriptor<'a> {
        self.0.desc.clone()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        Comments::default()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
    pub fn service(&self) -> Service<'a, U> {
        self.0.service.clone().into()
    }
    pub fn file(&self) -> File<'a, U> {
        self.service().file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.file().package()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.comments.replace(comments);
    }

    pub(crate) fn nodes(&self) -> crate::Nodes<'a, U> {
        Nodes::empty()
    }
    pub fn util(&self) -> Rc<U> {
        self.0.util.borrow().clone()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.0.util.replace(util);
    }
}

impl<'a, U> Clone for Method<'a, U> {
    fn clone(&self) -> Self {
        Method(self.0.clone())
    }
}

impl<'a, U> NodeAtPath<'a, U> for Method<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            Some(self.into())
        } else {
            None
        }
    }
}

impl<'a, U> FullyQualified for Method<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}
