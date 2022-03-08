use std::rc::{Rc, Weak};

use crate::{
    proto::{FieldDescriptor, Scalar, Syntax},
    Comments, Enum, File, FullyQualified, Message, Name, Oneof, Package, WeakEnum, WeakMessage,
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
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.detail.descriptor()
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
    pub fn syntax(&self) -> Syntax {
        self.detail.syntax()
    }
    pub fn file(&self) -> File<'a, U> {
        self.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.detail.package()
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
    Embed(OneofEmbedField<'a, U>),
}
impl<'a, U> Clone for OneofField<'a, U> {
    fn clone(&self) -> Self {
        match self {
            Self::Scalar(f) => Self::Scalar(f.clone()),
            Self::Enum(f) => Self::Enum(f.clone()),
            Self::Embed(f) => Self::Embed(f.clone()),
        }
    }
}
impl<'a, U> OneofField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            OneofField::Scalar(f) => f.name(),
            OneofField::Enum(f) => f.name(),
            OneofField::Embed(f) => f.name(),
        }
    }
    pub fn fully_qualified_name(&self) -> String {
        match self {
            OneofField::Scalar(f) => f.fully_qualified_name(),
            OneofField::Enum(f) => f.fully_qualified_name(),
            OneofField::Embed(f) => f.fully_qualified_name(),
        }
    }
    pub fn message(&self) -> Message<'a, U> {
        match self {
            OneofField::Scalar(f) => f.message(),
            OneofField::Enum(f) => f.message(),
            OneofField::Embed(f) => f.message(),
        }
    }
    pub fn comments(&self) -> Comments<'a, U> {
        match self {
            OneofField::Scalar(f) => f.comments(),
            OneofField::Enum(f) => f.comments(),
            OneofField::Embed(f) => f.comments(),
        }
    }

    pub fn file(&self) -> File<'a, U> {
        match self {
            OneofField::Scalar(f) => f.file(),
            OneofField::Enum(f) => f.file(),
            OneofField::Embed(f) => f.file(),
        }
    }
    pub fn package(&self) -> Package<'a, U> {
        match self {
            OneofField::Scalar(f) => f.package(),
            OneofField::Enum(f) => f.package(),
            OneofField::Embed(f) => f.package(),
        }
    }

    pub fn has_import(&self) -> bool {
        match self {
            OneofField::Scalar(_) => false,
            OneofField::Enum(f) => f.has_import(),
            OneofField::Embed(f) => f.has_import(),
        }
    }
    pub fn imports(&self) -> Option<File<'a, U>> {
        match self {
            OneofField::Scalar(_) => None,
            OneofField::Enum(f) => f.imports(),
            OneofField::Embed(f) => f.imports(),
        }
    }

    pub fn build_target(&self) -> bool {
        match self {
            OneofField::Scalar(f) => f.build_target(),
            OneofField::Enum(f) => f.build_target(),
            OneofField::Embed(f) => f.build_target(),
        }
    }
    pub fn r#enum(&self) -> Option<Enum<'a, U>> {
        match self {
            _ => None,
        }
    }
    pub fn enumeration(&self) -> Option<Enum<'a, U>> {
        self.r#enum()
    }
    pub fn embed(&self) -> Option<Message<'a, U>> {
        match self {
            OneofField::Embed(f) => Some(f.embed()),
            _ => None,
        }
    }
    pub fn is_scalar(&self) -> bool {
        match self {
            OneofField::Scalar(_) => true,
            _ => false,
        }
    }
    pub fn scalar(&self) -> Option<Scalar> {
        match self {
            OneofField::Scalar(f) => Some(f.scalar()),
            _ => None,
        }
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        match self {
            OneofField::Scalar(f) => f.set_comments(comments),
            OneofField::Enum(f) => f.set_comments(comments),
            OneofField::Embed(f) => f.set_comments(comments),
        }
    }

    pub fn descriptor(&self) -> crate::proto::FieldDescriptor {
        match self {
            OneofField::Scalar(f) => f.descriptor(),
            OneofField::Enum(f) => f.descriptor(),
            OneofField::Embed(f) => f.descriptor(),
        }
    }

    pub fn syntax(&self) -> crate::proto::Syntax {
        match self {
            OneofField::Scalar(f) => f.syntax(),
            OneofField::Enum(f) => f.syntax(),
            OneofField::Embed(f) => f.syntax(),
        }
    }
}

impl<'a, U> FullyQualified for OneofField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            OneofField::Scalar(f) => f.fully_qualified_name(),
            OneofField::Enum(f) => f.fully_qualified_name(),
            OneofField::Embed(f) => f.fully_qualified_name(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OneofEnumFieldDetail<'a, U> {
    detail: OneofFieldDetail<'a, U>,
    e: WeakEnum<'a, U>,
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
    pub fn r#enum(&self) -> Enum<'a, U> {
        self.0.e.into()
    }
    pub fn enumeration(&self) -> Enum<'a, U> {
        self.r#enum()
    }
    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }
    pub fn imports(&self) -> Option<File<'a, U>> {
        if self.has_import() {
            Some(self.0.e.file())
        } else {
            None
        }
    }
    pub fn has_import(&self) -> bool {
        self.0.e.file() != self.file()
    }

    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }

    pub fn descriptor(&self) -> FieldDescriptor {
        self.0.detail.descriptor()
    }

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
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

    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
    }
    pub fn scalar(&self) -> Scalar {
        self.0.scalar
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }

    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }

    pub fn descriptor(&self) -> crate::proto::FieldDescriptor {
        self.0.detail.descriptor()
    }

    pub fn syntax(&self) -> crate::proto::Syntax {
        self.0.detail.syntax()
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
    embed: WeakMessage<'a, U>,
}

#[derive(Debug)]
pub struct OneofEmbedField<'a, U>(Rc<OneofMessageFieldDetail<'a, U>>);
impl<'a, U> Clone for OneofEmbedField<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<'a, U> OneofEmbedField<'a, U> {
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
    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
    }

    pub fn embed(&self) -> Message<'a, U> {
        self.0.embed.into()
    }
    pub fn has_import(&self) -> bool {
        self.0.embed.file() != self.file()
    }
    pub fn imports(&self) -> Option<File<'a, U>> {
        if self.has_import() {
            Some(self.0.embed.file())
        } else {
            None
        }
    }
    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }

    pub fn descriptor(&self) -> FieldDescriptor {
        self.0.detail.descriptor()
    }

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
}

impl<'a, U> FullyQualified for OneofEmbedField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}
