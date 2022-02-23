use std::rc::Rc;

use crate::{Name, RepeatedEnumField, RepeatedMessageField, RepeatedScalarField};

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
