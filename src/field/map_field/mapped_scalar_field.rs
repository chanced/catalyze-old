use std::rc::{Rc, Weak};

use crate::{
    descriptor::FieldDescriptor, proto::Syntax, FullyQualified, MapFieldDetail, Message, Name,
    Named, ScalarField,
};

#[derive(Debug, Clone)]
pub struct MappedScalarFieldDetail<'a, U> {
    detail: MapFieldDetail<'a, U>,
    scalar_field: ScalarField<'a, U>,
}

#[derive(Debug)]
pub struct MappedScalarField<'a, U>(Rc<MappedScalarFieldDetail<'a, U>>);
impl<'a, U> Clone for MappedScalarField<'a, U> {
    fn clone(&self) -> Self {
        MappedScalarField(self.0.clone())
    }
}

impl<'a, U> MappedScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
    pub fn is_repeated(&self) -> bool {
        self.0.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.0.detail.is_map()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.0.detail.util()
    }
    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.0.detail.descriptor()
    }
}

impl<'a, U> Named<U> for MappedScalarField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
}

impl<'a, U> FullyQualified for MappedScalarField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}

#[derive(Debug)]
pub(crate) struct WeakMappedScalarField<'a, U>(Weak<MappedScalarFieldDetail<'a, U>>);
impl<'a, U> Clone for WeakMappedScalarField<'a, U> {
    fn clone(&self) -> Self {
        WeakMappedScalarField(self.0.clone())
    }
}
