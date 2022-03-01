use std::rc::Rc;

use crate::{
    descriptor::FieldDescriptor, proto::Syntax, FullyQualified, Message, MessageFieldDetail, Name,
};

#[derive(Debug)]
pub struct RepeatedMessageField<'a, U>(Rc<MessageFieldDetail<'a, U>>);

impl<'a, U> RepeatedMessageField<'a, U> {
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
}

impl<'a, U> Clone for RepeatedMessageField<'a, U> {
    fn clone(&self) -> Self {
        RepeatedMessageField(self.0.clone())
    }
}

impl<'a, U> FullyQualified for RepeatedMessageField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}
