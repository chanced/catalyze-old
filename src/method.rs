use std::{cell::RefCell, rc::Rc};

use crate::proto::MethodDescriptor;
use crate::{
    Comments, File, FullyQualified, Name, Node, NodeAtPath, Package, Service, WeakService,
};
pub(crate) type MethodList<'a, U> = Rc<RefCell<Vec<Method<'a, U>>>>;

#[derive(Debug, Clone)]
struct MethodDetail<'a, U> {
    name: Name<U>,
    desc: MethodDescriptor<'a>,
    fqn: String,
    comments: RefCell<Comments<'a>>,
    service: WeakService<'a, U>,
    util: Rc<U>,
}

#[derive(Debug)]
pub struct Method<'a, U>(Rc<MethodDetail<'a, U>>);

impl<'a, U> Method<'a, U> {
    pub(crate) fn new(
        _descriptor: prost_types::MethodDescriptorProto,
        _svc: Service<'a, U>,
    ) -> Self {
        todo!()
    }

    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
    pub fn descriptor(&self) -> MethodDescriptor<'a> {
        self.0.desc.clone()
    }
    pub fn comments(&self) -> Comments<'a> {
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

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.comments.replace(comments);
    }

    pub fn util(&self) -> Rc<U> {
        self.0.util.clone()
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
