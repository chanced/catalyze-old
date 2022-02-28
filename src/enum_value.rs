use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::{
    format_fqn,
    traits::{Downgrade, Upgrade},
    Enum, FullyQualified, Name, Named, Node, NodeAtPath, WeakEnum,
};
use prost_types::EnumValueDescriptorProto;

pub(crate) type EnumValueList<'a, U> = Rc<RefCell<Vec<EnumValue<'a, U>>>>;

#[derive(Debug, Clone)]
struct EnumValueDetail<'a, U> {
    pub name: Name<U>,
    fqn: String,
    pub descriptor: &'a EnumValueDescriptorProto,
    r#enum: WeakEnum<'a, U>,
}

#[derive(Debug)]
pub struct EnumValue<'a, U>(Rc<EnumValueDetail<'a, U>>);

impl<'a, U> EnumValue<'a, U> {
    pub(crate) fn new(
        desc: &'a EnumValueDescriptorProto,
        r#enum: Enum<'a, U>,
        util: Rc<RefCell<U>>,
    ) -> Self {
        EnumValue(Rc::new(EnumValueDetail {
            name: Name::new(desc.name(), util),
            fqn: format_fqn(&r#enum, desc.name()),
            descriptor: desc,
            r#enum: r#enum.downgrade(),
        }))
    }
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }

    /// Alias for `r#enum`.
    ///
    /// Returns the enum that contains this enum value.
    pub fn container(&self) -> Enum<'a, U> {
        self.r#enum()
    }

    /// Returns the `Enum` that contains this value.
    pub fn r#enum(&self) -> Enum<'a, U> {
        self.0.r#enum.upgrade()
    }

    fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
    }
}

impl<'a, U> Named<U> for EnumValue<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
}

impl<'a, U> FullyQualified for EnumValue<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
    }
}

impl<'a, U> Clone for EnumValue<'a, U> {
    fn clone(&self) -> Self {
        EnumValue(self.0.clone())
    }
}

impl<'a, U> Into<Node<'a, U>> for EnumValue<'a, U> {
    fn into(self) -> Node<'a, U> {
        Node::EnumValue(self)
    }
}

impl<'a, U> NodeAtPath<'a, U> for EnumValue<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            Some(self.to_owned().into())
        } else {
            None
        }
    }
}
