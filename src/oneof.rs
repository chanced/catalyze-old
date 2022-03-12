use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    iter::Iter, proto::OneofDescriptor, Comments, Field, File, Files, FullyQualified, Message,
    Name, Node, Package, WeakFile, WeakMessage,
};

#[derive(Debug, Clone)]
pub(crate) struct OneofDetail<'a, U> {
    pub name: Name<U>,
    pub desc: OneofDescriptor<'a>,
    fqn: String,
    fields: Rc<RefCell<Vec<Field<'a, U>>>>,
    msg: WeakMessage<'a, U>,
    is_synthetic: bool,
    comments: RefCell<Comments<'a>>,
    imports: Rc<RefCell<Vec<WeakFile<'a, U>>>>,
    util: Rc<U>,
}

#[derive(Debug)]
pub struct Oneof<'a, U>(Rc<OneofDetail<'a, U>>);

impl<'a, U> Oneof<'a, U> {
    pub fn new(desc: OneofDescriptor<'a>, msg: Message<'a, U>) -> Self {
        let fully_qualified_name = format!("{}.{}", msg.fully_qualified_name(), desc.name());
        Oneof(Rc::new(OneofDetail {
            name: Name::new(desc.name(), msg.util()),
            desc,
            fqn: fully_qualified_name,
            fields: Rc::new(RefCell::new(Vec::default())),
            msg: msg.clone().into(),
            is_synthetic: true,
            comments: RefCell::new(Comments::default()),
            imports: Rc::new(RefCell::new(Vec::default())),
            util: msg.util(),
        }))
    }

    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
    pub fn fields(&self) -> Iter<Field<'a, U>> {
        Iter::from(&self.0.fields)
    }
    pub fn comments(&self) -> Comments<'a> {
        *self.0.comments.borrow()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.0.msg.clone().into()
    }
    pub fn util(&self) -> Rc<U> {
        self.0.util.clone()
    }
    pub fn file(&self) -> File<'a, U> {
        self.0.msg.file()
    }
    pub fn descriptor(&self) -> OneofDescriptor<'a> {
        self.0.desc
    }
    pub fn package(&self) -> Package<'a, U> {
        self.file().package()
    }
    pub fn imports(&self) -> Files<'a, U> {
        Files::from(&self.0.imports)
    }
    pub(crate) fn add_field(&self, field: Field<'a, U>) {
        self.0.fields.borrow_mut().push(field.clone());
        field
            .imports()
            .for_each(|i| self.0.imports.borrow_mut().push(i.into()))
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.comments.replace(comments);
    }

    fn downgrade(&self) -> WeakOneof<'a, U> {
        WeakOneof(Rc::downgrade(&self.0))
    }

    pub fn is_real(&self) -> bool {
        !self.0.is_synthetic
    }
    pub fn is_synthetic(&self) -> bool {
        self.0.is_synthetic
    }
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            Some(Node::Oneof(self.clone()))
        } else {
            None
        }
    }
}

impl<'a, U> Clone for Oneof<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'a, U> From<WeakOneof<'a, U>> for Oneof<'a, U> {
    fn from(oneof: WeakOneof<'a, U>) -> Self {
        oneof.upgrade()
    }
}
impl<'a, U> From<&WeakOneof<'a, U>> for Oneof<'a, U> {
    fn from(oneof: &WeakOneof<'a, U>) -> Self {
        oneof.upgrade()
    }
}
impl<'a, U> FullyQualified for Oneof<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}

#[derive(Debug)]
pub(crate) struct WeakOneof<'a, U>(Weak<OneofDetail<'a, U>>);
impl<'a, U> Clone for WeakOneof<'a, U> {
    fn clone(&self) -> Self {
        WeakOneof(self.0.clone())
    }
}

impl<'a, U> WeakOneof<'a, U> {
    fn upgrade(&self) -> Oneof<'a, U> {
        Oneof(self.0.upgrade().expect("Failed to upgrade Oneof"))
    }
}
impl<'a, U> From<Oneof<'a, U>> for WeakOneof<'a, U> {
    fn from(oneof: Oneof<'a, U>) -> Self {
        oneof.downgrade()
    }
}
impl<'a, U> From<&Oneof<'a, U>> for WeakOneof<'a, U> {
    fn from(oneof: &Oneof<'a, U>) -> Self {
        oneof.downgrade()
    }
}
