mod enum_field;

pub mod descriptor;
mod map_field;
mod message_field;
mod oneof_field;
mod repeated_field;
mod scalar_field;
mod well_known_type_field;
pub use enum_field::*;
pub use map_field::*;
pub use message_field::*;
pub use oneof_field::*;
pub use repeated_field::*;
pub use scalar_field::*;

use std::{cell::RefCell, rc::Rc};
pub use well_known_type_field::*;

use crate::{
    proto::{Syntax, descriptor::Comments}, traits::Upgrade,  FullyQualified,  Message, Name, Node,
    NodeAtPath, SyntheticOneof, WeakMessage,

};

use self::descriptor::*;

pub(crate) type FieldList<'a, U> = Rc<RefCell<Vec<Field<'a, U>>>>;

#[derive(Debug)]
pub(crate) struct FieldDetail<'a, U> {
    msg: WeakMessage<'a, U>,
    name: Name<U>,
    fqn: String,
    syntax: Syntax,
    is_map: bool,
    is_repeated: bool,
    in_oneof: bool,
    util: Rc<U>,
    descriptor: FieldDescriptor<'a, U>,
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
            is_repeated: self.is_repeated,
            util: self.util.clone(),
            descriptor: self.descriptor.clone(),
            in_oneof: self.in_oneof,
            comments: self.comments.clone(),
        }
    }
}

fn example<T, R>(f: fn(T) -> R) -> R {
    
}


impl<'a, U> FieldDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.name.clone()
    }
    pub fn fully_qualified_name(&self) -> &str {
        &self.fqn
    }
    pub fn container(&self) -> Message<'a, U> {
        self.msg.upgrade()
    }
    pub fn containing_message(&self) -> Message<'a, U> {
        self.container()
    }
    pub fn util(&self) -> Rc<U> {
        self.util.clone()
    }
    pub fn syntax(&self) -> Syntax {
        
        self.syntax
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.descriptor.clone()
    }
    pub fn is_map(&self) -> bool {
        return self.is_map;
    }
    pub fn is_repeated(&self) -> bool {
        return self.descriptor.is_repeated();
    }
    pub fn is_scalar(&self) -> bool {
        return self.descriptor.is_scalar();
    }
    pub fn is_marked_optional(&self) -> bool {
        return self.descriptor.is_marked_optional(self.syntax);
    }
    pub fn is_required(&self) -> bool {
        return self.descriptor.is_required(self.syntax);
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
        if self.is
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
    WellKnownType(WellKnownTypeField<'a, U>),
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
impl<'a, U> From<MapScalarField<'a, U>> for Field<'a, U> {
    fn from(f: MapScalarField<'a, U>) -> Self {
        Field::Map(f.into())
    }
}
impl<'a, U> From<MapMessageField<'a, U>> for Field<'a, U> {
    fn from(f: MapMessageField<'a, U>) -> Self {
        Field::Map(f.into())
    }
}
impl<'a, U> From<MapEnumField<'a, U>> for Field<'a, U> {
    fn from(f: MapEnumField<'a, U>) -> Self {
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

impl<'a, U> From<WellKnownTypeField<'a, U>> for Field<'a, U> {
    fn from(f: WellKnownTypeField<'a, U>) -> Self {
        Field::WellKnownType(f)
    }
}
impl<'a, U> From<WellKnownMessageField<'a, U>> for Field<'a, U> {
    fn from(f: WellKnownMessageField<'a, U>) -> Self {
        Field::WellKnownType(f.into())
    }
}

impl<'a, U> From<WellKnownEnumField<'a, U>> for Field<'a, U> {
    fn from(f: WellKnownEnumField<'a, U>) -> Self {
        Field::WellKnownType(f.into())
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
impl<'a, U> From<&MapScalarField<'a, U>> for Field<'a, U> {
    fn from(f: &MapScalarField<'a, U>) -> Self {
        f.clone().into()
    }
}
impl<'a, U> From<&MapMessageField<'a, U>> for Field<'a, U> {
    fn from(f: &MapMessageField<'a, U>) -> Self {
        f.clone().into()
    }
}
impl<'a, U> From<&MapEnumField<'a, U>> for Field<'a, U> {
    fn from(f: &MapEnumField<'a, U>) -> Self {
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

impl<'a, U> From<&WellKnownTypeField<'a, U>> for Field<'a, U> {
    fn from(f: &WellKnownTypeField<'a, U>) -> Self {
        f.clone().into()
    }
}
impl<'a, U> From<&WellKnownMessageField<'a, U>> for Field<'a, U> {
    fn from(f: &WellKnownMessageField<'a, U>) -> Self {
        f.clone().into()
    }
}

impl<'a, U> From<&WellKnownEnumField<'a, U>> for Field<'a, U> {
    fn from(f: &WellKnownEnumField<'a, U>) -> Self {
        f.clone().into()
    }
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
            Field::WellKnownType(f) => f.name(),
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
            Field::WellKnownType(f) => f.fully_qualified_name(),
        }
    }
}
#[derive(Debug)]
pub(crate) enum WeakField<'a, U> {
    Scalar(WeakScalarField<'a, U>),
    Message(WeakMessageField<'a, U>),
    Map(WeakMapField<'a, U>),
    Repeated(WeakRepeatedField<'a, U>),
    Oneof(WeakOneofField<'a, U>),
    WellKnownType(WeakWellKnownTypeField<'a, U>),
}

impl<'a, U> Clone for WeakField<'a, U> {
    fn clone(&self) -> Self {
        match self {
            WeakField::Scalar(_) => todo!(),
            WeakField::Message(_) => todo!(),
            WeakField::Map(_) => todo!(),
            WeakField::Repeated(_) => todo!(),
            WeakField::Oneof(_) => todo!(),
            WeakField::WellKnownType(_) => todo!(),
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
            WeakField::WellKnownType(_) => todo!(),
        }
    }
}
