#![allow(clippy::new_ret_no_self)]

use std::{cell::RefCell, rc::Rc};

use anyhow::bail;

use crate::{
    proto::{FieldDescriptor, Scalar, Syntax},
    Comments, Enum, Field, File, Files, FullyQualified, Message, Name, Node, Oneof, Package, Type,
    WeakEnum, WeakMessage, WeakOneof, WellKnownType,
};

use super::FieldDetail;
#[derive(Debug)]
pub(crate) struct OneofFieldDetail<'a, U> {
    pub detail: FieldDetail<'a, U>,
    pub oneof: WeakOneof<'a, U>,
}
impl<'a, U> OneofFieldDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.detail.descriptor()
    }
    pub fn util(&self) -> Rc<U> {
        self.detail.util()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.detail.message()
    }
    pub fn comments(&self) -> Comments<'a> {
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
    pub fn oneof(&self) -> Oneof<'a, U> {
        self.oneof.clone().into()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.detail.set_comments(comments);
    }

    pub fn is_marked_required(&self) -> bool {
        self.detail.is_marked_required()
    }

    pub fn is_in_real_oneof(&self) -> bool {
        self.oneof().is_real()
    }
    pub fn is_in_synthetic_oneof(&self) -> bool {
        self.oneof().is_synthetic()
    }
}
impl<'a, U> Clone for OneofFieldDetail<'a, U> {
    fn clone(&self) -> Self {
        Self {
            detail: self.detail.clone(),
            oneof: self.oneof.clone(),
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
    pub fn comments(&self) -> Comments<'a> {
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
    pub fn imports(&self) -> Files<'a, U> {
        match self {
            OneofField::Scalar(_) => Files::empty(),
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
            OneofField::Enum(f) => Some(f.r#enum()),
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
        matches!(self, OneofField::Scalar(_))
    }
    pub fn scalar(&self) -> Option<Scalar> {
        match self {
            OneofField::Scalar(f) => Some(f.scalar()),
            _ => None,
        }
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        match self {
            OneofField::Scalar(f) => f.set_comments(comments),
            OneofField::Enum(f) => f.set_comments(comments),
            OneofField::Embed(f) => f.set_comments(comments),
        }
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        match self {
            OneofField::Scalar(f) => f.descriptor(),
            OneofField::Enum(f) => f.descriptor(),
            OneofField::Embed(f) => f.descriptor(),
        }
    }

    pub fn syntax(&self) -> Syntax {
        match self {
            OneofField::Scalar(f) => f.syntax(),
            OneofField::Enum(f) => f.syntax(),
            OneofField::Embed(f) => f.syntax(),
        }
    }

    pub fn is_marked_required(&self) -> bool {
        match self {
            OneofField::Scalar(f) => f.is_marked_required(),
            OneofField::Enum(f) => f.is_marked_required(),
            OneofField::Embed(f) => f.is_marked_required(),
        }
    }

    pub fn is_embed(&self) -> bool {
        matches!(self, OneofField::Embed(_))
    }

    pub fn is_in_real_oneof(&self) -> bool {
        match self {
            OneofField::Scalar(f) => f.is_in_real_oneof(),
            OneofField::Enum(f) => f.is_in_real_oneof(),
            OneofField::Embed(f) => f.is_in_real_oneof(),
        }
    }

    pub fn is_in_synthetic_oneof(&self) -> bool {
        match self {
            OneofField::Scalar(f) => f.is_in_synthetic_oneof(),
            OneofField::Enum(f) => f.is_in_synthetic_oneof(),
            OneofField::Embed(f) => f.is_in_synthetic_oneof(),
        }
    }

    pub fn is_well_known_type(&self) -> bool {
        match self {
            OneofField::Enum(f) => f.is_well_known_type(),
            OneofField::Embed(f) => f.is_well_known_type(),
            OneofField::Scalar(_) => false,
        }
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        match self {
            OneofField::Enum(f) => f.well_known_type(),
            OneofField::Embed(f) => f.well_known_type(),
            OneofField::Scalar(_) => None,
        }
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, OneofField::Enum(_))
    }

    pub fn has_presence(&self) -> bool {
        match self {
            OneofField::Scalar(f) => f.has_presence(),
            OneofField::Enum(f) => f.has_presence(),
            OneofField::Embed(f) => f.has_presence(),
        }
    }

    pub fn util(&self) -> Rc<U> {
        match self {
            OneofField::Scalar(f) => f.util(),
            OneofField::Enum(f) => f.util(),
            OneofField::Embed(f) => f.util(),
        }
    }

    pub(crate) fn set_value(&self, value: crate::Node<'a, U>) -> Result<(), anyhow::Error> {
        match self {
            OneofField::Enum(f) => f.set_value(value),
            OneofField::Embed(f) => f.set_value(value),
            _ => unreachable!(),
        }
    }

    pub fn value_type(&self) -> Type<'a> {
        self.descriptor().proto_type()
    }

    pub(crate) fn new(
        detail: FieldDetail<'a, U>,
        oneof: Oneof<'a, U>,
    ) -> Result<crate::Field<'a, U>, anyhow::Error> {
        match detail.value_type() {
            Type::Scalar(scalar) => Ok(Field::Oneof(OneofField::Scalar(OneofScalarField(
                Rc::new(OneofScalarFieldDetail {
                    scalar,
                    detail: OneofFieldDetail {
                        detail,
                        oneof: oneof.into(),
                    },
                }),
            )))),
            Type::Enum(_) => Ok(Field::Oneof(OneofField::Enum(OneofEnumField(Rc::new(
                OneofEnumFieldDetail {
                    detail: OneofFieldDetail {
                        detail,
                        oneof: oneof.into(),
                    },
                    enumeration: RefCell::new(WeakEnum::empty()),
                },
            ))))),
            Type::Message(_) => Ok(Field::Oneof(OneofField::Embed(OneofEmbedField(Rc::new(
                OneofEmbedFieldDetail {
                    detail: OneofFieldDetail {
                        detail,
                        oneof: oneof.into(),
                    },
                    embed: RefCell::new(WeakMessage::empty()),
                },
            ))))),
            Type::Group => bail!("Group is not supported. Use an embedded Message instead."),
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
pub(crate) struct OneofEnumFieldDetail<'a, U> {
    detail: OneofFieldDetail<'a, U>,
    enumeration: RefCell<WeakEnum<'a, U>>,
}
#[derive(Debug)]
pub struct OneofEnumField<'a, U>(Rc<OneofEnumFieldDetail<'a, U>>);

impl<'a, U> OneofEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn util(&self) -> Rc<U> {
        self.0.detail.util()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }
    pub fn r#enum(&self) -> Enum<'a, U> {
        self.0.enumeration.borrow().clone().into()
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

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }
    pub fn imports(&self) -> Files<'a, U> {
        if self.has_import() {
            Files::from(self.0.enumeration.borrow().weak_file())
        } else {
            Files::empty()
        }
    }

    pub fn has_import(&self) -> bool {
        self.enumeration().file() != self.file()
    }

    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }

    pub fn is_in_real_oneof(&self) -> bool {
        self.0.detail.is_in_real_oneof()
    }

    pub fn is_in_synthetic_oneof(&self) -> bool {
        self.0.detail.is_in_synthetic_oneof()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.enumeration().is_well_known_type()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.r#enum().well_known_type()
    }

    pub fn has_presence(&self) -> bool {
        true
    }

    fn set_value(&self, value: crate::Node<'a, U>) -> Result<(), anyhow::Error> {
        match value {
            Node::Enum(v) => {
                self.0.enumeration.replace(v.into());
                Ok(())
            }
            _ => bail!("expected Enum, received {}", value),
        }
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
    pub fn oneof(&self) -> Oneof<'a, U> {
        self.0.detail.oneof()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }

    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }

    pub fn comments(&self) -> Comments<'a> {
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
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }

    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }

    pub fn is_in_real_oneof(&self) -> bool {
        self.0.detail.is_in_real_oneof()
    }

    pub fn is_in_synthetic_oneof(&self) -> bool {
        self.0.detail.is_in_synthetic_oneof()
    }

    pub fn has_presence(&self) -> bool {
        true
    }

    pub fn util(&self) -> Rc<U> {
        self.0.detail.util()
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
pub(crate) struct OneofEmbedFieldDetail<'a, U> {
    detail: OneofFieldDetail<'a, U>,
    embed: RefCell<WeakMessage<'a, U>>,
}

