use std::rc::Rc;

use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub enum OneofField<'a, U> {
    Scalar(Rc<OneofScalarField<'a, U>>),
    Enum(Rc<OneofEnumField<'a, U>>),
    Message(Rc<OneofMessageField<'a, U>>),
}

impl<'a, U> OneofField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            OneofField::Scalar(f) => f.name(),
            OneofField::Enum(f) => f.name(),
            OneofField::Message(f) => f.name(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OneofScalarField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> OneofScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}

#[derive(Debug, Clone)]
pub struct OneofMessageField<'a, U> {
    detail: FieldDetail<'a, U>,
}
impl<'a, U> OneofMessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}

#[derive(Debug, Clone)]
pub struct OneofEnumField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> OneofEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
