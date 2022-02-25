use std::rc::{Rc, Weak};

use crate::{name::Named, Field, FieldDetail, FullyQualified, Name};

#[derive(Debug, Clone)]
pub enum RepeatedField<'a, U> {
    Scalar(Rc<RepeatedScalarField<'a, U>>),
    Enum(Rc<RepeatedEnumField<'a, U>>),
    Message(Rc<RepeatedMessageField<'a, U>>),
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
pub struct RepeatedEnumField<'a, U> {
    detail: FieldDetail<'a, U>,
    repeated_field: Weak<RepeatedField<'a, U>>,
    field: Weak<Field<'a, U>>,
}

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

#[derive(Debug, Clone)]
pub struct RepeatedMessageField<'a, U> {
    detail: FieldDetail<'a, U>,
    repeated_field: Rc<RepeatedField<'a, U>>,
    field: Field<'a, U>,
}

impl<'a, U> RepeatedMessageField<'a, U> {
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
        self.repeated_field.clone()
    }
}

impl<'a, U> FullyQualified for RepeatedMessageField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}

#[derive(Debug, Clone)]
pub struct RepeatedScalarField<'a, U> {
    detail: FieldDetail<'a, U>,
    repeated_field: Weak<RepeatedField<'a, U>>,
    field: Field<'a, U>,
}
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
