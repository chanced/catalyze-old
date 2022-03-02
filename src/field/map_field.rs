use std::rc::Rc;

use crate::{
    proto::descriptor::FieldDescriptor, proto::Syntax, Enum, FullyQualified, Message, Name,
    ScalarField, WeakEnum, WeakMessage, WellKnownEnum, WellKnownType,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapKey {
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
    pub fn fully_qualified_name(&self) -> String {
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

#[derive(Debug)]
pub enum MapField<'a, U> {
    Scalar(MappedScalarField<'a, U>),
    Enum(MappedEnumField<'a, U>),
    Embed(MappedEmbedField<'a, U>),
}

impl<'a, U> MapField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            MapField::Scalar(f) => f.name(),
            MapField::Enum(f) => f.name(),
            MapField::Embed(f) => f.name(),
        }
    }
    fn fully_qualified_name(&self) -> String {
        match self {
            MapField::Scalar(f) => f.fully_qualified_name(),
            MapField::Enum(f) => f.fully_qualified_name(),
            MapField::Embed(f) => f.fully_qualified_name(),
        }
    }
    pub fn embed(&self) -> Option<Message<'a, U>> {
        match self {
            MapField::Embed(m) => m.embed().into(),
            _ => None,
        }
    }
    /// alias for `r#enum`
    pub fn enumeration(&self) -> Option<Enum<'a, U>> {
        self.r#enum()
    }

    pub fn r#enum(&self) -> Option<Enum<'a, U>> {
        match self {
            MapField::Enum(e) => e.r#enum().into(),
            _ => None,
        }
    }
}
impl<'a, U> Clone for MapField<'a, U> {
    fn clone(&self) -> Self {
        match self {
            Self::Scalar(f) => Self::Scalar(f.clone()),
            Self::Enum(f) => Self::Enum(f.clone()),
            Self::Embed(f) => Self::Embed(f.clone()),
        }
    }
}
impl<'a, U> FullyQualified for MapField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            MapField::Scalar(f) => f.fully_qualified_name(),
            MapField::Enum(f) => f.fully_qualified_name(),
            MapField::Embed(f) => f.fully_qualified_name(),
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
        MapField::Embed(f)
    }
}
impl<'a, U> From<&MappedEmbedField<'a, U>> for MapField<'a, U> {
    fn from(f: &MappedEmbedField<'a, U>) -> Self {
        MapField::Embed(f.clone())
    }
}

#[derive(Debug, Clone)]
pub struct MappedScalarFieldDetail<'a, U> {
    detail: MapFieldDetail<'a, U>,
    scalar_field: ScalarField<'a, U>,
}

#[derive(Debug)]
pub struct MappedScalarField<'a, U>(Rc<MappedScalarFieldDetail<'a, U>>);
impl<'a, U> Clone for MappedScalarField<'a, U> {
    fn clone(&self) -> Self {
        MappedScalarField(self.0.clone())
    }
}

impl<'a, U> MappedScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn is_repeated(&self) -> bool {
        self.0.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.0.detail.is_map()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.0.detail.util()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.0.detail.replace_util(util);
    }
    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.0.detail.descriptor()
    }
}

impl<'a, U> FullyQualified for MappedScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}

#[derive(Debug)]
pub struct MappedEmbedFieldDetail<'a, U> {
    embed: WeakMessage<'a, U>,
    detail: MapFieldDetail<'a, U>,
}
impl<'a, U> Clone for MappedEmbedFieldDetail<'a, U> {
    fn clone(&self) -> Self {
        Self {
            embed: self.embed.clone(),
            detail: self.detail.clone(),
        }
    }
}

#[derive(Debug)]
pub struct MappedEmbedField<'a, U>(Rc<MappedEmbedFieldDetail<'a, U>>);

impl<'a, U> MappedEmbedField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn is_repeated(&self) -> bool {
        self.0.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.0.detail.is_map()
    }
    /// Returns the Message which contains this field.
    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.0.detail.util()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.0.detail.replace_util(util);
    }
    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.0.detail.descriptor()
    }
    /// Returns the embedded message.
    pub fn embed(&self) -> Message<'a, U> {
        self.0.embed.clone().into()
    }

    pub fn has_presence(&self) -> bool {
        true
    }
}

impl<'a, U> FullyQualified for MappedEmbedField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}

impl<'a, U> Clone for MappedEmbedField<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Debug, Clone)]
pub struct MappedEnumFieldDetail<'a, U> {
    enm: WeakEnum<'a, U>,
    detail: MapFieldDetail<'a, U>,
}

#[derive(Debug)]
pub struct MappedEnumField<'a, U>(Rc<MappedEnumFieldDetail<'a, U>>);
impl<'a, U> Clone for MappedEnumField<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<'a, U> MappedEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }

    pub fn is_repeated(&self) -> bool {
        self.0.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.0.detail.is_map()
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.0.detail.util()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.0.detail.replace_util(util);
    }
    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.0.detail.descriptor()
    }

    /// Alias for `r#enum()`
    pub fn enumeration(&self) -> Enum<'a, U> {
        self.r#enum()
    }
    pub fn r#enum(&self) -> Enum<'a, U> {
        self.0.enm.clone().into()
    }

    pub fn well_known_type(&self) -> Option<WellKnownEnum> {
        self.0.detail.well_known_type().map(|wkt| match wkt {
            crate::WellKnownType::Enum(wke) => wke,
            _ => unreachable!(),
        })
    }
}

impl<'a, U> FullyQualified for MappedEnumField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}
