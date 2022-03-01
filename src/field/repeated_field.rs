use std::rc::{Rc, Weak};
mod repeated_enum_field;
mod repeated_message_field;
mod repeated_scalar_field;
pub use repeated_enum_field::*;
pub use repeated_message_field::*;
pub use repeated_scalar_field::*;

use crate::{name::Named, proto::Syntax, util::Util, FieldDetail, FullyQualified, Message, Name};

use super::descriptor::FieldDescriptor;

pub(crate) struct RepeatedFieldDetail<'a, U> {
    pub detail: FieldDetail<'a, U>,
}

/// Represents a field marked as `repeated`. The field can hold
/// a scalar value, an enum, or a message.
#[derive(Debug, Clone)]
pub enum RepeatedField<'a, U> {
    Scalar(RepeatedScalarField<'a, U>),
    Enum(RepeatedEnumField<'a, U>),
    Message(RepeatedMessageField<'a, U>),
}

impl<'a, U> RepeatedField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            RepeatedField::Scalar(f) => f.name(),
            RepeatedField::Enum(f) => f.name(),
            RepeatedField::Message(f) => f.name(),
        }
    }
    fn fully_qualified_name(&self) -> &str {
        match self {
            RepeatedField::Scalar(f) => f.fully_qualified_name(),
            RepeatedField::Enum(f) => f.fully_qualified_name(),
            RepeatedField::Message(f) => f.fully_qualified_name(),
        }
    }
    pub fn containing_message(&self) -> Message<'a, U> {
        self.container()
    }

    pub fn container(&self) -> Message<'a, U> {
        match self {
            RepeatedField::Scalar(f) => f.container(),
            RepeatedField::Enum(f) => f.container(),
            RepeatedField::Message(f) => f.container(),
        }
    }

    pub fn util(&self) -> Util<U> {
        match self {
            RepeatedField::Scalar(f) => f.util(),
            RepeatedField::Enum(f) => f.util(),
            RepeatedField::Message(f) => f.util(),
        }
    }
    pub fn syntax(&self) -> Syntax {
        match self {
            RepeatedField::Scalar(f) => f.syntax(),
            RepeatedField::Enum(f) => f.syntax(),
            RepeatedField::Message(f) => f.syntax(),
        }
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        match self {
            RepeatedField::Scalar(f) => f.descriptor(),
            RepeatedField::Enum(f) => f.descriptor(),
            RepeatedField::Message(f) => f.descriptor(),
        }
    }

    pub fn is_map(&self) -> bool {
        false
    }

    pub fn is_repeated(&self) -> bool {
        true
    }

    pub fn is_scalar(&self) -> bool {
        match self {
            RepeatedField::Scalar(_) => true,
            _ => false,
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            RepeatedField::Scalar(f) => f.is_optional(),
            RepeatedField::Enum(f) => f.is_optional(),
            RepeatedField::Message(f) => f.is_optional(),
        }
    }
}

impl<'a, U> Named<U> for RepeatedField<'a, U> {
    fn name(&self) -> Name<U> {
        match self {
            RepeatedField::Scalar(f) => f.name(),
            RepeatedField::Enum(f) => f.name(),
            RepeatedField::Message(f) => f.name(),
        }
    }
}
impl<'a, U> FullyQualified for RepeatedField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        match self {
            RepeatedField::Scalar(f) => f.fully_qualified_name(),
            RepeatedField::Enum(f) => f.fully_qualified_name(),
            RepeatedField::Message(f) => f.fully_qualified_name(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum WeakRepeatedField<'a, U> {
    Scalar(WeakRepeatedScalarField<'a, U>),
    Enum(WeakRepeatedEnumField<'a, U>),
    Message(RepeatedMessageField<'a, U>),
}

pub(crate) struct WeakRepeatedEnumField<'a, U>(Weak<RepeatedScalarField<'a, U>>);
