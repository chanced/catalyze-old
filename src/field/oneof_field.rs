use std::rc::{Rc, Weak};

use crate::{
    proto::Scalar, Comments, FullyQualified, Message, Name, Oneof, ScalarField, WeakMessage,
};

use super::FieldDetail;
#[derive(Debug)]
pub(crate) struct OneofFieldDetail<'a, U> {
    pub detail: FieldDetail<'a, U>,
    pub oneof: Weak<Oneof<'a, U>>,
    pub is_synthetic: bool,
}
impl<'a, U> OneofFieldDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.detail.message()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        self.detail.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.detail.set_comments(comments);
    }
}
impl<'a, U> Clone for OneofFieldDetail<'a, U> {
    fn clone(&self) -> Self {
        Self {
            detail: self.detail.clone(),
            oneof: self.oneof.clone(),
            is_synthetic: self.is_synthetic,
        }
    }
}

#[derive(Debug)]
pub enum OneofField<'a, U> {
    Scalar(OneofScalarField<'a, U>),
    Enum(OneofEnumField<'a, U>),
    Message(OneofMessageField<'a, U>),
}
impl<'a, U> Clone for OneofField<'a, U> {
    fn clone(&self) -> Self {
        match self {
            Self::Scalar(f) => Self::Scalar(f.clone()),
            Self::Enum(f) => Self::Enum(f.clone()),
            Self::Message(f) => Self::Message(f.clone()),
        }
    }
}
impl<'a, U> OneofField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            OneofField::Scalar(f) => f.name(),
            OneofField::Enum(f) => f.name(),
            OneofField::Message(f) => f.name(),
        }
    }
    pub fn fully_qualified_name(&self) -> String {
        match self {
            OneofField::Scalar(f) => f.fully_qualified_name(),
            OneofField::Enum(f) => f.fully_qualified_name(),
            OneofField::Message(f) => f.fully_qualified_name(),
        }
    }
    pub fn message(&self) -> Message<'a, U> {
        match self {
            OneofField::Scalar(f) => f.message(),
            OneofField::Enum(f) => f.message(),
            OneofField::Message(f) => f.message(),
        }
    }
    pub fn comments(&self) -> Comments<'a, U> {
        match self {
            OneofField::Scalar(f) => f.comments(),
            OneofField::Enum(f) => f.comments(),
            OneofField::Message(f) => f.comments(),
        }
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        match self {
            OneofField::Scalar(f) => f.set_comments(comments),
            OneofField::Enum(f) => f.set_comments(comments),
            OneofField::Message(f) => f.set_comments(comments),
        }
    }
}

impl<'a, U> FullyQualified for OneofField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            OneofField::Scalar(f) => f.fully_qualified_name(),
            OneofField::Enum(f) => f.fully_qualified_name(),
            OneofField::Message(f) => f.fully_qualified_name(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OneofEnumFieldDetail<'a, U> {
    detail: OneofFieldDetail<'a, U>,
    scalar: ScalarField<'a, U>,
}
#[derive(Debug)]
pub struct OneofEnumField<'a, U>(Rc<OneofEnumFieldDetail<'a, U>>);

impl<'a, U> OneofEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        self.0.detail.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }
}
impl<'a, U> FullyQualified for OneofEnumField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}
impl<'a, U> Clone for OneofEnumField<'a, U> {
    fn clone(&self) -> Self {
        OneofEnumField(self.0.clone())
    }
}

#[derive(Debug, Clone)]
pub struct OneofScalarFieldDetail<'a, U> {
    scalar: Scalar,
    detail: OneofFieldDetail<'a, U>,
}
#[derive(Debug)]
pub struct OneofScalarField<'a, U>(Rc<OneofScalarFieldDetail<'a, U>>);

impl<'a, U> OneofScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }

    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }

    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }

    pub fn comments(&self) -> Comments<'a, U> {
        self.0.detail.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }
}

impl<'a, U> FullyQualified for OneofScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}

impl<'a, U> Clone for OneofScalarField<'a, U> {
    fn clone(&self) -> Self {
        OneofScalarField(self.0.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OneofMessageFieldDetail<'a, U> {
    detail: OneofFieldDetail<'a, U>,
    message: WeakMessage<'a, U>,
}

#[derive(Debug)]
pub struct OneofMessageField<'a, U>(Rc<OneofMessageFieldDetail<'a, U>>);
impl<'a, U> Clone for OneofMessageField<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<'a, U> OneofMessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        self.0.detail.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }
}

impl<'a, U> FullyQualified for OneofMessageField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}
