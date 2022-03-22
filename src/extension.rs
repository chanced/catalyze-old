use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{
    container::{Container, WeakContainer},
    iter::Iter,
    proto::FieldDescriptor,
    Comments, File, Message, Name, Node, Package, WeakMessage,
};

#[derive(Debug, Clone)]
struct ExtensionDetail<'a> {
    name: Name,
    desc: FieldDescriptor<'a>,
    fqn: String,
    container: WeakContainer<'a>,
    comments: RefCell<Comments<'a>>,
    extendee: RefCell<WeakMessage<'a>>,
}

#[derive(Debug, Clone)]
pub struct Extension<'a>(Rc<ExtensionDetail<'a>>);

impl<'a> Extension<'a> {
    pub(crate) fn new(desc: FieldDescriptor<'a>, container: Container<'a>) -> Self {
        let fqn = format!("{}.{}", container.fully_qualified_name(), desc.name());
        let ext = Self(Rc::new(ExtensionDetail {
            fqn,
            name: desc.name().into(),
            desc,
            container: container.into(),
            comments: RefCell::new(Comments::default()),
            extendee: RefCell::new(WeakMessage::new()),
        }));
        ext
    }
    /// Returns the Container where the Extension is defined
    pub fn defined_in(&self) -> Container<'a> {
        self.0.container.clone().into()
    }
    /// Returns the Message that the Extension is extending.
    pub fn extendee(&self) -> Option<Message<'a>> {
        self.0.extendee.borrow().clone().into()
    }
    pub fn name(&self) -> &Name {
        &self.0.name
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
    pub fn comments(&self) -> Comments<'a> {
        *self.0.comments.borrow()
    }
    pub fn file(&self) -> File<'a> {
        self.0.container.file()
    }
    pub fn package(&self) -> Package<'a> {
        self.file().package()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.desc
    }
    pub fn node_at_path(&self, path: &[i32]) -> Option<Node<'a>> {
        if path.is_empty() {
            Some(Node::Extension(self.clone()))
        } else {
            None
        }
    }
    pub(crate) fn set_extendee(&self, msg: Message<'a>) {
        self.0.extendee.replace(msg.clone().into());
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.comments.replace(comments);
    }

    fn downgrade(&self) -> WeakExtension<'a> {
        WeakExtension(Rc::downgrade(&self.0))
    }
}

impl<'a> From<WeakExtension<'a>> for Extension<'a> {
    fn from(weak: WeakExtension<'a>) -> Self {
        weak.upgrade()
    }
}
#[derive(Debug, Clone)]
pub(crate) struct WeakExtension<'a>(Weak<ExtensionDetail<'a>>);
impl<'a> WeakExtension<'a> {
    fn upgrade(&self) -> Extension<'a> {
        Extension(self.0.upgrade().expect("WeakExtension is expired"))
    }
}
impl<'a> From<Extension<'a>> for WeakExtension<'a> {
    fn from(ext: Extension<'a>) -> Self {
        ext.downgrade()
    }
}

#[derive(Clone, Debug)]
pub struct Extensions<'a> {
    ext_map: Rc<RefCell<HashMap<String, Extension<'a>>>>,
    ext_vec: Rc<RefCell<Vec<Extension<'a>>>>,
}
impl<'a> Extensions<'a> {
    pub fn get(&self, key: &str) -> Option<Extension<'a>> {
        self.ext_map.borrow().get(key).cloned()
    }
    pub fn len(&self) -> usize {
        self.ext_map.borrow().len()
    }
    pub fn is_empty(&self) -> bool {
        self.ext_map.borrow().is_empty()
    }
    pub fn iter(&self) -> Iter<Extension<'a>> {
        Iter::from(&self.ext_vec)
    }
    pub fn first(&self) -> Option<Extension<'a>> {
        self.ext_vec.borrow().first().cloned()
    }
    pub fn last(&self) -> Option<Extension<'a>> {
        self.ext_vec.borrow().last().cloned()
    }
    pub fn get_by_index(&self, index: usize) -> Option<Extension<'a>> {
        self.ext_vec.borrow().get(index).cloned()
    }
    pub fn contains_key(&self, key: &str) -> bool {
        self.ext_map.borrow().contains_key(key)
    }

    pub(crate) fn insert(&self, ext: Extension<'a>) {
        self.ext_vec.borrow_mut().push(ext.clone());
        self.ext_map
            .borrow_mut()
            .insert(ext.fully_qualified_name(), ext.clone());
    }

    pub fn new() -> Extensions<'a> {
        Self {
            ext_map: Default::default(),
            ext_vec: Default::default(),
        }
    }
}
impl<'a> Default for Extensions<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for Extensions<'a> {
    type Item = Extension<'a>;
    type IntoIter = Iter<Extension<'a>>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::from(&self.ext_vec)
    }
}
