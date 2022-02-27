use std::rc::{Rc, Weak};

use crate::{traits::Upgrade, Enum, Field, FullyQualified, Name, Named, WeakEnum};

use super::{FieldDetail, WeakField};

#[derive(Debug, Clone)]
struct EnumFieldDetail<'a, U> {
    detail: FieldDetail<'a, U>,
    r#enum: WeakEnum<'a, U>,
}

#[derive(Debug)]
pub struct EnumField<'a, U>(Rc<EnumFieldDetail<'a, U>>);

impl<'a, U> EnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn field(&self) -> Field<'a, U> {
        Field::Enum(self.clone())
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn r#enum(&self) -> Enum<'a, U> {
        self.0.r#enum.upgrade()
    }
    /// alias for r#enum
    pub fn enum_value(&self) -> Enum<'a, U> {
        self.0.r#enum.upgrade()
    }
}

impl<'a, U> Named<U> for EnumField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
}
impl<'a, U> FullyQualified for EnumField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}

impl<'a, U> Clone for EnumField<'a, U> {
    fn clone(&self) -> Self {
        EnumField(self.0.clone())
    }
}
pub(crate) struct WeakEnumField<'a, U>(Weak<EnumFieldDetail<'a, U>>);
impl<'a, U> Upgrade for WeakEnumField<'a, U> {
    type Target = EnumField<'a, U>;

    fn upgrade(self) -> Self::Target {
        EnumField(self.0.upgrade().expect("EnumField upgrade failed"))
    }
}
