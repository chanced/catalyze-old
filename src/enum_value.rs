use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{Enum, Name, Node, NodeAtPath};
use prost_types::EnumValueDescriptorProto;

pub(crate) type EnumValueList<U> = Rc<RefCell<Vec<Rc<EnumValue<U>>>>>;

#[derive(Debug, Clone)]
pub struct EnumValue<U> {
    pub name: Name<U>,
    pub fully_qualified_name: String,
    pub descriptor: EnumValueDescriptorProto,
    pub(crate) container: Weak<Enum<U>>,
}

impl<U> EnumValue<U> {
    pub fn new(
        desc: EnumValueDescriptorProto,
        container: Rc<Enum<U>>,
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
    pub fn containing_enum(&self) -> Rc<Enum<U>> {
        self.container.upgrade().unwrap()
    }
    /// Alias for `containing_enum`.
    ///
    /// Returns the enum that contains this enum value.
    pub fn container(&self) -> Rc<Enum<U>> {
        self.containing_enum()
    }
}

impl<U> NodeAtPath<U> for Rc<EnumValue<U>> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<U>> {
        if path.is_empty() {
            Some(Node::EnumValue(self.clone()))
        } else {
            None
        }
    }
}
