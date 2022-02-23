use std::rc::Rc;

use crate::{Name, RepeatedEnumField, RepeatedMessageField, RepeatedScalarField};

#[derive(Debug, Clone)]
pub enum RepeatedField<'a, U> {
    Scalar(RepeatedScalarField<'a, U>),
    Enum(RepeatedEnumField<'a, U>),
    Message(RepeatedMessageField<'a, U>),
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
