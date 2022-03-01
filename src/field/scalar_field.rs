use std::rc::{Rc, Weak};

use super::{FieldDetail, MapScalarFieldDetail, WeakField};
use crate::{
    descriptor::{FieldDescriptor, Scalar},
    proto::Syntax,
    util::Util,
    FullyQualified, Message, Name, Named,
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
            is_repeated: self.is_repeated,
        }
    }
}

impl<'a, U> ScalarFieldDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.detail.fully_qualified_name()
    }
    pub fn scalar(&self) -> &Scalar {
        &self.scalar
    }
    pub fn is_repeated(&self) -> bool {
        self.detail.is_repeated
    }
    pub fn is_map(&self) -> bool {
        self.detail.is_map
    }
    pub fn container(&self) -> Message<'a, U> {
        self.detail.container()
    }
    pub fn util(&self) -> Util<U> {
        self.detail.util()
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
    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for ScalarField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
}
impl<'a, U> FullyQualified for ScalarField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}
#[derive(Debug, Clone)]
pub(crate) struct WeakScalarField<'a, U>(Weak<MapScalarFieldDetail<'a, U>>);
