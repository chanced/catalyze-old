use std::rc::Rc;

use crate::{
    proto::FieldDescriptor, proto::Syntax, Enum, FieldDetail, FullyQualified, Message, Name,
    WeakEnum,
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
pub struct EnumField<'a, U>(Rc<EnumFieldDetail<'a, U>>);

impl<'a, U> EnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    /// Returns the `Enum` of this `EnumField`.
    pub fn r#enum(&self) -> Enum<'a, U> {
        self.0.r#enum.clone().into()
    }
    /// alias for `r#enum`
    ///
    /// Returns the `Enum` of this `EnumField`.
    pub fn enumeration(&self) -> Enum<'a, U> {
        self.r#enum()
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
