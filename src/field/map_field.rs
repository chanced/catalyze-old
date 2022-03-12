#![allow(clippy::new_ret_no_self)]
use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, bail};

use crate::{
    proto::{FieldDescriptor, Type},
    proto::{Scalar, Syntax},
    Comments, Enum, Field, File, Files, FullyQualified, Message, Name, Node, Package, WeakEnum,
    WeakMessage, WellKnownEnum, WellKnownMessage, WellKnownType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
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

impl<'a> TryFrom<Type<'a>> for Key {
    type Error = anyhow::Error;

    fn try_from(t: Type<'a>) -> Result<Self, Self::Error> {
        match t {
            Type::Scalar(s) => match s {
                Scalar::Int64 => Ok(Key::Int64),
                Scalar::Uint64 => Ok(Key::Uint64),
                Scalar::Int32 => Ok(Key::Int32),
                Scalar::Fixed64 => Ok(Key::Fixed64),
                Scalar::Fixed32 => Ok(Key::Fixed32),
                Scalar::String => Ok(Key::String),
                Scalar::Uint32 => Ok(Key::Uint32),
                Scalar::Sfixed32 => Ok(Key::Sfixed32),
                Scalar::Sfixed64 => Ok(Key::Sfixed64),
                Scalar::Sint32 => Ok(Key::Sint32),
                Scalar::Sint64 => Ok(Key::Sint64),
                _ => bail!("invalid map key type: {}", s),
            },
            _ => bail!("invalid map key type: {}", t),
        }
    }
}

use super::FieldDetail;

#[derive(Debug)]
pub(crate) struct MapFieldDetail<'a, U> {
    key: Key,
    detail: FieldDetail<'a, U>,
}

impl<'a, U> MapFieldDetail<'a, U> {
    pub fn new(detail: FieldDetail<'a, U>) -> Result<Self, anyhow::Error> {
        if detail.map_entry.is_none() {}
        let map_entry = detail.map_entry()?;

        let key = map_entry
            .fields()
            .get(0)
            .ok_or_else(|| anyhow!("map entry {} is missing key field", &map_entry.name()))?;

        todo!()
    }

    fn value_field(&self) -> Result<Field<'a, U>, anyhow::Error> {
        let map_entry = self.detail.map_entry()?;
        map_entry
            .fields()
            .get(1)
            .ok_or_else(|| anyhow!("map entry {} is missing value field", &map_entry.name()))
    }

    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }

    pub fn key(&self) -> Key {
        self.key
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
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.detail.descriptor()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.detail.comments()
    }
    pub fn set_comments(&self, comments: Comments<'a>) {
        self.detail.comments.replace(comments);
    }

    pub fn file(&self) -> File<'a, U> {
        self.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.detail.package()
    }

    pub fn build_target(&self) -> bool {
        self.detail.build_target()
    }

    pub fn is_marked_required(&self) -> bool {
        self.detail.is_marked_required()
    }
}

impl<'a, U> Clone for MapFieldDetail<'a, U> {
    fn clone(&self) -> Self {
        Self {
            key: self.key,
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

    pub fn key(&self) -> Key {
        match self {
            MapField::Scalar(f) => f.key(),
            MapField::Enum(f) => f.key(),
            MapField::Embed(f) => f.key(),
        }
    }

    pub fn fully_qualified_name(&self) -> String {
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
    pub fn comments(&self) -> Comments<'a> {
        match self {
            MapField::Scalar(f) => f.comments(),
            MapField::Enum(f) => f.comments(),
            MapField::Embed(f) => f.comments(),
        }
    }

    pub fn file(&self) -> File<'a, U> {
        match self {
            MapField::Scalar(f) => f.file(),
            MapField::Enum(f) => f.file(),
            MapField::Embed(f) => f.file(),
        }
    }

    pub fn build_target(&self) -> bool {
        match self {
            MapField::Scalar(f) => f.build_target(),
            MapField::Enum(f) => f.build_target(),
            MapField::Embed(f) => f.build_target(),
        }
    }

    pub fn package(&self) -> Package<'a, U> {
        match self {
            MapField::Scalar(f) => f.package(),
            MapField::Enum(f) => f.package(),
            MapField::Embed(f) => f.package(),
        }
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        match self {
            MapField::Scalar(f) => f.set_comments(comments),
            MapField::Enum(f) => f.set_comments(comments),
            MapField::Embed(f) => f.set_comments(comments),
        }
    }

