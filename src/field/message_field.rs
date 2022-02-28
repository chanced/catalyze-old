use std::rc::{Rc, Weak};

use crate::{FullyQualified, Message, Name, WeakMessage};

use super::FieldDetail;

#[derive(Debug, Clone)]
pub(crate) struct MessageFieldDetail<'a, U> {
    pub detail: FieldDetail<'a, U>,
    pub message: Message<'a, U>,
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
