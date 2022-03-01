use std::rc::Rc;

use crate::{FullyQualified, MessageFieldDetail, Name};

#[derive(Debug)]
pub struct RepeatedMessageField<'a, U>(Rc<MessageFieldDetail<'a, U>>);

impl<'a, U> RepeatedMessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.0.fully_qualified_name()
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
