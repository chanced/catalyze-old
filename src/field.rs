mod enum_field;

pub mod descriptor;
mod map_field;
mod message_field;
mod oneof_field;
mod repeated_field;
mod scalar_field;

pub use enum_field::*;
pub use map_field::*;
pub use message_field::*;
pub use oneof_field::*;
pub use repeated_field::*;
pub use scalar_field::*;

use crate::{
    proto::{descriptor::Comments, Syntax},
    traits::Upgrade,
    FullyQualified, Message, Name, Node, NodeAtPath, SyntheticOneof, WeakMessage,
};
use std::{cell::RefCell, convert::From, rc::Rc};

use self::descriptor::*;

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
    desc: FieldDescriptor<'a, U>,
    comments: Comments<'a, U>,
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
    pub fn fully_qualified_name(&self) -> &str {
        &self.fqn
    }
    pub fn message(&self) -> Message<'a, U> {
        self.msg.upgrade()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.util.replace(util);
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.util.borrow().clone()
    }
    pub fn syntax(&self) -> Syntax {
        self.syntax
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.desc.clone()
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

    pub fn comments(&self) -> Comments<'a, U> {
        self.comments.clone()
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
        if self.desc {
            return true;
        }
    }
}

#[derive(Debug, Clone)]
pub enum Field<'a, U> {
    Enum(EnumField<'a, U>),
    Map(MapField<'a, U>),
    Message(MessageField<'a, U>),
    Oneof(OneofField<'a, U>),
    Repeated(RepeatedField<'a, U>),
    Scalar(ScalarField<'a, U>),
}

impl<'a, U> Field<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            Field::Enum(f) => f.name(),
            Field::Map(f) => f.name(),
            Field::Message(f) => f.name(),
            Field::Oneof(f) => f.name(),
            Field::Repeated(f) => f.name(),
            Field::Scalar(f) => f.name(),
        }
    }
}

impl<'a, U> NodeAtPath<'a, U> for Field<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            return Some(self.into());
        } else {
            None
        }
    }
}

impl<'a, U> FullyQualified for Field<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        match self {
            Field::Enum(_) => todo!(),
            Field::Map(f) => f.fully_qualified_name(),
            Field::Message(f) => f.fully_qualified_name(),
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
impl<'a, U> From<MappedScalarField<'a, U>> for Field<'a, U> {
    fn from(f: MappedScalarField<'a, U>) -> Self {
        Field::Map(f.into())
    }
}
impl<'a, U> From<MappedEmbedField<'a, U>> for Field<'a, U> {
    fn from(f: MappedEmbedField<'a, U>) -> Self {
        Field::Map(f.into())
    }
}
impl<'a, U> From<MappedEnumField<'a, U>> for Field<'a, U> {
    fn from(f: MappedEnumField<'a, U>) -> Self {
        Field::Map(f.into())
    }
}
impl<'a, U> From<MessageField<'a, U>> for Field<'a, U> {
    fn from(f: MessageField<'a, U>) -> Self {
        Field::Message(f)
    }
}
impl<'a, U> From<OneofField<'a, U>> for Field<'a, U> {
    fn from(f: OneofField<'a, U>) -> Self {
        Field::Oneof(f)
    }
}
impl<'a, U> From<RealOneofField<'a, U>> for Field<'a, U> {
    fn from(f: RealOneofField<'a, U>) -> Self {
        Field::Oneof(f.into())
    }
}
impl<'a, U> From<SyntheticOneof<'a, U>> for Field<'a, U> {
    fn from(f: SyntheticOneof<'a, U>) -> Self {
        Field::Oneof(f.into())
    }
}
impl<'a, U> From<RepeatedField<'a, U>> for Field<'a, U> {
    fn from(f: RepeatedField<'a, U>) -> Self {
        Field::Repeated(f)
    }
}
impl<'a, U> From<RepeatedMessageField<'a, U>> for Field<'a, U> {
    fn from(f: RepeatedMessageField<'a, U>) -> Self {
        Field::Repeated(f.into())
    }
}
impl<'a, U> From<RepeatedEnumField<'a, U>> for Field<'a, U> {
    fn from(f: RepeatedEnumField<'a, U>) -> Self {
        Field::Repeated(f.into())
    }
}

