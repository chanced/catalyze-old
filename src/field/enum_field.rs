use std::rc::{Rc, Weak};

use crate::{Field, FieldDetail, Name};

#[derive(Debug, Clone)]
pub struct EnumField<'a, U> {
    detail: FieldDetail<'a, U>,
    field: Weak<Field<'a, U>>,
}
impl<'a, U> EnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.upgrade().unwrap()
    }
    pub fn fully_qualified_name(&self) -> Name<U> {
        self.detail.name()
    }
}
