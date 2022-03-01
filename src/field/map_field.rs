mod map_key;
mod mapped_embed_field;
mod mapped_enum_field;
mod mapped_scalar_field;

use std::rc::Rc;

pub use map_key::*;
pub use mapped_embed_field::*;
pub use mapped_enum_field::*;
pub use mapped_scalar_field::*;

use crate::{
    descriptor::FieldDescriptor, name::Named, proto::Syntax, Enum, Field, FullyQualified, Message,
    Name, WellKnownType,
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
    pub fn message(&self) -> Message<'a, U> {
        self.detail.message()
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.detail.util()
    }
    pub fn syntax(&self) -> Syntax {
        self.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.detail.descriptor()
    }

    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.detail.replace_util(util);
    }

    fn well_known_type(&self) -> Option<WellKnownType> {
        todo!()
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
    Scalar(MappedScalarField<'a, U>),
    Enum(MappedEnumField<'a, U>),
    Message(MappedEmbedField<'a, U>),
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
    pub fn embed(&self) -> Option<Message<'a, U>> {
        match self {
            MapField::Message(m) => Some(m.clone()),
            _ => None,
        }
    }
    pub fn r#enum(&self) -> Option<Enum<'a, U>> {
        match self {
            MapField::Enum(e) => Some(e.clone()),
            _ => None,
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

impl<'a, U> From<MappedEnumField<'a, U>> for MapField<'a, U> {
    fn from(f: MappedEnumField<'a, U>) -> Self {
        MapField::Enum(f)
    }
}
impl<'a, U> From<&MappedEnumField<'a, U>> for MapField<'a, U> {
    fn from(f: &MappedEnumField<'a, U>) -> Self {
        MapField::Enum(f.clone())
    }
}

impl<'a, U> From<MappedScalarField<'a, U>> for MapField<'a, U> {
    fn from(f: MappedScalarField<'a, U>) -> Self {
        MapField::Scalar(f)
    }
}
impl<'a, U> From<&MappedScalarField<'a, U>> for MapField<'a, U> {
    fn from(f: &MappedScalarField<'a, U>) -> Self {
        MapField::Scalar(f.clone())
    }
}

impl<'a, U> From<MappedEmbedField<'a, U>> for MapField<'a, U> {
    fn from(f: MappedEmbedField<'a, U>) -> Self {
        MapField::Message(f)
    }
}
impl<'a, U> From<&MappedEmbedField<'a, U>> for MapField<'a, U> {
    fn from(f: &MappedEmbedField<'a, U>) -> Self {
        MapField::Message(f.clone())
    }
}

#[derive(Clone, Debug)]
pub(crate) enum WeakMapField<'a, U> {
    Scalar(WeakMappedScalarField<'a, U>),
    Enum(WeakMapEnumField<'a, U>),
    Message(WeakMappedEmbedField<'a, U>),
}
