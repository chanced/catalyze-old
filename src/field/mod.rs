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
    visit::{Accept, Visitor},
    FullyQualified, Message, Name, Node, NodeAtPath,
};

pub(crate) type FieldList<'a, U> = Rc<RefCell<Vec<Rc<Field<'a, U>>>>>;

#[derive(Clone, Debug)]
pub enum Field<'a, U> {
    Scalar(ScalarField<'a, U>),
    Message(MessageField<'a, U>),
    Map(MapField<'a, U>),
    Repeated(RepeatedField<'a, U>),
    Oneof(OneofField<'a, U>),
    WellKnownType(WellKnownTypeField<'a, U>),
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
    //         msg: Rc<Message<'a, U>>,
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
    pub fn name(&self) -> Name<U> {
        self.name.clone()
    }

    //     pub fn message(&self) -> Rc<Message<'a, U>> {
    //         self.msg.upgrade().unwrap()
    //     }
    //     pub fn field_type(&self) -> Rc<FieldType<'a, U>> {
    //         self.field_type.borrow().as_ref().unwrap().clone()
    //     }

    //     pub fn is_map(&self) -> bool {
    //         self.descriptor.r#type() == field_descriptor_proto::Type::Message
    //             && self.descriptor.label() == field_descriptor_proto::Label::Repeated
    //     }

    //     pub fn is_enum(&self) -> bool {
    //         self.descriptor.r#type() == field_descriptor_proto::Type::Enum
    //     }

    //     pub fn is_repeated(&self) -> bool {
    //         !self.is_map() && self.descriptor.label() == field_descriptor_proto::Label::Repeated
    //     }

    //     pub fn proto_type(&self) -> Type {
    //         self.descriptor.r#type()
    //     }

    //     pub fn oneof(&self) -> Option<Rc<Oneof<'a, U>>> {
    //         self.oneof.borrow().upgrade()
    //     }

    //     pub(crate) fn assign_oneof(&self, oneof: Rc<Oneof<'a, U>>) {
    //         self.oneof.replace(Rc::downgrade(&oneof));
    //     }
}
impl<'a, U> NodeAtPath<'a, U> for Rc<Field<'a, U>> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            Some(Node::Field(self.clone()))
        } else {
            None
        }
    }
}

impl<'a, U, V: Visitor<'a, U>> Accept<'a, U, V> for Field<'a, U> {
    fn accept(&self, visitor: &mut V) -> Result<(), V::Error> {
        todo!();
        // visitor.visit_field(self.clone())
    }
}

impl<'a, U> FullyQualified for FieldDetail<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
}

// #[derive(Clone)]
// pub struct FieldIter<'a, U> {
//     fields: FieldList<'a, U>,
//     idx: usize,
// }

// impl<'a, U> Iterator for FieldIter<'a, U> {
//     type Item = Field<'a, U>;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.idx += 1;
//         self.fields.borrow().get(self.idx - 1).cloned()
//     }
// }
// impl<'a, U> From<&'a FieldList<'a, U>> for FieldIter<'a, U> {
//     fn from(fields: &'a FieldList<'a, U>) -> Self {
//         Self {
//             fields: fields.clone(),
//             idx: 0,
//         }
//     }
// }
