mod enum_field;

mod embed_field;
mod map_field;
mod oneof_field;
mod repeated_field;
mod scalar_field;

pub use embed_field::*;
pub use enum_field::*;
pub use map_field::*;
pub use oneof_field::*;
pub use repeated_field::*;
pub use scalar_field::*;

use crate::{
    proto::{FieldDescriptor, Scalar, Syntax},
    Comments, Enum, File, Files, FullyQualified, Message, Name, Node, NodeAtPath, Nodes, Package,
    WeakMessage, WellKnownType,
};
use std::{cell::RefCell, convert::From, rc::Rc};

pub(crate) type FieldList<'a, U> = Rc<RefCell<Vec<Field<'a, U>>>>;

#[derive(Debug)]
pub(crate) struct FieldDetail<'a, U> {
    msg: WeakMessage<'a, U>,
    name: Name<U>,
    fqn: String,
    syntax: Syntax,
    is_map: bool,
    in_oneof: bool,
    util: RefCell<Rc<U>>,
    desc: FieldDescriptor<'a>,
    comments: RefCell<Comments<'a, U>>,
}
impl<'a, U> Clone for FieldDetail<'a, U> {
    fn clone(&self) -> Self {
        Self {
            msg: self.msg.clone(),
            name: self.name.clone(),
            fqn: self.fqn.clone(),
            syntax: self.syntax,
            is_map: self.is_map,
            util: self.util.clone(),
            desc: self.desc,
            in_oneof: self.in_oneof,
            comments: self.comments.clone(),
        }
    }
}

impl<'a, U> FieldDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.name.clone()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.msg.clone().into()
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.util.borrow().clone()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.util.replace(util);
    }

    pub fn syntax(&self) -> Syntax {
        self.syntax
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.desc
    }
    pub fn is_map(&self) -> bool {
        self.is_map
    }
    pub fn is_repeated(&self) -> bool {
        self.desc.is_repeated()
    }
    pub fn is_scalar(&self) -> bool {
        self.desc.is_scalar()
    }
    pub fn is_enum(&self) -> bool {
        self.desc.is_enum()
    }
    pub fn is_embed(&self) -> bool {
        self.desc.is_embed()
    }
    pub fn is_marked_optional(&self) -> bool {
        self.desc.is_marked_optional(self.syntax)
    }
    pub fn is_marked_required(&self) -> bool {
        self.desc.is_marked_required(self.syntax)
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.comments.replace(comments);
    }
    pub fn comments(&self) -> Comments<'a, U> {
        *self.comments.borrow()
    }
    pub fn file(&self) -> File<'a, U> {
        self.msg.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.file().package()
    }
    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }
}

