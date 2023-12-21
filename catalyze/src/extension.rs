use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use protobuf::reflect::FieldDescriptor;

use crate::{
    comments::Comments,
    file::File,
    iter::Iter,
    message::{Message, WeakMessage},
    node::{Container, Node, WeakContainer},
    package::Package,
};

#[derive(Debug, Clone)]
struct ExtensionDetail {
    descriptor: FieldDescriptor,
    fqn: String,
    container: WeakContainer,
    comments: RefCell<Comments>,
    extendee: RefCell<WeakMessage>,
}

#[derive(Debug, Clone)]
pub struct Extension(Rc<ExtensionDetail>);

impl Extension {
    pub(crate) fn new(desc: FieldDescriptor, container: Container) -> Self {
        let fqn = format!("{}.{}", container.fully_qualified_name(), desc.name());
        let ext = Self(Rc::new(ExtensionDetail {
            fqn,
            descriptor: desc,
            container: container.into(),
            comments: RefCell::new(Comments::default()),
            extendee: RefCell::new(WeakMessage::new()),
        }));
        ext
    }

    /// Returns the Container where the Extension is defined
    pub fn defined_in(&self) -> Container {
        self.0.container.clone().into()
    }
    /// Returns the Message that the Extension is extending.
    pub fn extendee(&self) -> Option<Message> {
        self.0.extendee.borrow().clone().into()
    }
    pub fn name(&self) -> &str {
        &self.0.descriptor.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
    }
    pub fn comments(&self) -> Comments {
        *self.0.comments.borrow()
    }
    pub fn file(&self) -> File {
        self.0.container.file()
    }
    pub fn package(&self) -> Package {
        self.file().package()
    }
    pub fn descriptor(&self) -> FieldDescriptor {
        self.0.descriptor
    }
    pub fn node_at_path(&self, path: &[i32]) -> Option<Node> {
        if path.is_empty() {
            Some(Node::Extension(self.clone()))
        } else {
            None
        }
    }
    // pub(crate) fn set_extendee(&self, msg: Message) {
    //     self.0.extendee.replace(msg.clone().into());
    // }
    pub(crate) fn set_comments(&self, comments: Comments) {
        self.0.comments.replace(comments);
    }

    fn downgrade(&self) -> WeakExtension {
        WeakExtension(Rc::downgrade(&self.0))
    }
}

impl From<WeakExtension> for Extension {
    fn from(weak: WeakExtension) -> Self {
        weak.upgrade()
    }
}
#[derive(Debug, Clone)]
pub(crate) struct WeakExtension(Weak<ExtensionDetail>);
impl WeakExtension {
    pub(crate) fn upgrade(&self) -> Extension {
        Extension(self.0.upgrade().expect("WeakExtension is expired"))
    }
}
impl From<Extension> for WeakExtension {
    fn from(ext: Extension) -> Self {
        ext.downgrade()
    }
}

#[derive(Clone, Debug)]
pub struct Extensions {
    ext_map: Rc<RefCell<HashMap<String, Extension>>>,
    ext_vec: Rc<RefCell<Vec<Extension>>>,
}
impl Extensions {
    pub fn get(&self, key: &str) -> Option<Extension> {
        self.ext_map.borrow().get(key).cloned()
    }
    pub fn len(&self) -> usize {
        self.ext_map.borrow().len()
    }
    pub fn is_empty(&self) -> bool {
        self.ext_map.borrow().is_empty()
    }
    pub fn iter(&self) -> Iter<Extension> {
        Iter::from(&self.ext_vec)
    }
    pub fn first(&self) -> Option<Extension> {
        self.ext_vec.borrow().first().cloned()
    }
    pub fn last(&self) -> Option<Extension> {
        self.ext_vec.borrow().last().cloned()
    }
    pub fn get_by_index(&self, index: usize) -> Option<Extension> {
        self.ext_vec.borrow().get(index).cloned()
    }
    pub fn contains_key(&self, key: &str) -> bool {
        self.ext_map.borrow().contains_key(key)
    }

    pub(crate) fn insert(&self, ext: Extension) {
        self.ext_vec.borrow_mut().push(ext.clone());
        self.ext_map
            .borrow_mut()
            .insert(ext.fully_qualified_name().to_string(), ext.clone());
    }

    pub fn new() -> Extensions {
        Self {
            ext_map: Default::default(),
            ext_vec: Default::default(),
        }
    }
}
impl Default for Extensions {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for Extensions {
    type Item = Extension;
    type IntoIter = Iter<Extension>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::from(&self.ext_vec)
    }
}
