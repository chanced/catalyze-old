use std::rc::Rc;

use super::{FieldDetail, MapScalarFieldDetail, WeakField};
use crate::{Field, FullyQualified, Name};

#[derive(Debug, Clone)]
pub(crate) struct ScalarFieldDetail<'a, U> {
    detail: FieldDetail<'a, U>,
    field: WeakField<'a, U>,
}

#[derive(Debug)]
pub struct ScalarField<'a, U>(Rc<MapScalarFieldDetail<'a, U>>);

impl<'a, U> Clone for ScalarField<'a, U> {
    fn clone(&self) -> Self {
        ScalarField(self.0.clone())
    }
}

impl<'a, U> ScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.upgrade()
    }
}

impl<'a, U> FullyQualified for ScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}
