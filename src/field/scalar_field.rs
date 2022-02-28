use std::rc::{Rc, Weak};

use super::{FieldDetail, MapScalarFieldDetail, WeakField};
use crate::{descriptor::Scalar, FullyQualified, Name, Named};

#[derive(Debug)]
pub(crate) struct ScalarFieldDetail<'a, U> {
    detail: FieldDetail<'a, U>,
    field: WeakField<'a, U>,
    scalar: Scalar,
}

impl<'a, U> Clone for ScalarFieldDetail<'a, U> {
    fn clone(&self) -> Self {
        Self {
            detail: self.detail.clone(),
            field: self.field.clone(),
            scalar: self.scalar.clone(),
        }
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
