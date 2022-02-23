use std::rc::Rc;

use crate::{Name, OneofEnumField, OneofMessageField, OneofScalarField};

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
