use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use prost_types::EnumDescriptorProto;

use crate::{
    container::{Container, WeakContainer},
    iter::Iter,
    proto::EnumDescriptorPath,
    EnumValue, FullyQualified, Name, Node, NodeAtPath, Package, WeakMessage,
};

pub(crate) type EnumList<'a, U> = Rc<RefCell<Vec<Enum<'a, U>>>>;

#[derive(Debug, Clone)]
struct EnumDetail<'a, U> {
    name: Name<U>,
    fqn: String,
    values: Rc<RefCell<Vec<EnumValue<'a, U>>>>,
    container: WeakContainer<'a, U>,
    dependents: Rc<RefCell<Vec<WeakMessage<'a, U>>>>,
    util: RefCell<Rc<U>>,
}

#[derive(Debug)]
pub struct Enum<'a, U>(Rc<EnumDetail<'a, U>>);

impl<'a, U> Enum<'a, U> {
    pub(crate) fn new(desc: &'a EnumDescriptorProto, container: Container<'a, U>) -> Self {
        let util = container.util();
        let fully_qualified_name = format!("{}.{}", container.fully_qualified_name(), desc.name());

        let e = Enum(Rc::new(EnumDetail {
            name: Name::new(desc.name(), util.clone()),
            values: Rc::new(RefCell::new(Vec::with_capacity(desc.value.len()))),
            container: container.into(),
            dependents: Rc::new(RefCell::new(Vec::default())),
            fqn: fully_qualified_name,
            util: RefCell::new(util),
        }));
        {
            let mut values = e.0.values.borrow_mut();
            for v in &desc.value {
                values.push(EnumValue::new(v, e.clone()));
            }
        }
        e
    }

    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
    pub fn util(&self) -> Rc<U> {
        self.0.util.borrow().clone()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.0.util.replace(util);
    }
    pub fn values(&self) -> Iter<EnumValue<'a, U>> {
        Iter::from(&self.0.values)
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.container.package()
    }
    fn downgrade(&self) -> WeakEnum<'a, U> {
        WeakEnum(Rc::downgrade(&self.0))
    }
}

impl<'a, U> Clone for Enum<'a, U> {
    fn clone(&self) -> Self {
        Enum(self.0.clone())
    }
}

impl<'a, U> NodeAtPath<'a, U> for Enum<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            return Some(Node::Enum(self.clone()));
        }
        if path.len() != 2 {
            return None;
        }

        let next = path[1] as usize;
        EnumDescriptorPath::try_from(path[0])
            .ok()
            .and_then(|p| match p {
                EnumDescriptorPath::Value => self.0.values.borrow().get(next).map(Into::into),
                // _ => None,
            })
    }
}

impl<'a, U> FullyQualified for Enum<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}
impl<'a, U> From<WeakEnum<'a, U>> for Enum<'a, U> {
    fn from(e: WeakEnum<'a, U>) -> Self {
        e.upgrade()
    }
}
impl<'a, U> From<&WeakEnum<'a, U>> for Enum<'a, U> {
    fn from(e: &WeakEnum<'a, U>) -> Self {
        e.upgrade()
    }
}
#[derive(Debug)]
pub(crate) struct WeakEnum<'a, U>(Weak<EnumDetail<'a, U>>);
impl<'a, U> WeakEnum<'a, U> {
    fn upgrade(&self) -> Enum<'a, U> {
        Enum(self.0.upgrade().expect("Failed to upgrade WeakEnum"))
    }
}
impl<'a, U> Clone for WeakEnum<'a, U> {
    fn clone(&self) -> Self {
        WeakEnum(self.0.clone())
    }
}
impl<'a, U> From<Enum<'a, U>> for WeakEnum<'a, U> {
    fn from(e: Enum<'a, U>) -> Self {
        e.downgrade()
    }
}
impl<'a, U> From<&Enum<'a, U>> for WeakEnum<'a, U> {
    fn from(e: &Enum<'a, U>) -> Self {
        e.downgrade()
    }
}
