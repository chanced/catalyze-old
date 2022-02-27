use std::rc::{Rc, Weak};

use crate::{
    name::Named,
    traits::{Downgrade, Upgrade},
    Field, FullyQualified, Message, Name, Oneof, WeakMessage,
};

use super::{FieldDetail, MessageField, ScalarField};

#[derive(Debug, Clone)]
pub enum OneofField<'a, U> {
    Real(RealOneofField<'a, U>),
    Synethic(SyntheticOneofField<'a, U>),
}

impl<'a, U> OneofField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            OneofField::Real(f) => f.name(),
            OneofField::Synethic(f) => f.name(),
        }
    }
    pub fn fully_qualified_name(&self) -> String {
        match self {
            OneofField::Real(f) => f.name(),
            OneofField::Synethic(f) => f.name(),
        }
    }
    pub fn containing_message(&self) -> Message<'a, U> {
        match self {
            OneofField::Real(f) => f.containing_message(),
            OneofField::Synethic(f) => f.containing_message(),
        }
    }
}
impl<'a, U> FullyQualified for OneofField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            OneofField::Real(f) => f.fully_qualified_name(),
            OneofField::Synethic(f) => f.fully_qualified_name(),
        }
    }
}
impl<'a, U> Named<U> for OneofField<'a, U> {
    fn name(&self) -> Name<U> {
        match self {
            OneofField::Real(f) => f.name(),
            OneofField::Synethic(f) => f.name(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RealOneofField<'a, U> {
    Scalar(OneofScalarField<'a, U>),
    Enum(OneofEnumField<'a, U>),
    Message(OneofMessageField<'a, U>),
}

impl<'a, U> RealOneofField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            RealOneofField::Scalar(f) => f.name(),
            RealOneofField::Enum(f) => f.name(),
            RealOneofField::Message(f) => f.name(),
        }
    }
    pub fn fully_qualified_name(&self) -> String {
        match self {
            RealOneofField::Scalar(f) => f.fully_qualified_name(),
            RealOneofField::Enum(f) => f.fully_qualified_name(),
            RealOneofField::Message(f) => f.fully_qualified_name(),
        }
    }
    pub fn containing_message(&self) -> Message<'a, U> {
        match self {
            RealOneofField::Scalar(f) => f.containing_message(),
            RealOneofField::Enum(f) => f.containing_message(),
            RealOneofField::Message(f) => f.containing_message(),
        }
    }
}
impl<'a, U> FullyQualified for RealOneofField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            RealOneofField::Scalar(f) => f.fully_qualified_name(),
            RealOneofField::Enum(f) => f.fully_qualified_name(),
            RealOneofField::Message(f) => f.fully_qualified_name(),
        }
    }
}
impl<'a, U> Named<U> for RealOneofField<'a, U> {
    fn name(&self) -> Name<U> {
        match self {
            RealOneofField::Scalar(f) => f.name(),
            RealOneofField::Enum(f) => f.name(),
            RealOneofField::Message(f) => f.name(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SyntheticOneofField<'a, U> {
    Scalar(OneofScalarField<'a, U>),
    Enum(OneofEnumField<'a, U>),
    Message(OneofMessageField<'a, U>),
}

impl<'a, U> SyntheticOneofField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            SyntheticOneofField::Scalar(f) => f.name(),
            SyntheticOneofField::Enum(f) => f.name(),
            SyntheticOneofField::Message(f) => f.name(),
        }
    }
    pub fn fully_qualified_name(&self) -> String {
        match self {
            SyntheticOneofField::Scalar(f) => f.fully_qualified_name(),
            SyntheticOneofField::Enum(f) => f.fully_qualified_name(),
            SyntheticOneofField::Message(f) => f.fully_qualified_name(),
        }
    }
    pub fn containing_message(&self) -> Message<'a, U> {
        match self {
            SyntheticOneofField::Scalar(f) => f.containing_message(),
            SyntheticOneofField::Enum(f) => f.containing_message(),
            SyntheticOneofField::Message(f) => f.containing_message(),
        }
    }
}
impl<'a, U> FullyQualified for SyntheticOneofField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            SyntheticOneofField::Scalar(f) => f.fully_qualified_name(),
            SyntheticOneofField::Enum(f) => f.fully_qualified_name(),
            SyntheticOneofField::Message(f) => f.fully_qualified_name(),
        }
    }
}
impl<'a, U> Named<U> for SyntheticOneofField<'a, U> {
    fn name(&self) -> Name<U> {
        match self {
            SyntheticOneofField::Scalar(f) => f.name(),
            SyntheticOneofField::Enum(f) => f.name(),
            SyntheticOneofField::Message(f) => f.name(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OneofScalarFieldDetail<'a, U> {
    scalar_field: ScalarField<'a, U>,
}
#[derive(Debug, Clone)]
pub struct OneofScalarField<'a, U> {
    scalar_field: Rc<OneofScalarFieldDetail<'a, U>>,
}

impl<'a, U> OneofScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.upgrade().unwrap()
    }
    pub fn oneof_field(&self) -> Rc<OneofField<'a, U>> {
        self.oneof_field.upgrade().unwrap()
    }
}
impl<'a, U> FullyQualified for OneofScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for OneofScalarField<'a, U> {
    fn name(&self) -> Name<U> {
        self.detail.name()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OneofMessageFieldDetail<'a, U> {
    oneof: Weak<Oneof<'a, U>>,
    message: WeakMessage<'a, U>,

    is_synthetic: bool,
    message_field: MessageField<'a, U>,
}

#[derive(Debug)]
pub struct OneofMessageField<'a, U>(Rc<OneofMessageFieldDetail<'a, U>>);

