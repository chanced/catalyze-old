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
    proto::{FieldDescriptor, Location, Syntax},
    Comments, FullyQualified, Message, Name, Node, NodeAtPath, WeakMessage,
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
    desc: dyn FieldDescriptor<'a, U>,
    comments: RefCell<Comments<'a, U>>,
}
impl<'a, U> Clone for FieldDetail<'a, U> {
    fn clone(&self) -> Self {
        Self {
            msg: self.msg.clone(),
            name: self.name.clone(),
            fqn: self.fqn.clone(),
            syntax: self.syntax.clone(),
            is_map: self.is_map,
            util: self.util.clone(),
            desc: self.desc.clone(),
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
    pub fn descriptor(&self) -> dyn FieldDescriptor<'a, U> {
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
    pub fn is_required(&self) -> bool {
        self.desc.is_required(self.syntax)
    }

    pub fn is_well_known_type(&self) -> bool {
        self.desc.is_well_known_type()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.comments.replace(comments);
    }
    pub fn comments(&self) -> Comments<'a, U> {
        *self.comments.borrow()
    }
    /// Returns `true` for all fields that have explicit presence.
    ///
    /// See:
    /// - https://github.com/protocolbuffers/protobuf/blob/v3.17.0/docs/field_presence.md
    /// - https://github.com/protocolbuffers/protobuf/blob/master/docs/implementing_proto3_presence.md
    pub fn has_presence(&self) -> bool {
        if self.in_oneof {
            return true;
        }
        if self.desc.is_embed() {
            return true;
        }
        if !self.is_repeated() && !self.is_map() {
            if self.syntax.is_proto2() {
                true
            } else {
                self.desc.is_marked_optional(self.syntax)
            }
        } else {
            false
        }
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
