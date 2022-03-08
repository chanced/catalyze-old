use std::{
    cell::RefCell,
    iter::FilterMap,
    rc::{Rc, Weak},
};

use crate::{
    iter::Iter, proto::OneofDescriptor, Comments, Field, File, FullyQualified, Message, Name, Node,
    NodeAtPath, Package, WeakMessage,
};
pub(crate) type OneofList<'a, U> = Rc<RefCell<Vec<Oneof<'a, U>>>>;

#[derive(Debug, Clone)]
pub(crate) struct OneofDetail<'a, U> {
    pub name: Name<U>,
    pub desc: OneofDescriptor<'a>,
    fqn: String,
    fields: Rc<RefCell<Vec<Field<'a, U>>>>,
    msg: WeakMessage<'a, U>,
    is_synthetic: bool,
    comments: RefCell<Comments<'a, U>>,
}

#[derive(Debug)]
pub struct Oneof<'a, U>(Rc<OneofDetail<'a, U>>);

impl<'a, U> Oneof<'a, U> {
    pub fn new(desc: OneofDescriptor<'a>, msg: Message<'a, U>) -> Self {
        let util = msg.util();
        let fully_qualified_name = format!("{}.{}", msg.fully_qualified_name(), desc.name());

        let o = Oneof(Rc::new(OneofDetail {
            name: Name::new(desc.name(), util.clone()),
            desc,
            fqn: fully_qualified_name,
            fields: Rc::new(RefCell::new(Vec::default())),
            msg: msg.into(),
            is_synthetic: true,
            comments: RefCell::new(Comments::default()),
        }));

        o
    }

    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
    pub fn fields(&self) -> Iter<Field<'a, U>> {
        Iter::from(&self.0.fields)
    }
    pub fn comments(&self) -> Comments<'a, U> {
        *self.0.comments.borrow()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.0.msg.clone().into()
    }

    pub fn file(&self) -> File<'a, U> {
        self.0.msg.file()
    }

    pub fn package(&self) -> Package<'a, U> {
        self.file().package()
    }
    pub fn imports(&self) -> std::slice::Iter<File<'a, U>> {
        self.fields()
            .filter_map(|f| f.imports())
            .collect::<Vec<_>>()
            .iter()
    }
    pub(crate) fn add_field(&self, field: Field<'a, U>) {
        self.0.fields.borrow_mut().push(field);
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.comments.replace(comments);
    }
    fn downgrade(&self) -> WeakOneof<'a, U> {
        WeakOneof(Rc::downgrade(&self.0))
    }
}
impl<'a, U> NodeAtPath<'a, U> for Oneof<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
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

#[cfg(test)]
impl<'a> Default for Oneof<'a, crate::util::Generic> {
    fn default() -> Self {
        let msg = Message::default();
        Oneof(Rc::new(OneofDetail {
            name: Name::default(),
            desc: Default::default(),
            fqn: Default::default(),
            fields: Default::default(),
            msg: msg.clone().into(),
            is_synthetic: false,
            comments: Default::default(),
        }))
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
