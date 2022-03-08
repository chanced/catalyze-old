use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    container::{Container, WeakContainer},
    format_fqn,
    proto::FieldDescriptor,
    Comments, File, FullyQualified, Name, Node, Package,
};

pub(crate) type ExtensionList<'a, U> = Rc<RefCell<Vec<Extension<'a, U>>>>;

#[derive(Debug, Clone)]
struct ExtensionDetail<'a, U> {
    name: Name<U>,
    desc: FieldDescriptor<'a>,
    fqn: String,
    container: WeakContainer<'a, U>,
    comments: RefCell<Comments<'a, U>>,
    util: RefCell<Rc<U>>,
}

#[derive(Debug)]
pub struct Extension<'a, U>(Rc<ExtensionDetail<'a, U>>);

impl<'a, U> Extension<'a, U> {
    pub(crate) fn new(desc: FieldDescriptor<'a>, container: Container<'a, U>) -> Self {
        let util = container.util();
        let fqn = format_fqn(&container, desc.name());
        let ext = Self(Rc::new(ExtensionDetail {
            fqn,
            name: Name::new(desc.name(), util.clone()),
            desc,
            container: container.into(),
            comments: RefCell::new(Comments::default()),
            util: RefCell::new(util.clone()),
        }));
        ext
    }
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        self.0.comments.borrow().clone()
    }
    pub fn file(&self) -> File<'a, U> {
        self.0.container.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.file().package()
    }

    pub fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.len() == 0 {
            Some(Node::Extension(self.clone()))
        } else {
            None
        }
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.comments.replace(comments);
    }
}

impl<U> FullyQualified for Extension<'_, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}
impl<'a, U> Clone for Extension<'a, U> {
    fn clone(&self) -> Self {
        Extension(self.0.clone())
    }
}

#[cfg(test)]
impl<'a> Default for Extension<'a, crate::util::Generic> {
    fn default() -> Self {
        let f = File::default();
        Self(Rc::new(ExtensionDetail {
            name: Name::default(),
            desc: FieldDescriptor::default(),
            fqn: "".to_string(),
            container: f.clone().into(),
            comments: RefCell::new(Comments::default()),
            util: RefCell::new(f.util()),
        }))
    }
}

#[derive(Debug)]
pub(crate) struct WeakExtension<'a, U>(Weak<ExtensionDetail<'a, U>>);

impl<'a, U> Clone for WeakExtension<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
