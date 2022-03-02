use std::{rc::Rc};

use crate::{format_fqn, Enum, FullyQualified, Name, Node, NodeAtPath, WeakEnum};
use prost_types::EnumValueDescriptorProto;

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
    pub(crate) fn new(desc: &'a EnumValueDescriptorProto, enumeration: Enum<'a, U>) -> Self {
        EnumValue(Rc::new(EnumValueDetail {
            name: Name::new(desc.name(), enumeration.util()),
            fqn: format_fqn(&enumeration, desc.name()),
            descriptor: desc,
            r#enum: enumeration.into(),
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
    /// alias for `r#enum`.
    pub fn enumeration(&self) -> Enum<'a, U> {
        self.r#enum()
    }
    /// Returns the `Enum` that contains this value.
    pub fn r#enum(&self) -> Enum<'a, U> {
        self.0.r#enum.clone().into()
    }

    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}

impl<'a, U> FullyQualified for EnumValue<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}

impl<'a, U> Clone for EnumValue<'a, U> {
    fn clone(&self) -> Self {
        EnumValue(self.0.clone())
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
