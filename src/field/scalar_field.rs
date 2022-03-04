use std::rc::Rc;

use super::FieldDetail;
use crate::{
    proto::Syntax,
    proto::{FieldDescriptor, Scalar},
    FullyQualified, Message, Name,
};

#[derive(Debug)]
pub(crate) struct ScalarFieldDetail<'a, U> {
    detail: FieldDetail<'a, U>,
    scalar: Scalar,
}

impl<'a, U> Clone for ScalarFieldDetail<'a, U> {
    fn clone(&self) -> Self {
        Self {
            detail: self.detail.clone(),
            scalar: self.scalar.clone(),
        }
    }
}

impl<'a, U> ScalarFieldDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn scalar(&self) -> &Scalar {
        &self.scalar
    }
    pub fn is_repeated(&self) -> bool {
        self.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.detail.is_map()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.detail.message()
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.detail.util()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.detail.util.replace(util);
    }
    pub fn syntax(&self) -> Syntax {
        self.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.detail.descriptor()
    }
}

#[derive(Debug)]
pub struct ScalarField<'a, U>(Rc<ScalarFieldDetail<'a, U>>);

impl<'a, U> Clone for ScalarField<'a, U> {
    fn clone(&self) -> Self {
        ScalarField(self.0.clone())
    }
}

impl<'a, U> ScalarField<'a, U> {
    pub fn scalar(&self) -> Scalar {
        self.0.scalar
    }
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}
impl<'a, U> FullyQualified for ScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}
// #[derive(Debug, Clone)]
// pub(crate) struct WeakScalarField<'a, U>(Weak<MappedScalarFieldDetail<'a, U>>);
