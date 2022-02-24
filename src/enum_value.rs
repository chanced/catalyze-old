use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{format_fqn, iter::Iter, visit::Visitor, Enum, FullyQualified, Name, Node, NodeAtPath};
use prost_types::EnumValueDescriptorProto;

pub(crate) type EnumValueList<'a, U> = Rc<RefCell<Vec<Rc<EnumValue<'a, U>>>>>;

#[derive(Debug, Clone)]
pub struct EnumValue<'a, U> {
    pub name: Name<U>,
    fqn: String,
    pub descriptor: &'a EnumValueDescriptorProto,
    pub(crate) container: Weak<Enum<'a, U>>,
}

impl<'a, U> EnumValue<'a, U> {
    pub(crate) fn new(
        desc: &'a EnumValueDescriptorProto,
        enm: Rc<Enum<'a, U>>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        Rc::new(EnumValue {
            name: Name::new(desc.name(), util),
            fqn: format_fqn(enm.as_ref(), desc.name()),
            descriptor: desc,
            container: Rc::downgrade(&enm),
        })
    }
    pub fn name(&self) -> Name<U> {
        self.name.clone()
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

impl<'a, U> FullyQualified for EnumValue<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
}
