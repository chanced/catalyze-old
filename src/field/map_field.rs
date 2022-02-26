use std::rc::{Rc, Weak};

use crate::{name::Named, Field, FullyQualified, Name};

use super::WeakField;

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
    fn fully_qualified_name(&self) -> String {
        match self {
            MapField::Scalar(f) => f.fully_qualified_name(),
            MapField::Enum(f) => f.fully_qualified_name(),
            MapField::Message(f) => f.fully_qualified_name(),
        }
    }
    fn field(&self) -> Field<'a, U> {
        match self {
            MapField::Scalar(s) => s.field(),
            MapField::Enum(e) => e.field(),
            MapField::Message(m) => m.field(),
        }
    }
}

impl<'a, U> FullyQualified for MapField<'a, U> {
    fn fully_qualified_name(&self) -> String {
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

#[derive(Debug, Clone)]
pub struct MapScalarFieldDetail<'a, U> {
    field: WeakField<'a, U>,
}
pub struct MapScalarField<'a, U>(Rc<MapScalarFieldDetail<'a, U>>);
impl<'a, U> MapScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.upgrade().unwrap()
    }
    pub fn map_field(&self) -> Rc<MapField<'a, U>> {
        self.map_field.upgrade().unwrap()
    }
}

impl<'a, U> Named<U> for MapScalarField<'a, U> {
    fn name(&self) -> Name<U> {
        self.detail.name()
    }
}

impl<'a, U> FullyQualified for MapScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}

#[derive(Debug, Clone)]
pub struct MapMessageField<'a, U> {
    field: Weak<Field<'a, U>>,
    map_field: Weak<MapField<'a, U>>,
    detail: FieldDetail<'a, U>,
}

impl<'a, U> MapMessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.upgrade().unwrap()
    }
    pub fn map_field(&self) -> Rc<MapField<'a, U>> {
        self.map_field.upgrade().unwrap()
    }
}

impl<'a, U> FullyQualified for MapMessageField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}

impl<'a, U> Named<U> for MapMessageField<'a, U> {
    fn name(&self) -> Name<U> {
        self.detail.name()
    }
}

#[derive(Debug, Clone)]
pub struct MapEnumField<'a, U> {
    field: Weak<Field<'a, U>>,
    map_field: Weak<MapField<'a, U>>,
    detail: FieldDetail<'a, U>,
}

impl<'a, U> MapEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.upgrade().unwrap()
    }
    pub fn map_field(&self) -> Rc<MapField<'a, U>> {
        self.map_field.upgrade().unwrap()
    }
}

impl<'a, U> FullyQualified for MapEnumField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for MapEnumField<'a, U> {
    fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
