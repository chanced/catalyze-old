mod enum_field;

pub mod descriptor;
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

use std::{cell::RefCell, rc::Rc};
pub use wkt_field::*;

use crate::{
    proto::Syntax, traits::Upgrade, util::Util, FullyQualified, IntoNode, Message, Name, Node,
    NodeAtPath, WeakMessage,
};

use self::descriptor::*;

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
    Enum(EnumField<'a, U>),
    Map(MapField<'a, U>),
    Message(MessageField<'a, U>),
    Oneof(OneofField<'a, U>),
    Repeated(RepeatedField<'a, U>),
    Scalar(ScalarField<'a, U>),
    WellKnownType(WellKnownTypeField<'a, U>),
}

#[derive(Debug, Clone)]
pub(crate) struct FieldDetail<'a, U> {
    name: Name<U>,
    descriptor: FieldDescriptor<'a, U>,
    fqn: String,
    containing_msg: WeakMessage<'a, U>,
    util: Util<U>,
    syntax: Syntax,
}

impl<'a, U> FieldDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.name.clone()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
    pub fn container(&self) -> Message<'a, U> {
        self.containing_message()
    }
    pub fn containing_message(&self) -> Message<'a, U> {
        self.msg.upgrade()
    }
    pub fn util(&self) -> Util<U> {
        self.util.clone()
    }
    pub fn syntax(&self) -> Syntax {
        self.syntax
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.descriptor.clone()
    }

    pub fn is_map(&self) -> bool {
        return self.descriptor.is_map();
    }

    pub fn is_repeated(&self) -> bool {
        return self.descriptor.is_repeated();
    }

    pub fn is_scalar(&self) -> bool {
        return self.descriptor.is_scalar();
    }

    pub fn is_optional(&self) -> bool {
        return self.descriptor.is_optional(self.syntax);
    }

    pub fn is_required(&self) -> bool {
        return self.descriptor.is_required(self.syntax);
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
            return Some(self.into_node());
        } else {
            None
        }
    }
}

impl<'a, U> FullyQualified for Field<'a, U> {
    fn fully_qualified_name(&self) -> String {
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
    type Target = Field<'a, U>;
    fn upgrade(self) -> Self::Target {
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
