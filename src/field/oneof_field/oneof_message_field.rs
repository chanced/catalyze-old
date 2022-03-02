use std::rc::Rc;

use crate::{FullyQualified, Message, Name, OneofFieldDetail, WeakMessage};

#[derive(Debug, Clone)]
pub(crate) struct OneofMessageFieldDetail<'a, U> {
    detail: OneofFieldDetail<'a, U>,
    message: WeakMessage<'a, U>,
}

#[derive(Debug)]
pub struct OneofMessageField<'a, U>(Rc<OneofMessageFieldDetail<'a, U>>);
impl<'a, U> Clone for OneofMessageField<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<'a, U> OneofMessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }
}

impl<'a, U> FullyQualified for OneofMessageField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}
