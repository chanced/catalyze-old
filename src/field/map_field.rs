use crate::{MapEnumField, MapMessageField, MapScalarField, Name};

#[derive(Debug, Clone)]
pub enum MapField<'a, U> {
    Scalar(MapScalarField<'a, U>),
    Enum(MapEnumField<'a, U>),
    Message(MapMessageField<'a, U>),
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
