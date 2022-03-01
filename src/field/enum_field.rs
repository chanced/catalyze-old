use std::rc::{Rc, Weak};

use crate::{
    descriptor::FieldDescriptor, proto::Syntax, traits::Upgrade, Enum, Field, FieldDetail,
    FullyQualified, Message, Name, Named, WeakEnum,
};

#[derive(Debug, Clone)]
pub(crate) struct EnumFieldDetail<'a, U> {
    pub detail: FieldDetail<'a, U>,
    pub r#enum: WeakEnum<'a, U>,
}

impl<'a, U> EnumFieldDetail<'a, U> {
    pub fn is_repeated(&self) -> bool {
        self.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.detail.is_map()
    }
    pub fn container(&self) -> Message<'a, U> {
        self.detail.container()
    }
    pub fn util(&self) -> Rc<U> {
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
pub struct EnumField<'a, U>(Rc<EnumFieldDetail<'a, U>>);

impl<'a, U> EnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
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
    fn fully_qualified_name(&self) -> &str {
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
    type Output = EnumField<'a, U>;

    fn upgrade(self) -> Self::Output {
        EnumField(self.0.upgrade().expect("EnumField upgrade failed"))
    }
}
