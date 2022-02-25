use std::rc::{Rc, Weak};

use crate::{name::Named, Field, FieldDetail, FullyQualified, Name};

#[derive(Debug, Clone)]
pub enum OneofField<'a, U> {
    Scalar(Rc<OneofScalarField<'a, U>>),
    Enum(Rc<OneofEnumField<'a, U>>),
    Message(Rc<OneofMessageField<'a, U>>),
}

impl<'a, U> OneofField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            OneofField::Scalar(s) => s.name(),
            OneofField::Enum(e) => e.name(),
            OneofField::Message(m) => m.name(),
        }
    }
    pub fn fully_qualified_name(&self) -> String {
        match self {
            OneofField::Scalar(s) => s.fully_qualified_name(),
            OneofField::Enum(e) => e.fully_qualified_name(),
            OneofField::Message(m) => m.fully_qualified_name(),
        }
    }
}
impl<'a, U> FullyQualified for OneofField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            OneofField::Scalar(s) => s.fully_qualified_name(),
            OneofField::Enum(e) => e.fully_qualified_name(),
            OneofField::Message(m) => m.fully_qualified_name(),
        }
    }
}

impl<'a, U> Named<U> for OneofField<'a, U> {
    fn name(&self) -> Name<U> {
        match self {
            OneofField::Scalar(s) => s.name(),
            OneofField::Enum(e) => e.name(),
            OneofField::Message(m) => m.name(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OneofScalarField<'a, U> {
    detail: FieldDetail<'a, U>,
    field: Weak<Field<'a, U>>,
    oneof_field: Weak<OneofField<'a, U>>,
}

impl<'a, U> OneofScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.upgrade().unwrap()
    }
    pub fn oneof_field(&self) -> Rc<OneofField<'a, U>> {
        self.oneof_field.upgrade().unwrap()
    }
}
impl<'a, U> FullyQualified for OneofScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for OneofScalarField<'a, U> {
    fn name(&self) -> Name<U> {
        self.detail.name()
    }
}

#[derive(Debug, Clone)]
pub struct OneofMessageField<'a, U> {
    detail: FieldDetail<'a, U>,
    oneof_field: Weak<OneofField<'a, U>>,
    field: Weak<Field<'a, U>>,
}

impl<'a, U> OneofMessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.upgrade().unwrap()
    }
    pub fn oneof_field(&self) -> Rc<OneofField<'a, U>> {
        self.oneof_field.upgrade().unwrap()
    }
}

impl<'a, U> FullyQualified for OneofMessageField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for OneofMessageField<'a, U> {
    fn name(&self) -> Name<U> {
        self.detail.name()
    }
}

#[derive(Debug, Clone)]
pub struct OneofEnumField<'a, U> {
    detail: FieldDetail<'a, U>,
    oneof_field: Weak<OneofField<'a, U>>,
    field: Weak<Field<'a, U>>,
}

impl<'a, U> OneofEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.upgrade().unwrap()
    }
    pub fn oneof_field(&self) -> Rc<OneofField<'a, U>> {
        self.oneof_field.upgrade().unwrap()
    }
}
impl<'a, U> FullyQualified for OneofEnumField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for OneofEnumField<'a, U> {
    fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
