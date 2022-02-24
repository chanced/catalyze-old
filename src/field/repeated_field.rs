use std::rc::Rc;

use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub enum RepeatedField<'a, U> {
    Scalar(Rc<RepeatedScalarField<'a, U>>),
    Enum(Rc<RepeatedEnumField<'a, U>>),
    Message(Rc<RepeatedMessageField<'a, U>>),
}

impl<'a, U> RepeatedField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            RepeatedField::Scalar(f) => f.name(),
            RepeatedField::Enum(f) => f.name(),
            RepeatedField::Message(f) => f.name(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RepeatedEnumField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> RepeatedEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}

#[derive(Debug, Clone)]
pub struct RepeatedMessageField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> RepeatedMessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}

#[derive(Debug, Clone)]
pub struct RepeatedScalarField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> RepeatedScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
