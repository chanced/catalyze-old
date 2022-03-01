use std::rc::{Rc, Weak};

use crate::{
    descriptor::FieldDescriptor,
    proto::{descriptor::Comments, Syntax},
    traits::Upgrade,
    EnumFieldDetail, FullyQualified, Message, Name,
};

#[derive(Debug, Clone)]
pub struct RepeatedEnumField<'a, U>(Rc<EnumFieldDetail<'a, U>>);

impl<'a, U> RepeatedEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
    pub fn is_repeated(&self) -> bool {
        self.0.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.0.detail.is_map()
    }
    pub fn container(&self) -> Message<'a, U> {
        self.0.detail.container()
    }
    pub fn util(&self) -> Rc<U> {
        self.0.detail.util()
    }
    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.0.detail.descriptor()
    }
    pub fn is_marked_optional(&self) -> bool {
        self.0.detail.is_marked_optional()
    }
    pub fn is_required(&self) -> bool {
        self.0.detail.is_required()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        self.0.detail.comments()
    }
}

impl<'a, U> FullyQualified for RepeatedEnumField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}

#[derive(Debug)]
pub(crate) struct WeakRepeatedEnumField<'a, U>(Weak<EnumFieldDetail<'a, U>>);

impl<'a, U> Clone for WeakRepeatedEnumField<'a, U> {
    fn clone(&self) -> Self {
        WeakRepeatedEnumField(self.0.clone())
    }
}

impl<'a, U> Upgrade for WeakRepeatedEnumField<'a, U> {
    type Output = RepeatedEnumField<'a, U>;
    fn upgrade(self) -> Self::Output {
        RepeatedEnumField(self.0.upgrade().unwrap())
    }
}
