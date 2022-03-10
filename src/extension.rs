use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{
    container::{Container, WeakContainer},
    format_fqn,
    iter::Iter,
    proto::FieldDescriptor,
    Comments, File, FullyQualified, Name, Node, Nodes, Package,
};

pub(crate) type ExtensionList<'a, U> = Rc<RefCell<Vec<Extension<'a, U>>>>;
pub(crate) type ExtensionMap<'a, U> = Rc<RefCell<HashMap<String, Extension<'a, U>>>>;

#[derive(Debug, Clone)]
struct ExtensionDetail<'a, U> {
    name: Name<U>,
    desc: FieldDescriptor<'a>,
    fqn: String,
    container: WeakContainer<'a, U>,
    comments: RefCell<Comments<'a, U>>,
    util: Rc<U>,
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
            util,
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
        *self.0.comments.borrow()
    }
    pub fn file(&self) -> File<'a, U> {
        self.0.container.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.file().package()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.desc
    }
    pub fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            Some(Node::Extension(self.clone()))
        } else {
            None
        }
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.comments.replace(comments);
    }

    pub fn nodes(&self) -> Nodes<'a, U> {
        Nodes::empty()
    }
    pub fn util(&self) -> Rc<U> {
        self.0.util.clone()
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

#[derive(Debug)]
pub(crate) struct WeakExtension<'a, U>(Weak<ExtensionDetail<'a, U>>);

impl<'a, U> Clone for WeakExtension<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[derive(Clone, Debug)]
pub struct Extensions<'a, U> {
    ext_map: ExtensionMap<'a, U>,
    ext_vec: ExtensionList<'a, U>,
}
impl<'a, U> Extensions<'a, U> {
    pub fn get(&self, key: &str) -> Option<Extension<'a, U>> {
        self.ext_map.borrow().get(key).cloned()
    }
    pub fn len(&self) -> usize {
        self.ext_map.borrow().len()
    }
    pub fn is_empty(&self) -> bool {
        self.ext_map.borrow().is_empty()
    }
    pub fn iter(&self) -> Iter<Extension<'a, U>> {
        Iter::from(&self.ext_vec)
    }
    pub fn first(&self) -> Option<Extension<'a, U>> {
        self.ext_vec.borrow().first().cloned()
    }
    pub fn last(&self) -> Option<Extension<'a, U>> {
        self.ext_vec.borrow().last().cloned()
    }
    pub fn get_by_index(&self, index: usize) -> Option<Extension<'a, U>> {
        self.ext_vec.borrow().get(index).cloned()
    }
    pub fn contains_key(&self, key: &str) -> bool {
        self.ext_map.borrow().contains_key(key)
    }

    pub(crate) fn insert(&self, ext: Extension<'a, U>) {
        self.ext_vec.borrow_mut().push(ext.clone());
        self.ext_map
            .borrow_mut()
            .insert(ext.fully_qualified_name(), ext.clone());
    }

    pub fn new() -> Extensions<'a, U> {
        Self {
            ext_map: ExtensionMap::default(),
            ext_vec: ExtensionList::default(),
        }
    }
}

impl<'a, U> Default for Extensions<'a, U> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, U> IntoIterator for Extensions<'a, U> {
    type Item = Extension<'a, U>;
    type IntoIter = Iter<Extension<'a, U>>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::from(&self.ext_vec)
    }
}