#[derive(Debug)]
pub struct OneofEmbedField<'a, U>(Rc<OneofEmbedFieldDetail<'a, U>>);
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

    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }
    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
    }

    pub fn embed(&self) -> Message<'a, U> {
        self.0.embed.borrow().clone().into()
    }
    pub fn has_import(&self) -> bool {
        self.0.embed.borrow().file() != self.file()
    }
    pub fn imports(&self) -> Files<'a, U> {
        if self.has_import() {
            Files::from(self.0.embed.borrow().weak_file())
        } else {
            Files::empty()
        }
    }
    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }

    fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }

    pub fn is_in_real_oneof(&self) -> bool {
        self.0.detail.is_in_real_oneof()
    }
    pub fn is_in_synthetic_oneof(&self) -> bool {
        self.0.detail.is_in_synthetic_oneof()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.embed().is_well_known_type()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.embed().well_known_type()
    }

    pub fn has_presence(&self) -> bool {
        true
    }

    pub fn util(&self) -> Rc<U> {
        self.0.detail.util()
    }

    fn set_value(&self, value: crate::Node<'a, U>) -> Result<(), anyhow::Error> {
        match value {
            Node::Message(v) => {
                self.0.embed.replace(v.into());
                Ok(())
            }
            _ => bail!("expected Enum, received {}", value),
        }
    }
}

impl<'a, U> FullyQualified for OneofEmbedField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}
