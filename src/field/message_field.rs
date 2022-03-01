use std::rc::{Rc, Weak};

use crate::{
    descriptor::FieldDescriptor, proto::Syntax, FullyQualified, Message, Name, WeakMessage,
};

use super::FieldDetail;

#[derive(Debug)]
pub(crate) struct MessageFieldDetail<'a, U> {
    pub detail: FieldDetail<'a, U>,
    pub message: WeakMessage<'a, U>,
}
impl<'a, U> MessageFieldDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.detail.fully_qualified_name()
    }
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

impl<'a, U> Clone for MessageFieldDetail<'a, U> {
    fn clone(&self) -> Self {
        Self {
            detail: self.detail.clone(),
            message: self.message.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MessageField<'a, U>(Rc<MessageFieldDetail<'a, U>>);

impl<'a, U> MessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name.clone()
    }
}
impl<'a, U> FullyQualified for MessageField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        &self.0.detail.fqn
    }
}
#[derive(Debug)]
pub(crate) struct WeakMessageField<'a, U>(Weak<MessageFieldDetail<'a, U>>);
impl<'a, U> Clone for WeakMessageField<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
