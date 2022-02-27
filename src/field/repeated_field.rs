use std::rc::{Rc, Weak};

use crate::{name::Named, proto::Syntax, util::Util, Field, FullyQualified, Message, Name};

use super::{descriptor::FieldDescriptor, EnumFieldDetail, MessageFieldDetail, ScalarFieldDetail};

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
    pub fn field(&self) -> Field<'a, U> {
        match self {
            RepeatedField::Scalar(s) => s.field(),
            RepeatedField::Enum(e) => e.field(),
            RepeatedField::Message(m) => m.field(),
        }
    }
    fn fully_qualified_name(&self) -> String {
        match self {
            RepeatedField::Scalar(f) => f.fully_qualified_name(),
            RepeatedField::Enum(f) => f.fully_qualified_name(),
            RepeatedField::Message(f) => f.fully_qualified_name(),
        }
    }
    pub fn containing_message(&self) -> Message<'a, U> {
        match self {
            RepeatedField::Scalar(f) => f.containing_message(),
            RepeatedField::Enum(f) => f.containing_message(),
            RepeatedField::Message(f) => f.containing_message(),
        }
    }

    pub fn container(&self) -> Message<'a, U> {
        self.containing_message()
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
    fn fully_qualified_name(&self) -> String {
        match self {
            RepeatedField::Scalar(f) => f.fully_qualified_name(),
            RepeatedField::Enum(f) => f.fully_qualified_name(),
            RepeatedField::Message(f) => f.fully_qualified_name(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RepeatedEnumField<'a, U>(EnumFieldDetail<'a, U>);

impl<'a, U> RepeatedEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.upgrade().unwrap()
    }
    pub fn repeated_field(&self) -> Rc<RepeatedField<'a, U>> {
        self.repeated_field.upgrade().unwrap()
    }
}

impl<'a, U> FullyQualified for RepeatedEnumField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}

#[derive(Debug)]
pub struct RepeatedMessageField<'a, U>(MessageFieldDetail<'a, U>);

impl<'a, U> RepeatedMessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.0.field()
    }
    pub fn repeated_field(&self) -> Rc<RepeatedField<'a, U>> {
        self.repeated_field.clone()
    }
}

impl<'a, U> Clone for RepeatedMessageField<'a, U> {
    fn clone(&self) -> Self {
        RepeatedMessageField(self.0.clone())
    }
}

impl<'a, U> FullyQualified for RepeatedMessageField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RepeatedScalarFieldDetail<'a, U> {
    detail: ScalarFieldDetail<'a, U>,
}

#[derive(Debug)]
pub(crate) struct RepeatedScalarField<'a, U>(Rc<RepeatedScalarFieldDetail<'a, U>>);

impl<'a, U> RepeatedScalarField<'a, U> {}

impl<'a, U> RepeatedScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.clone()
    }
    pub fn repeated_field(&self) -> Rc<RepeatedField<'a, U>> {
        self.repeated_field.upgrade().unwrap()
    }
}

impl<'a, U> FullyQualified for RepeatedScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for RepeatedScalarField<'a, U> {
    fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
impl<'a, U> Clone for RepeatedScalarField<'a, U> {
    fn clone(&self) -> Self {
        RepeatedScalarField(self.0.clone())
    }
}
#[derive(Debug, Clone)]
pub(crate) enum WeakRepeatedField<'a, U> {
    Scalar(WeakRepeatedScalarField<'a, U>),
    Enum(WeakRepeatedEnumField<'a, U>),
    Message(RepeatedMessageField<'a, U>),
}

pub(crate) struct WeakRepeatedScalarField<'a, U>(Weak<RepeatedScalarField<'a, U>>);
