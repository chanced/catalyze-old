use std::{cell::RefCell, rc::Rc};

use crate::{
    container::Container, format_fqn, proto::EnumValueDescriptor, Comments, Enum, File,
    FullyQualified, Name, Node, NodeAtPath, Nodes, Package, WeakEnum,
};

#[derive(Debug, Clone)]
struct EnumValueDetail<'a, U> {
    name: Name<U>,
    fqn: String,
    desc: EnumValueDescriptor<'a>,
    e: WeakEnum<'a, U>,
    comments: RefCell<Comments<'a, U>>,
    util: RefCell<Rc<U>>,
}

impl<'a, U> EnumValueDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.name.clone()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
    pub fn descriptor(&self) -> EnumValueDescriptor<'a> {
        self.desc
    }

    pub fn enumeration(&self) -> Enum<'a, U> {
        self.e.clone().into()
    }

    pub fn comments(&self) -> Comments<'a, U> {
        *self.comments.borrow()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.comments.replace(comments);
    }
}

#[derive(Debug)]
pub struct EnumValue<'a, U>(Rc<EnumValueDetail<'a, U>>);

impl<'a, U> EnumValue<'a, U> {
    pub(crate) fn new(desc: EnumValueDescriptor<'a>, e: Enum<'a, U>) -> Self {
        EnumValue(Rc::new(EnumValueDetail {
            name: Name::new(desc.name(), e.util()),
            fqn: format_fqn(&e, desc.name()),
            desc,
            e: e.clone().into(),
            comments: RefCell::new(Comments::default()),
            util: RefCell::new(e.util()),
        }))
    }
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }

    /// Alias for `r#enum`.
    ///
    /// Returns the enum that contains this enum value.
    pub fn enumeration(&self) -> Enum<'a, U> {
        self.r#enum()
    }

    /// Returns the `Enum` that contains this value.
    pub fn r#enum(&self) -> Enum<'a, U> {
        self.0.e.clone().into()
    }
    pub fn container(&self) -> Container<'a, U> {
        self.enumeration().container()
    }

    pub fn file(&self) -> File<'a, U> {
        self.enumeration().file()
    }

    pub fn package(&self) -> Package<'a, U> {
        self.enumeration().package()
    }

    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        self.0.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.set_comments(comments);
    }
    pub fn util(&self) -> Rc<U> {
        self.0.util.borrow().clone()
    }
    pub(crate) fn nodes(&self) -> Nodes<'a, U> {
        Nodes::empty()
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

#[cfg(test)]
impl<'a> Default for EnumValue<'a, crate::util::Generic> {
    fn default() -> Self {
        let e = Enum::default();
        EnumValue(Rc::new(EnumValueDetail {
            name: Name::new("", e.util()),
            fqn: "".to_string(),
            desc: EnumValueDescriptor::default(),
            e: e.clone().into(),
            comments: RefCell::new(Comments::default()),
            util: RefCell::new(Rc::new(crate::util::Generic::default())),
        }))
    }
}