#[derive(Debug)]
pub enum Field<'a, U> {
    Embed(EmbedField<'a, U>),
    Enum(EnumField<'a, U>),
    Map(MapField<'a, U>),
    Oneof(OneofField<'a, U>),
    Repeated(RepeatedField<'a, U>),
    Scalar(ScalarField<'a, U>),
}

impl<'a, U> Field<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            Field::Embed(f) => f.name(),
            Field::Enum(f) => f.name(),
            Field::Map(f) => f.name(),
            Field::Oneof(f) => f.name(),
            Field::Repeated(f) => f.name(),
            Field::Scalar(f) => f.name(),
        }
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        match self {
            Field::Embed(f) => f.descriptor(),
            Field::Enum(f) => f.descriptor(),
            Field::Map(f) => f.descriptor(),
            Field::Oneof(f) => f.descriptor(),
            Field::Repeated(f) => f.descriptor(),
            Field::Scalar(f) => f.descriptor(),
        }
    }

    pub fn syntax(&self) -> Syntax {
        match self {
            Field::Embed(f) => f.syntax(),
            Field::Enum(f) => f.syntax(),
            Field::Map(f) => f.syntax(),
            Field::Oneof(f) => f.syntax(),
            Field::Repeated(f) => f.syntax(),
            Field::Scalar(f) => f.syntax(),
        }
    }

    pub fn fully_qualified_name(&self) -> String {
        match self {
            Field::Embed(f) => f.fully_qualified_name(),
            Field::Enum(f) => f.fully_qualified_name(),
            Field::Map(f) => f.fully_qualified_name(),
            Field::Oneof(f) => f.fully_qualified_name(),
            Field::Repeated(f) => f.fully_qualified_name(),
            Field::Scalar(f) => f.fully_qualified_name(),
        }
    }

    pub fn comments(&self) -> Comments<'a, U> {
        match self {
            Field::Embed(f) => f.comments(),
            Field::Enum(f) => f.comments(),
            Field::Map(f) => f.comments(),
            Field::Oneof(f) => f.comments(),
            Field::Repeated(f) => f.comments(),
            Field::Scalar(f) => f.comments(),
        }
    }
    pub fn has_import(&self) -> bool {
        match self {
            Field::Embed(f) => f.has_import(),
            Field::Enum(f) => f.has_import(),
            Field::Map(f) => f.has_import(),
            Field::Oneof(f) => f.has_import(),
            Field::Repeated(f) => f.has_import(),
            Field::Scalar(_) => false,
        }
    }

    pub fn imports(&self) -> Files<'a, U> {
        match self {
            Field::Embed(f) => f.imports(),
            Field::Enum(f) => f.imports(),
            Field::Map(f) => f.imports(),
            Field::Oneof(f) => f.imports(),
            Field::Repeated(f) => f.imports(),
            Field::Scalar(f) => Files::empty(),
        }
    }

    pub fn build_target(&self) -> bool {
        match self {
            Field::Embed(f) => f.build_target(),
            Field::Enum(f) => f.build_target(),
            Field::Map(f) => f.build_target(),
            Field::Oneof(f) => f.build_target(),
            Field::Repeated(f) => f.build_target(),
            Field::Scalar(f) => f.build_target(),
        }
    }
    pub fn r#enum(&self) -> Option<Enum<'a, U>> {
        match self {
            Field::Enum(f) => Some(f.r#enum()),
            Field::Map(f) => f.r#enum(),
            Field::Oneof(f) => f.r#enum(),
            Field::Repeated(f) => f.r#enum(),
            _ => None,
        }
    }
    pub fn enumeration(&self) -> Option<Enum<'a, U>> {
        match self {
            Field::Enum(f) => Some(f.enumeration()),
            Field::Map(f) => f.enumeration(),
            Field::Oneof(f) => f.enumeration(),
            Field::Repeated(f) => f.enumeration(),
            _ => None,
        }
    }
    pub fn embed(&self) -> Option<Message<'a, U>> {
        match self {
            Field::Embed(f) => Some(f.embed()),
            Field::Map(f) => f.embed(),
            Field::Oneof(f) => f.embed(),
            Field::Repeated(f) => f.embed(),
            _ => None,
        }
    }

    pub fn scalar(&self) -> Option<Scalar> {
        match self {
            Field::Map(f) => f.scalar(),
            Field::Oneof(f) => f.scalar(),
            Field::Repeated(f) => f.scalar(),
            Field::Scalar(f) => Some(f.scalar()),
            _ => None,
        }
    }

    pub fn is_repeated(&self) -> bool {
        match self {
            Field::Repeated(_) => true,
            _ => false,
        }
    }

    pub fn is_embed(&self) -> bool {
        match self {
            Field::Embed(f) => true,
            Field::Map(f) => f.is_embed(),
            Field::Oneof(f) => f.is_embed(),
            Field::Repeated(f) => f.is_embed(),
            _ => false,
        }
    }
    pub fn is_in_oneof(&self) -> bool {
        match self {
            Field::Oneof(_) => true,
            _ => false,
        }
    }

    pub fn is_in_real_oneof(&self) -> bool {
        match self {
            Field::Oneof(f) => f.is_in_real_oneof(),
            _ => false,
        }
    }
    pub fn is_in_synthetic_oneof(&self) -> bool {
        match self {
            Field::Oneof(f) => f.is_in_synthetic_oneof(),
            _ => false,
        }
    }
    pub fn is_well_known_type(&self) -> bool {
        match self {
            Field::Embed(f) => f.is_well_known_type(),
            Field::Enum(f) => f.is_well_known_type(),
            Field::Map(f) => f.is_well_known_type(),
            Field::Oneof(f) => f.is_well_known_type(),
            Field::Repeated(f) => f.is_well_known_type(),
            _ => false,
        }
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        match self {
            Field::Embed(f) => f.well_known_type(),
            Field::Enum(f) => f.well_known_type(),
            Field::Map(f) => f.well_known_type(),
            Field::Oneof(f) => f.well_known_type(),
            Field::Repeated(f) => f.well_known_type(),
            _ => None,
        }
    }
    /// Indicates whether or not the field is labeled as a required field. This
    /// will only be `true` if the syntax is proto2.
    pub fn is_marked_required(&self) -> bool {
        match self {
            Field::Embed(f) => f.is_marked_required(),
            Field::Enum(f) => f.is_marked_required(),
            Field::Map(f) => f.is_marked_required(),
            Field::Oneof(f) => f.is_marked_required(),
            Field::Repeated(f) => f.is_marked_required(),
            Field::Scalar(f) => f.is_marked_required(),
        }
    }
    pub fn is_marked_optional(&self) -> bool {
        match self {
            Field::Embed(f) => f.is_marked_optional(),
            Field::Enum(f) => f.is_marked_optional(),
            Field::Scalar(f) => f.is_marked_optional(),
            _ => false,
        }
    }
    pub fn is_scalar(&self) -> bool {
        match self {
            Field::Map(f) => f.is_scalar(),
            Field::Oneof(f) => f.is_scalar(),
            Field::Repeated(f) => f.is_scalar(),
            Field::Scalar(f) => true,
            _ => false,
        }
    }
    pub fn is_enum(&self) -> bool {
        match self {
            Field::Enum(f) => true,
            Field::Map(f) => f.is_enum(),
            Field::Oneof(f) => f.is_enum(),
            Field::Repeated(f) => f.is_enum(),
            _ => false,
        }
    }
    /// Returns `true` for all fields that have explicit presence.
    ///
    /// ---
    /// ### Proto2
    /// Field type                                   | Explicit Presence
    /// -------------------------------------------- | :-----------------:
    /// Singular numeric (integer or floating point) | ✔️
    /// Singular enum                                | ✔️
    /// Singular string or bytes                     | ✔️
    /// Singular message                             | ✔️
    /// Repeated                                     |
    /// Oneofs                                       | ✔️
    /// Maps                                         |
    ///
    /// ---
    /// ### Proto3
    /// Field type                                   | `optional` | Explicit Presence
    /// -------------------------------------------- |:----------: | :-----------------:
    /// Singular numeric (integer or floating point) | No         |
    /// Singular enum                                | No         |
    /// Singular string or bytes                     | No         |
    /// Singular numeric (integer or floating point) | Yes        | ✔️
    /// Singular enum                                | Yes        | ✔️
    /// Singular string or bytes                     | Yes        | ✔️
    /// Singular message                             | Yes        | ✔️
    /// Singular message                             | No         | ✔️
    /// Repeated                                     | N/A        |
    /// Oneofs                                       | N/A        | ✔️
    /// Maps                                         | N/A        |
    ///
    /// See:
    /// - https://github.com/protocolbuffers/protobuf/blob/v3.17.0/docs/field_presence.md
    /// - https://github.com/protocolbuffers/protobuf/blob/master/docs/implementing_proto3_presence.md
    pub fn has_presence(&self) -> bool {
        match self {
            Field::Embed(f) => f.has_presence(),
            Field::Enum(f) => f.has_presence(),
            Field::Map(f) => f.has_presence(),
            Field::Oneof(f) => f.has_presence(),
            Field::Repeated(f) => f.has_presence(),
            Field::Scalar(f) => f.has_presence(),
        }
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        match self {
            Field::Embed(f) => f.replace_util(util),
            Field::Enum(f) => f.replace_util(util),
            Field::Map(f) => f.replace_util(util),
            Field::Oneof(f) => f.replace_util(util),
            Field::Repeated(f) => f.replace_util(util),
            Field::Scalar(f) => f.replace_util(util),
        }
    }
    pub fn is_map(&self) -> bool {
        matches!(self, Field::Map(_))
    }

    pub fn file(&self) -> File<'a, U> {
        match self {
            Field::Embed(f) => f.file(),
            Field::Enum(f) => f.file(),
            Field::Map(f) => f.file(),
            Field::Oneof(f) => f.file(),
            Field::Repeated(f) => f.file(),
            Field::Scalar(f) => f.file(),
        }
    }
    pub fn package(&self) -> Package<'a, U> {
        match self {
            Field::Embed(f) => f.package(),
            Field::Enum(f) => f.package(),
            Field::Map(f) => f.package(),
            Field::Oneof(f) => f.package(),
            Field::Repeated(f) => f.package(),
            Field::Scalar(f) => f.package(),
        }
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        match self {
            Field::Embed(f) => f.set_comments(comments),
            Field::Enum(f) => f.set_comments(comments),
            Field::Map(f) => f.set_comments(comments),
            Field::Oneof(f) => f.set_comments(comments),
            Field::Repeated(f) => f.set_comments(comments),
            Field::Scalar(f) => f.set_comments(comments),
        }
    }

    pub(crate) fn nodes(&self) -> Nodes<'a, U> {
        Nodes::empty()
    }

    pub(crate) fn util(&self) -> Rc<U> {
        match self {
            Field::Embed(f) => f.util(),
            Field::Enum(f) => f.util(),
            Field::Map(f) => f.util(),
            Field::Oneof(f) => f.util(),
            Field::Repeated(f) => f.util(),
            Field::Scalar(f) => f.util(),
        }
    }
}
impl<'a, U> Clone for Field<'a, U> {
    fn clone(&self) -> Self {
        match self {
            Self::Embed(f) => Self::Embed(f.clone()),
            Self::Enum(f) => Self::Enum(f.clone()),
            Self::Map(f) => Self::Map(f.clone()),
            Self::Oneof(f) => Self::Oneof(f.clone()),
            Self::Repeated(f) => Self::Repeated(f.clone()),
            Self::Scalar(f) => Self::Scalar(f.clone()),
        }
    }
}