    pub fn has_import(&self) -> bool {
        match self {
            MapField::Scalar(_) => false,
            MapField::Enum(f) => f.has_import(),
            MapField::Embed(f) => f.has_import(),
        }
    }
    pub fn imports(&self) -> Files<'a, U> {
        match self {
            MapField::Scalar(_) => Files::empty(),
            MapField::Enum(f) => f.imports(),
            MapField::Embed(f) => f.imports(),
        }
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        match self {
            MapField::Scalar(f) => f.descriptor(),
            MapField::Enum(f) => f.descriptor(),
            MapField::Embed(f) => f.descriptor(),
        }
    }

    pub fn syntax(&self) -> Syntax {
        match self {
            MapField::Scalar(f) => f.syntax(),
            MapField::Enum(f) => f.syntax(),
            MapField::Embed(f) => f.syntax(),
        }
    }

    pub fn scalar(&self) -> Option<Scalar> {
        match self {
            MapField::Scalar(f) => Some(f.scalar()),
            _ => None,
        }
    }

    pub fn is_marked_required(&self) -> bool {
        match self {
            MapField::Scalar(f) => f.is_marked_required(),
            MapField::Enum(f) => f.is_marked_required(),
            MapField::Embed(f) => f.is_marked_required(),
        }
    }

    pub fn is_embed(&self) -> bool {
        matches!(self, MapField::Embed(_))
    }

    pub fn is_well_known_type(&self) -> bool {
        match self {
            MapField::Enum(f) => f.is_well_known_type(),
            MapField::Embed(f) => f.is_well_known_type(),
            MapField::Scalar(_) => false,
        }
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        match self {
            MapField::Enum(f) => f.well_known_type(),
            MapField::Embed(f) => f.well_known_type(),
            MapField::Scalar(_) => None,
        }
    }

    pub fn is_scalar(&self) -> bool {
        matches!(self, MapField::Scalar(_))
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, MapField::Enum(_))
    }

    pub(crate) fn has_presence(&self) -> bool {
        match self {
            MapField::Scalar(f) => f.has_presence(),
            MapField::Enum(f) => f.has_presence(),
            MapField::Embed(f) => f.has_presence(),
        }
    }

    pub fn util(&self) -> Rc<U> {
        match self {
            MapField::Scalar(f) => f.util(),
            MapField::Enum(f) => f.util(),
            MapField::Embed(f) => f.util(),
        }
    }

    pub(crate) fn set_value(&self, value: Node<'a, U>) -> Result<(), anyhow::Error> {
        match self {
            MapField::Enum(f) => f.set_value(value),
            MapField::Embed(f) => f.set_value(value),
            _ => unreachable!(),
        }
    }

    pub fn value_type(&self) -> Type<'a> {
        match self {
            MapField::Scalar(_) => self.descriptor().proto_type(),
            MapField::Enum(e) => e.value_type(),
            MapField::Embed(e) => e.value_type(),
        }
    }

    pub(crate) fn new(detail: FieldDetail<'a, U>) -> Result<Field<'a, U>, anyhow::Error> {
        if !detail.is_map() {
            bail!("Field is not a map")
        }
        let key = detail.map_key()?;
        match detail.value_type() {
            Type::Scalar(s) => Ok(Field::Map(MapField::Scalar(MappedScalarField(Rc::new(
                MappedScalarFieldDetail {
                    detail: MapFieldDetail { detail, key },
                    scalar: s,
                },
            ))))),
            Type::Enum(_) => Ok(Field::Map(MapField::Enum(MappedEnumField(Rc::new(
                MappedEnumFieldDetail {
                    detail: MapFieldDetail { detail, key },
                    enumeration: RefCell::new(WeakEnum::empty()),
                },
            ))))),
            Type::Message(_) => Ok(Field::Map(MapField::Embed(MappedEmbedField(Rc::new(
                MappedEmbedFieldDetail {
                    detail: MapFieldDetail { detail, key },
                    embed: RefCell::new(WeakMessage::empty()),
                },
            ))))),
            Type::Group => bail!("group is not a map"),
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
    scalar: Scalar,
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

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }

    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }

    pub fn build_target(&self) -> bool {
        self.0.detail.build_target()
    }

    pub fn scalar(&self) -> Scalar {
        self.0.scalar
    }

    pub fn key(&self) -> Key {
        self.0.detail.key()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }

    pub fn has_presence(&self) -> bool {
        false
    }
}

