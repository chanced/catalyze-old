use std::rc::Rc;

use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub enum MapField<'a, U> {
    Scalar(Rc<MapScalarField<'a, U>>),
    Enum(Rc<MapEnumField<'a, U>>),
    Message(Rc<MapMessageField<'a, U>>),
}

pub enum MapFieldKey {
    Int64 = 3,
    Uint64 = 4,
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    String = 9,
    Uint32 = 13,
    Sfixed32 = 15,
    Sfixed64 = 16,
    Sint32 = 17,
    Sint64 = 18,
}

impl<'a, U> MapField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            MapField::Scalar(f) => f.name(),
            MapField::Enum(f) => f.name(),
            MapField::Message(f) => f.name(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MapScalarField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> MapScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}

#[derive(Debug, Clone)]
pub struct MapMessageField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> MapMessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}

#[derive(Debug, Clone)]
pub struct MapEnumField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> MapEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