impl<'a, U> OneofMessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.upgrade().unwrap()
    }
    pub fn oneof_field(&self) -> Rc<OneofField<'a, U>> {
        self.oneof_field.upgrade().unwrap()
    }
}

impl<'a, U> FullyQualified for OneofMessageField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for OneofMessageField<'a, U> {
    fn name(&self) -> Name<U> {
        self.detail.name()
    }
}

#[derive(Debug, Clone)]
pub struct OneofEnumFieldDetail<'a, U> {
    detail: FieldDetail<'a, U>,
    oneof_field: Weak<OneofField<'a, U>>,
    field: Weak<Field<'a, U>>,
    is_synthetic: bool,
}
#[derive(Debug)]
pub struct OneofEnumField<'a, U>(Rc<OneofEnumFieldDetail<'a, U>>);

impl<'a, U> Clone for OneofEnumField<'a, U> {
    fn clone(&self) -> Self {
        OneofEnumField(self.0.clone())
    }
}

impl<'a, U> Downgrade for OneofEnumField<'a, U> {
    type Target = WeakOneofEnumField<'a, U>;
    fn downgrade(&self) -> WeakOneofEnumField<'a, U> {
        WeakOneofEnumField(self.0.downgrade())
    }
}

impl<'a, U> OneofEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.upgrade().unwrap()
    }
    pub fn oneof_field(&self) -> Rc<OneofField<'a, U>> {
        self.oneof_field.upgrade().unwrap()
    }
}
impl<'a, U> FullyQualified for OneofEnumField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for OneofEnumField<'a, U> {
    fn name(&self) -> Name<U> {
        self.detail.name()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum WeakOneofField<'a, U> {
    Real(WeakRealOneofField<'a, U>),
    Synethic(WeakSyntheticOneofField<'a, U>),
}

#[derive(Debug, Clone)]
pub(crate) enum WeakRealOneofField<'a, U> {
    Enum(WeakOneofEnumField<'a, U>),
    Message(WeakOneofMessageField<'a, U>),
    Scalar(WeakOneofScalarField<'a, U>),
}

#[derive(Debug, Clone)]
pub(crate) enum WeakSyntheticOneofField<'a, U> {
    Enum(WeakOneofEnumField<'a, U>),
    Message(WeakOneofMessageField<'a, U>),
    Scalar(WeakOneofScalarField<'a, U>),
}

#[derive(Debug)]
struct WeakOneofEnumField<'a, U>(Weak<OneofEnumFieldDetail<'a, U>>);
impl<'a, U> Clone for WeakOneofEnumField<'a, U> {
    fn clone(&self) -> Self {
        WeakOneofEnumField(self.0.clone())
    }
}
#[derive(Debug)]
struct WeakOneofScalarField<'a, U>(Weak<OneofScalarFieldDetail<'a, U>>);
impl<'a, U> Clone for WeakOneofScalarField<'a, U> {
    fn clone(&self) -> Self {
        WeakOneofScalarField(self.0.clone())
    }
}
#[derive(Debug)]
struct WeakOneofMessageField<'a, U>(Weak<OneofMessageFieldDetail<'a, U>>);
impl<'a, U> Clone for WeakOneofMessageField<'a, U> {
    fn clone(&self) -> Self {
        WeakOneofMessageField(self.0.clone())
    }
}

impl<'a, U> Upgrade for WeakOneofMessageField<'a, U> {
    type Target = OneofMessageField<'a, U>;
}
