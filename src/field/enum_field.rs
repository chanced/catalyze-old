use std::rc::{Rc, Weak};

use crate::{traits::Upgrade, Field, Name, WeakEnum};

use super::{FieldDetail, WeakField};

#[derive(Debug, Clone)]
struct EnumFieldDetail<'a, U> {
    detail: FieldDetail<'a, U>,
    field: WeakField<'a, U>,
    r#enum: WeakEnum<'a, U>,
}

pub struct EnumField<'a, U>(Rc<EnumFieldDetail<'a, U>>);

impl<'a, U> EnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.0.field.upgrade()
    }
    pub fn fully_qualified_name(&self) -> Name<U> {
        self.0.detail.fully_qualified_name()
    }
}
