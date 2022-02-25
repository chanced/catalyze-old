use std::rc::{Rc, Weak};

use crate::{Field, FieldDetail, FullyQualified, Name};

#[derive(Debug, Clone)]
pub struct ScalarField<'a, U> {
    detail: FieldDetail<'a, U>,
    field: Weak<Field<'a, U>>,
}

impl<'a, U> ScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.upgrade().unwrap()
    }
}

impl<'a, U> FullyQualified for ScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}