impl<'a, U> FullyQualified for MappedScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}

#[derive(Debug)]
pub struct MappedEmbedFieldDetail<'a, U> {
    embed: RefCell<WeakMessage<'a, U>>,
    detail: MapFieldDetail<'a, U>,
}
impl<'a, U> MappedEmbedFieldDetail<'a, U> {
    pub(crate) fn embed(&self) -> WeakMessage<'a, U> {
        self.embed.borrow().clone()
    }
    pub(crate) fn value_type(&self) -> Type<'a> {
        self.detail.value_field().unwrap().value_type()
    }
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

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }
    /// Returns the embedded message.
    pub fn embed(&self) -> Message<'a, U> {
        self.0.embed().into()
    }

    pub fn has_presence(&self) -> bool {
        false
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }
    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }

    pub fn has_import(&self) -> bool {
        self.file() != self.0.embed().file()
    }
    pub fn imports(&self) -> Files<'a, U> {
        if self.has_import() {
            Files::from(self.0.embed().weak_file())
        } else {
            Files::empty()
        }
    }

    pub fn build_target(&self) -> bool {
        self.0.detail.build_target()
    }

    pub fn key(&self) -> Key {
        self.0.detail.key()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.0.embed().is_well_known_type()
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.0.embed().well_known_type()
    }
    pub fn well_known_message(&self) -> Option<WellKnownMessage> {
        self.0.embed().well_known_message()
    }

    pub(crate) fn set_value(&self, node: Node<'a, U>) -> Result<(), anyhow::Error> {
        match node {
            Node::Message(m) => {
                self.0.embed.replace(m.into());
                Ok(())
            }
            _ => bail!("expected Message, received {}", node),
        }
    }

    pub(crate) fn value_type(&self) -> Type<'a> {
        self.0.value_type()
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
    enumeration: RefCell<WeakEnum<'a, U>>,
    detail: MapFieldDetail<'a, U>,
}

impl<'a, U> MappedEnumFieldDetail<'a, U> {
    pub fn enumeration(&self) -> Enum<'a, U> {
        self.enumeration.borrow().clone().into()
    }
    pub(crate) fn value_type(&self) -> Type<'a> {
        self.detail.value_field().unwrap().value_type()
    }
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

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }
    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
    }

    /// Alias for `r#enum()`
    pub fn enumeration(&self) -> Enum<'a, U> {
        self.r#enum()
    }
    pub fn r#enum(&self) -> Enum<'a, U> {
        self.0.enumeration()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.enumeration().well_known_type()
    }
    pub fn well_known_enum(&self) -> Option<WellKnownEnum> {
        self.enumeration().well_known_enum()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.enumeration().is_well_known_type()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }

    pub fn has_import(&self) -> bool {
        return self.enumeration().file() != self.file();
    }
    pub fn imports(&self) -> Files<'a, U> {
        if self.has_import() {
            Files::from(self.enumeration().weak_file())
        } else {
            Files::empty()
        }
    }

    pub fn build_target(&self) -> bool {
        self.0.detail.build_target()
    }

    pub fn key(&self) -> Key {
        self.0.detail.key()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }

    pub fn has_presence(&self) -> bool {
        false
    }

    fn set_value(&self, value: Node<'a, U>) -> Result<(), anyhow::Error> {
        match value {
            Node::Enum(e) => {
                self.0.enumeration.replace(e.into());
                Ok(())
            }
            _ => bail!("expected Enum, received {}", value),
        }
    }

    fn value_type(&self) -> Type<'a> {
        self.0.value_type()
    }
}

impl<'a, U> FullyQualified for MappedEnumField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}
