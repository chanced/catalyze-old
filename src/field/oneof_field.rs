mod oneof_enum_field;
mod oneof_message_field;
mod oneof_scalar_field;
mod real_oneof_field;
mod synthetic_oneof_field;
pub use oneof_enum_field::*;
pub use oneof_message_field::*;
pub use oneof_scalar_field::*;
pub use real_oneof_field::*;
pub use synthetic_oneof_field::*;

use std::rc::Weak;

use crate::{FullyQualified, Message, Name, Oneof};

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
            OneofField::Real(f) => f.fully_qualified_name(),
            OneofField::Synethic(f) => f.fully_qualified_name(),
        }
    }
    pub fn message(&self) -> Message<'a, U> {
        match self {
            OneofField::Real(f) => f.message(),
            OneofField::Synethic(f) => f.message(),
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
impl<'a, U> Clone for OneofField<'a, U> {
    fn clone(&self) -> Self {
        match self {
            Self::Real(f) => Self::Real(f.clone()),
            Self::Synethic(f) => Self::Synethic(f.clone()),
        }
    }
}

impl<'a, U> From<RealOneofField<'a, U>> for OneofField<'a, U> {
    fn from(f: RealOneofField<'a, U>) -> Self {
        OneofField::Real(f)
    }
}
impl<'a, U> From<&RealOneofField<'a, U>> for OneofField<'a, U> {
    fn from(f: &RealOneofField<'a, U>) -> Self {
        OneofField::Real(f.clone())
    }
}
impl<'a, U> From<SyntheticOneofField<'a, U>> for OneofField<'a, U> {
    fn from(f: SyntheticOneofField<'a, U>) -> Self {
        OneofField::Synethic(f)
    }
}
impl<'a, U> From<&SyntheticOneofField<'a, U>> for OneofField<'a, U> {
    fn from(f: &SyntheticOneofField<'a, U>) -> Self {
        OneofField::Synethic(f.clone())
    }
}