impl<'a, U> NodeAtPath<'a, U> for Field<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            Some(self.into())
        } else {
            None
        }
    }
}

impl<'a, U> FullyQualified for Field<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            Field::Enum(f) => f.fully_qualified_name(),
            Field::Map(f) => f.fully_qualified_name(),
            Field::Embed(f) => f.fully_qualified_name(),
            Field::Oneof(f) => f.fully_qualified_name(),
            Field::Repeated(f) => f.fully_qualified_name(),
            Field::Scalar(f) => f.fully_qualified_name(),
        }
    }
}

#[cfg(test)]
impl<'a> Default for FieldDetail<'a, crate::util::Generic> {
    fn default() -> Self {
        let msg = Message::default();
        Self {
            msg: msg.clone().into(),
            name: Default::default(),
            fqn: Default::default(),
            syntax: Syntax::Proto3,
            is_map: false,
            in_oneof: false,
            util: RefCell::new(msg.util()),
            desc: FieldDescriptor::default(),
            comments: Default::default(),
        }
    }
}

#[cfg(test)]
impl<'a> Default for Field<'a, crate::util::Generic> {
    fn default() -> Self {
        let msg = Message::default();
        Self::Scalar(ScalarField::default())
    }
}

impl<'a, U> From<ScalarField<'a, U>> for Field<'a, U> {
    fn from(f: ScalarField<'a, U>) -> Self {
        Field::Scalar(f)
    }
}

impl<'a, U> From<EnumField<'a, U>> for Field<'a, U> {
    fn from(f: EnumField<'a, U>) -> Self {
        Field::Enum(f)
    }
}
impl<'a, U> From<MapField<'a, U>> for Field<'a, U> {
    fn from(f: MapField<'a, U>) -> Self {
        Field::Map(f)
    }
}
impl<'a, U> From<&ScalarField<'a, U>> for Field<'a, U> {
    fn from(f: &ScalarField<'a, U>) -> Self {
        f.clone().into()
    }
}

impl<'a, U> From<&EnumField<'a, U>> for Field<'a, U> {
    fn from(f: &EnumField<'a, U>) -> Self {
        f.clone().into()
    }
}
impl<'a, U> From<&MapField<'a, U>> for Field<'a, U> {
    fn from(f: &MapField<'a, U>) -> Self {
        f.clone().into()
    }
}
