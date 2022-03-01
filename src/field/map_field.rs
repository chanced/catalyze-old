mod map_enum_field;
mod map_key;
mod map_message_field;
mod map_scalar_field;

use std::rc::Rc;

pub use map_enum_field::*;
pub use map_key::*;
pub use map_message_field::*;
pub use map_scalar_field::*;

use crate::{
    descriptor::FieldDescriptor, name::Named, proto::Syntax, Field, FullyQualified, Message, Name,
};

use super::FieldDetail;

#[derive(Debug)]
pub(crate) struct MapFieldDetail<'a, U> {
    key: MapKey,
    syntax: Syntax,
    detail: FieldDetail<'a, U>,
}

impl<'a, U> MapFieldDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.detail.fully_qualified_name()
    }
    pub fn is_repeated(&self) -> bool {
        self.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.detail.is_map()
    }
    pub fn container(&self) -> Message<'a, U> {
        self.detail.container()
    }
    pub fn util(&self) -> Rc<U> {
        self.detail.util()
    }
    pub fn syntax(&self) -> Syntax {
        self.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.detail.descriptor()
    }
}

impl<'a, U> Clone for MapFieldDetail<'a, U> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            syntax: self.syntax.clone(),
            detail: self.detail.clone(),
        }
    }
}

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
    fn fully_qualified_name(&self) -> &str {
        match self {
            MapField::Scalar(f) => f.fully_qualified_name(),
            MapField::Enum(f) => f.fully_qualified_name(),
            MapField::Message(f) => f.fully_qualified_name(),
        }
    }
}

impl<'a, U> FullyQualified for MapField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        match self {
            MapField::Scalar(f) => f.fully_qualified_name(),
            MapField::Enum(f) => f.fully_qualified_name(),
            MapField::Message(f) => f.fully_qualified_name(),
        }
    }
}

impl<'a, U> Named<U> for MapField<'a, U> {
    fn name(&self) -> Name<U> {
        match self {
            MapField::Scalar(f) => f.name(),
            MapField::Enum(f) => f.name(),
            MapField::Message(f) => f.name(),
        }
    }
}

impl<'a, U> From<MapEnumField<'a, U>> for MapField<'a, U> {
    fn from(f: MapEnumField<'a, U>) -> Self {
        MapField::Enum(f)
    }
}
impl<'a, U> From<&MapEnumField<'a, U>> for MapField<'a, U> {
    fn from(f: &MapEnumField<'a, U>) -> Self {
        MapField::Enum(f.clone())
    }
}

impl<'a, U> From<MapScalarField<'a, U>> for MapField<'a, U> {
    fn from(f: MapScalarField<'a, U>) -> Self {
        MapField::Scalar(f)
    }
}
impl<'a, U> From<&MapScalarField<'a, U>> for MapField<'a, U> {
    fn from(f: &MapScalarField<'a, U>) -> Self {
        MapField::Scalar(f.clone())
    }
}

impl<'a, U> From<MapMessageField<'a, U>> for MapField<'a, U> {
    fn from(f: MapMessageField<'a, U>) -> Self {
        MapField::Message(f)
    }
}
impl<'a, U> From<&MapMessageField<'a, U>> for MapField<'a, U> {
    fn from(f: &MapMessageField<'a, U>) -> Self {
        MapField::Message(f.clone())
    }
}

#[derive(Clone, Debug)]
pub(crate) enum WeakMapField<'a, U> {
    Scalar(WeakMapScalarField<'a, U>),
    Enum(WeakMapEnumField<'a, U>),
    Message(WeakMapMessageField<'a, U>),
}
