use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
};

use prost_types::EnumDescriptorProto;

use crate::{
    container::{Container, WeakContainer},
    iter::Iter,
    proto::EnumDescriptorPath,
    traits::{Downgrade, Upgrade},
    EnumValue, EnumValueList, FullyQualified, Message, Name, Node, NodeAtPath, Package,
};

pub(crate) type EnumList<'a, U> = Rc<RefCell<Vec<Enum<'a, U>>>>;

#[derive(Debug, Clone)]
struct EnumDetail<'a, U> {
    name: Name<U>,
    fqn: String,
    values: EnumValueList<'a, U>,
    container: WeakContainer<'a, U>,
    dependents: Rc<RefCell<Vec<Weak<Message<'a, U>>>>>,
}

#[derive(Debug)]
pub struct Enum<'a, U>(Rc<EnumDetail<'a, U>>);

impl<'a, U> Enum<'a, U> {
    pub(crate) fn new(
        desc: &'a EnumDescriptorProto,
        container: Container<'a, U>,
        util: Rc<U>,
    ) -> Self {
        let fully_qualified_name = format!("{}.{}", container.fully_qualified_name(), desc.name());

        let e = Enum(Rc::new(EnumDetail {
            name: Name::new(desc.name(), util.clone()),
            values: Rc::new(RefCell::new(Vec::with_capacity(desc.value.len()))),
            container: container.downgrade(),
            dependents: Rc::new(RefCell::new(Vec::default())),
            fqn: fully_qualified_name,
        }));
        let mut values = e.0.values.borrow_mut();
        for v in &desc.value {
            values.push(EnumValue::new(v, e.clone(), util.clone()));
        }
        e
    }

    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }

    pub fn values(&self) -> Iter<EnumValue<'a, U>> {
        Iter::from(&self.0.values)
    }
    pub fn package(&self) -> Option<Package<'a, U>> {
        self.0.container.package()
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
                EnumDescriptorPath::Value => self
                    .0
                    .values
                    .borrow()
                    .get(next)
                    .cloned()
                    .map(Node::EnumValue),
                _ => None,
            })
    }
}

impl<'a, U> Downgrade for Enum<'a, U> {
    type Output = WeakEnum<'a, U>;
    fn downgrade(self) -> Self::Output {
        WeakEnum(Rc::downgrade(&self.0))
    }
}

impl<'a, U> FullyQualified for Enum<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
    }
}

#[derive(Debug)]
pub(crate) struct WeakEnum<'a, U>(Weak<EnumDetail<'a, U>>);

impl<'a, U> Upgrade for WeakEnum<'a, U> {
    type Output = Enum<'a, U>;
    fn upgrade(self) -> Self::Output {
        Enum(self.0.upgrade().expect("Enum was dropped"))
    }
}
impl<'a, U> Clone for WeakEnum<'a, U> {
    fn clone(&self) -> Self {
        WeakEnum(self.0.clone())
    }
}
