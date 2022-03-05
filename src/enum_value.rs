use std::{cell::RefCell, rc::Rc};

use crate::{
    format_fqn, proto::EnumValueDescriptor, Comments, Enum, FullyQualified, Name, Node, NodeAtPath,
    WeakEnum,
};

#[derive(Debug, Clone)]
struct EnumValueDetail<'a, U> {
    name: Name<U>,
    fqn: String,
    desc: dyn EnumValueDescriptor<'a, U>,
    e: WeakEnum<'a, U>,
    comments: RefCell<Comments<'a, U>>,
}

impl<'a, U> EnumValueDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.name.clone()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
    pub fn descriptor(&self) -> dyn EnumValueDescriptor<'a, U> {
        self.desc
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
    pub(crate) fn new(desc: dyn EnumValueDescriptor<'a, U>, e: Enum<'a, U>) -> Self {
        EnumValue(Rc::new(EnumValueDetail {
            name: Name::new(desc.name(), e.util()),
            fqn: format_fqn(&e, desc.name()),
            desc,
            e: e.into(),
            comments: RefCell::new(Comments::default()),
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

    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        self.0.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.set_comments(comments);
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
