mod enum_field;
mod map_field;
mod message_field;
mod oneof_field;
mod repeated_field;
mod scalar_field;
mod wkt_field;
pub use enum_field::*;
pub use map_field::*;
pub use message_field::*;
pub use oneof_field::*;
pub use repeated_field::*;
pub use scalar_field::*;
pub use wkt_field::*;

use prost_types::FieldDescriptorProto;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    proto::Type, traits::Upgrade, FullyQualified, IntoNode, Message, Name, Node, NodeAtPath,
};

pub(crate) type FieldList<'a, U> = Rc<RefCell<Vec<Field<'a, U>>>>;

pub enum FieldType {
    Scalar,
    Message,
    Map,
    Repeated,
    WellKnownType,
}

#[derive(Debug, Clone)]
pub enum Field<'a, U> {
    Scalar(ScalarField<'a, U>),
    Message(MessageField<'a, U>),
    Map(MapField<'a, U>),
    Repeated(RepeatedField<'a, U>),
    Oneof(OneofField<'a, U>),
    WellKnownType(WellKnownTypeField<'a, U>),
}

#[derive(Debug, Clone)]
pub(crate) struct FieldDetail<'a, U> {
    name: Name<U>,
    descriptor: &'a FieldDescriptorProto,
    fqn: String,
    msg: Weak<Message<'a, U>>,
}

impl<'a, U> FieldDetail<'a, U> {
    pub fn proto_type(&self) -> Type {
        self.descriptor.r#type()
    }

    pub fn name(&self) -> Name<U> {
        self.name.clone()
    }
}

impl<'a, U> Field<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            Field::Scalar(f) => f.name(),
            Field::Message(f) => f.name(),
            Field::Map(f) => f.name(),
            Field::Repeated(f) => f.name(),
            Field::Oneof(f) => f.name(),
            Field::WellKnownType(f) => f.name(),
        }
    }
}

impl<'a, U> NodeAtPath<'a, U> for Field<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            return Some(self.into_node());
        } else {
            None
        }
    }
}

impl<'a, U> FullyQualified for FieldDetail<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
}

impl<'a, U> FullyQualified for Field<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            Field::Scalar(f) => f.fully_qualified_name(),
            Field::Message(f) => f.fully_qualified_name(),
            Field::Map(f) => f.fully_qualified_name(),
            Field::Repeated(f) => f.fully_qualified_name(),
            Field::Oneof(f) => f.fully_qualified_name(),
            Field::WellKnownType(f) => f.fully_qualified_name(),
        }
    }
}

pub(crate) struct WeakField<'a, U>(Weak<FieldDetail<'a, U>>);

enum WeakField<'a, U> {
    Scalar(Weak<ScalarFieldDetail<'a, U>>),
    Message(Weak<MessageFieldDetail<'a, U>>),
    Map(Weak<MapFieldDetail<'a, U>>),
}
