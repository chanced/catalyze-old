use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    iter::Iter, proto::OneofDescriptor, Comments, Field, File, FileRefs, Message, Name, Node,
    Package, WeakFile, WeakMessage,
};

#[derive(Debug, Clone)]
pub(crate) struct OneofDetail<'a> {
    pub name: Name,
    pub desc: OneofDescriptor<'a>,
    fqn: String,
    fields: Rc<RefCell<Vec<Field<'a>>>>,
    msg: WeakMessage<'a>,
    is_synthetic: bool,
    comments: RefCell<Comments<'a>>,
    imports: Rc<RefCell<Vec<WeakFile<'a>>>>,
}

#[derive(Debug)]
pub struct Oneof<'a>(Rc<OneofDetail<'a>>);

impl<'a> Oneof<'a> {
    pub fn new(desc: OneofDescriptor<'a>, msg: Message<'a>) -> Self {
        let fully_qualified_name = format!("{}.{}", msg.fully_qualified_name(), desc.name());
        Oneof(Rc::new(OneofDetail {
            name: desc.name().into,
            desc,
            fqn: fully_qualified_name,
            fields: Rc::new(RefCell::new(Vec::default())),
            msg: msg.clone().into(),
            is_synthetic: true,
            comments: RefCell::new(Comments::default()),
            imports: Rc::new(RefCell::new(Vec::default())),
        }))
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
    pub fn name(&self) -> Name {
        self.0.name.clone()
    }
    pub fn fields(&self) -> Iter<Field<'a>> {
        Iter::from(&self.0.fields)
    }
    pub fn comments(&self) -> Comments<'a> {
        *self.0.comments.borrow()
    }
    pub fn message(&self) -> Message<'a> {
        self.0.msg.clone().into()
    }

    pub fn file(&self) -> File<'a> {
        self.0.msg.file()
    }
    pub fn descriptor(&self) -> OneofDescriptor<'a> {
        self.0.desc
    }
    pub fn package(&self) -> Package<'a> {
        self.file().package()
    }
    pub fn imports(&self) -> FileRefs<'a> {
        FileRefs::from(&self.0.imports)
    }
    pub(crate) fn add_field(&self, field: Field<'a>) {
        self.0.fields.borrow_mut().push(field.clone());
        field
            .imports()
            .for_each(|i| self.0.imports.borrow_mut().push(i.into()))
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.comments.replace(comments);
    }

    fn downgrade(&self) -> WeakOneof<'a> {
        WeakOneof(Rc::downgrade(&self.0))
    }

    pub fn is_real(&self) -> bool {
        !self.0.is_synthetic
    }
    pub fn is_synthetic(&self) -> bool {
        self.0.is_synthetic
    }
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a>> {
        if path.is_empty() {
            Some(Node::Oneof(self.clone()))
        } else {
            None
        }
    }
}

impl<'a> Clone for Oneof<'a> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'a> From<WeakOneof<'a>> for Oneof<'a> {
    fn from(oneof: WeakOneof<'a>) -> Self {
        oneof.upgrade()
    }
}
impl<'a> From<&WeakOneof<'a>> for Oneof<'a> {
    fn from(oneof: &WeakOneof<'a>) -> Self {
        oneof.upgrade()
    }
}

#[derive(Debug)]
pub(crate) struct WeakOneof<'a>(Weak<OneofDetail<'a>>);
impl<'a> Clone for WeakOneof<'a> {
    fn clone(&self) -> Self {
        WeakOneof(self.0.clone())
    }
}

impl<'a> WeakOneof<'a> {
    fn upgrade(&self) -> Oneof<'a> {
        Oneof(self.0.upgrade().expect("Failed to upgrade Oneof"))
    }
}
impl<'a> From<Oneof<'a>> for WeakOneof<'a> {
    fn from(oneof: Oneof<'a>) -> Self {
        oneof.downgrade()
    }
}
impl<'a> From<&Oneof<'a>> for WeakOneof<'a> {
    fn from(oneof: &Oneof<'a>) -> Self {
        oneof.downgrade()
    }
}
