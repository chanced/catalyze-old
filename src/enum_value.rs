use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    iter::Iter,
    visit::{Accept, Visitor},
    Enum, Name, Node, NodeAtPath,
};
use prost_types::EnumValueDescriptorProto;

pub(crate) type EnumValueList<'a, U> = Rc<RefCell<Vec<Rc<EnumValue<'a, U>>>>>;

#[derive(Debug, Clone)]
pub struct EnumValue<'a, U> {
    pub name: Name<U>,
    pub fully_qualified_name: String,
    pub descriptor: &'a EnumValueDescriptorProto,
    pub(crate) container: Weak<Enum<'a, U>>,
}

impl<'a, U> EnumValue<'a, U> {
    pub fn new(
        desc: &'a EnumValueDescriptorProto,
        container: Rc<Enum<'a, U>>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let fully_qualified_name = format!("{}.{}", container.fully_qualified_name, desc.name());
        Rc::new(EnumValue {
            name: Name::new(desc.name(), util),
            fully_qualified_name,
            descriptor: desc,
            container: Rc::downgrade(&container),
        })
    }

    /// Returns the enum that contains this enum value.
    pub fn containing_enum(&self) -> Rc<Enum<'a, U>> {
        self.container.upgrade().unwrap()
    }
    /// Alias for `containing_enum`.
    ///
    /// Returns the enum that contains this enum value.
    pub fn container(&self) -> Rc<Enum<'a, U>> {
        self.containing_enum()
    }
}

impl<'a, U> NodeAtPath<'a, U> for Rc<EnumValue<'a, U>> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            Some(Node::EnumValue(self.clone()))
        } else {
            None
        }
    }
}

impl<'a, U, V: Visitor<'a, U>> Accept<'a, U, V> for Rc<EnumValue<'a, U>> {
    fn accept(&self, v: &mut V) -> Result<(), V::Error> {
        if v.done() {
            return Ok(());
        }
        v.visit_enum_value(self.clone())
    }
}
