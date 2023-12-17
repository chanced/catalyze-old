use std::{cell::RefCell, rc::Rc};

use protobuf::reflect::EnumValueDescriptor;

use crate::{container::Container, Comments, Enum, File, Node, Package, WeakEnum};

#[derive(Debug, Clone)]
struct EnumValueDetail {
    fqn: String,
    descriptor: EnumValueDescriptor,
    e: WeakEnum,
    comments: RefCell<Comments>,
}

impl EnumValueDetail {
    pub fn name(&self) -> &str {
        &self.descriptor.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        &self.fqn
    }
    pub fn descriptor(&self) -> EnumValueDescriptor {
        self.descriptor
    }

    pub fn into_enum(&self) -> Enum {
        self.e.clone().into()
    }

    pub fn comments(&self) -> Comments {
        *self.comments.borrow()
    }
    pub(crate) fn set_comments(&self, comments: Comments) {
        self.comments.replace(comments);
    }
}

#[derive(Debug, Clone)]
pub struct EnumValue(Rc<EnumValueDetail>);

impl EnumValue {
    pub(crate) fn new(desc: EnumValueDescriptor, e: Enum) -> Self {
        let fqn = format!("{}.{}", e.fully_qualified_name(), desc.name());
        EnumValue(Rc::new(EnumValueDetail {
            fqn,
            descriptor: desc,
            e: e.clone().into(),
            comments: RefCell::new(Comments::default()),
        }))
    }
    pub fn name(&self) -> &str {
        self.0.name()
    }

    pub fn descriptor(&self) -> EnumValueDescriptor {
        self.0.descriptor()
    }
    /// Returns the `Enum` that contains this value.
    pub fn enum_(&self) -> Enum {
        self.0.into_enum()
    }
    pub fn container(&self) -> Container {
        self.enum_().container()
    }

    pub fn file(&self) -> File {
        self.enum_().file()
    }

    pub fn package(&self) -> Package {
        self.enum_().package()
    }
    pub fn number(&self) -> i32 {
        self.descriptor().number()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.0.fully_qualified_name()
    }
    pub fn comments(&self) -> Comments {
        self.0.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments) {
        self.0.set_comments(comments);
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node> {
        if path.is_empty() {
            Some(self.to_owned().into())
        } else {
            None
        }
    }
}

impl PartialEq for EnumValue {
    fn eq(&self, other: &Self) -> bool {
        self.fully_qualified_name() == other.fully_qualified_name()
    }
}
impl PartialEq<i32> for EnumValue {
    fn eq(&self, other: &i32) -> bool {
        self.number() == *other
    }
}
impl PartialEq<EnumValue> for i32 {
    fn eq(&self, other: &EnumValue) -> bool {
        self == &other.number()
    }
}
impl PartialEq<str> for EnumValue {
    fn eq(&self, other: &str) -> bool {
        self.fully_qualified_name() == other
    }
}
impl PartialEq<EnumValue> for str {
    fn eq(&self, other: &EnumValue) -> bool {
        self == other.fully_qualified_name()
    }
}
