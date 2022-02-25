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

use crate::{proto::Type, FullyQualified, IntoNode, Message, Name, Node, NodeAtPath};

pub(crate) type FieldList<'a, U> = Rc<RefCell<Vec<Field<'a, U>>>>;

#[derive(Debug)]
pub enum Field<'a, U> {
    Scalar(Rc<ScalarField<'a, U>>),
    Message(Rc<MessageField<'a, U>>),
    Map(Rc<MapField<'a, U>>),
    Repeated(Rc<RepeatedField<'a, U>>),
    Oneof(Rc<OneofField<'a, U>>),
    WellKnownType(Rc<WellKnownTypeField<'a, U>>),
}

impl<'a, U> Clone for Field<'a, U> {
    fn clone(&self) -> Self {
        match self {
            Field::Scalar(s) => Field::Scalar(Rc::clone(s)),
            Field::Message(m) => Field::Message(Rc::clone(m)),
            Field::Map(m) => Field::Map(Rc::clone(m)),
            Field::Repeated(r) => Field::Repeated(Rc::clone(r)),
            Field::Oneof(o) => Field::Oneof(Rc::clone(o)),
            Field::WellKnownType(w) => Field::WellKnownType(Rc::clone(w)),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FieldDetail<'a, U> {
    name: Name<U>,
    descriptor: &'a FieldDescriptorProto,
    fqn: String,
    msg: Weak<Message<'a, U>>,
}

impl<'a, U> FieldDetail<'a, U> {
    //     pub(crate) fn new(
    //         desc: &'a FieldDescriptorProto,
    //         msg: Message<'a, U>,
    //         util: Rc<RefCell<U>>,
    //     ) -> Rc<Self> {
    //         let field = Rc::new(Self {
    //             name: Name::new(desc.name(), util),
    //             descriptor: desc,
    //             fqn: fmt_fqn(msg.as_ref(), desc.name()),
    //             msg: Rc::downgrade(&msg),
    //             oneof: RefCell::new(Weak::new()),
    //             field_type: RefCell::new(None),
    //         });
    //         field
    //     }

    //     pub fn message(&self) -> Message<'a, U> {
    //         self.msg.upgrade().unwrap()
    //     }
    //     pub fn field_type(&self) -> Rc<FieldType<'a, U>> {
    //         self.field_type.borrow().as_ref().unwrap().clone()
    //     }

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