impl<'a, U> From<RepeatedScalarField<'a, U>> for Field<'a, U> {
    fn from(f: RepeatedScalarField<'a, U>) -> Self {
        Field::Repeated(f.into())
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
impl<'a, U> From<&MappedScalarField<'a, U>> for Field<'a, U> {
    fn from(f: &MappedScalarField<'a, U>) -> Self {
        f.clone().into()
    }
}
impl<'a, U> From<&MappedEmbedField<'a, U>> for Field<'a, U> {
    fn from(f: &MappedEmbedField<'a, U>) -> Self {
        f.clone().into()
    }
}
impl<'a, U> From<&MappedEnumField<'a, U>> for Field<'a, U> {
    fn from(f: &MappedEnumField<'a, U>) -> Self {
        f.clone().into()
    }
}
impl<'a, U> From<&MessageField<'a, U>> for Field<'a, U> {
    fn from(f: &MessageField<'a, U>) -> Self {
        f.clone().into()
    }
}
impl<'a, U> From<&OneofField<'a, U>> for Field<'a, U> {
    fn from(f: &OneofField<'a, U>) -> Self {
        f.clone().into()
    }
}
impl<'a, U> From<&RealOneofField<'a, U>> for Field<'a, U> {
    fn from(f: &RealOneofField<'a, U>) -> Self {
        f.clone().into()
    }
}
impl<'a, U> From<&SyntheticOneof<'a, U>> for Field<'a, U> {
    fn from(f: &SyntheticOneof<'a, U>) -> Self {
        f.clone().into()
    }
}
impl<'a, U> From<&RepeatedField<'a, U>> for Field<'a, U> {
    fn from(f: &RepeatedField<'a, U>) -> Self {
        f.clone().into()
    }
}
impl<'a, U> From<&RepeatedMessageField<'a, U>> for Field<'a, U> {
    fn from(f: &RepeatedMessageField<'a, U>) -> Self {
        f.clone().into()
    }
}
impl<'a, U> From<&RepeatedEnumField<'a, U>> for Field<'a, U> {
    fn from(f: &RepeatedEnumField<'a, U>) -> Self {
        f.clone().into()
    }
}

impl<'a, U> From<&RepeatedScalarField<'a, U>> for Field<'a, U> {
    fn from(f: &RepeatedScalarField<'a, U>) -> Self {
        f.clone().into()
    }
}

#[derive(Debug)]
pub(crate) enum WeakField<'a, U> {
    Scalar(WeakScalarField<'a, U>),
    Message(WeakMessageField<'a, U>),
    Map(WeakMapField<'a, U>),
    Repeated(WeakRepeatedField<'a, U>),
    Oneof(WeakOneofField<'a, U>),
}

impl<'a, U> Clone for WeakField<'a, U> {
    fn clone(&self) -> Self {
        match self {
            WeakField::Scalar(_) => todo!(),
            WeakField::Message(_) => todo!(),
            WeakField::Map(_) => todo!(),
            WeakField::Repeated(_) => todo!(),
            WeakField::Oneof(_) => todo!(),
        }
    }
}

impl<'a, U> Upgrade for WeakField<'a, U> {
    type Output = Field<'a, U>;
    fn upgrade(self) -> Self::Output {
        match self {
            WeakField::Scalar(f) => Field::Scalar(f.upgrade()),
            WeakField::Message(f) => Field::Message(f.upgrade()),
            WeakField::Map(f) => Field::Map(f.upgrade()),
            WeakField::Repeated(_) => todo!(),
            WeakField::Oneof(_) => todo!(),
        }
    }
}
