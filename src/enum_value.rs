use std::{cell::RefCell, rc::Rc};

use crate::{
    container::Container, proto::EnumValueDescriptor, Comments, Enum, File, Name, Node, Package,
    WeakEnum,
};

#[derive(Debug, Clone)]
struct EnumValueDetail<'a> {
    name: Name,
    fqn: String,
    desc: EnumValueDescriptor<'a>,
    e: WeakEnum<'a>,
    comments: RefCell<Comments<'a>>,
}

impl<'a> EnumValueDetail<'a> {
    pub fn name(&self) -> &Name {
        &self.name
    }
    pub fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
    pub fn descriptor(&self) -> EnumValueDescriptor<'a> {
        self.desc
    }

    pub fn enumeration(&self) -> Enum<'a> {
        self.e.clone().into()
    }

    pub fn comments(&self) -> Comments<'a> {
        *self.comments.borrow()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.comments.replace(comments);
    }
}

#[derive(Debug, Clone)]
pub struct EnumValue<'a>(Rc<EnumValueDetail<'a>>);

impl<'a> EnumValue<'a> {
    pub(crate) fn new(desc: EnumValueDescriptor<'a>, e: Enum<'a>) -> Self {
        let fqn = format!("{}.{}", e.fully_qualified_name(), desc.name());
        EnumValue(Rc::new(EnumValueDetail {
            fqn,
            name: desc.name().into(),
            desc,
            e: e.clone().into(),
            comments: RefCell::new(Comments::default()),
        }))
    }
    pub fn name(&self) -> &Name {
        self.0.name()
    }

    /// Alias for `r#enum`.
    ///
    /// Returns the enum that contains this enum value.
    pub fn enumeration(&self) -> Enum<'a> {
        self.r#enum()
    }

    pub fn descriptor(&self) -> EnumValueDescriptor<'a> {
        self.0.descriptor()
    }
    /// Returns the `Enum` that contains this value.
    pub fn r#enum(&self) -> Enum<'a> {
        self.0.enumeration()
    }
    pub fn container(&self) -> Container<'a> {
        self.enumeration().container()
    }

    pub fn file(&self) -> File<'a> {
        self.enumeration().file()
    }

    pub fn package(&self) -> Package<'a> {
        self.enumeration().package()
    }
    pub fn number(&self) -> i32 {
        self.descriptor().number()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.fully_qualified_name()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.set_comments(comments);
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a>> {
        if path.is_empty() {
            Some(self.to_owned().into())
        } else {
            None
        }
    }
}

impl<'a> PartialEq for EnumValue<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.fully_qualified_name() == other.fully_qualified_name()
    }
}
impl<'a> PartialEq<i32> for EnumValue<'a> {
    fn eq(&self, other: &i32) -> bool {
        self.number() == *other
    }
}
impl<'a> PartialEq<EnumValue<'a>> for i32 {
    fn eq(&self, other: &EnumValue<'a>) -> bool {
        self == &other.number()
    }
}
impl<'a> PartialEq<str> for EnumValue<'a> {
    fn eq(&self, other: &str) -> bool {
        self.fully_qualified_name() == other
    }
}
impl<'a> PartialEq<EnumValue<'a>> for str {
    fn eq(&self, other: &EnumValue<'a>) -> bool {
        self == other.fully_qualified_name()
    }
}
