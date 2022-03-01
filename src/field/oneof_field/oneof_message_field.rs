use std::rc::{Rc, Weak};

use crate::{
    traits::{Downgrade, Upgrade},
    FullyQualified, Message, Name, Named, OneofFieldDetail, WeakMessage,
};

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
    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }
}

impl<'a, U> FullyQualified for OneofMessageField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for OneofMessageField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
}

impl<'a, U> Downgrade for OneofMessageField<'a, U> {
    type Output = WeakOneofMessageField<'a, U>;
    fn downgrade(self) -> Self::Output {
        WeakOneofMessageField(Rc::downgrade(&self.0))
    }
}

#[derive(Debug)]
pub(crate) struct WeakOneofMessageField<'a, U>(Weak<OneofMessageFieldDetail<'a, U>>);
impl<'a, U> Clone for WeakOneofMessageField<'a, U> {
    fn clone(&self) -> Self {
        WeakOneofMessageField(self.0.clone())
    }
}

impl<'a, U> Upgrade for WeakOneofMessageField<'a, U> {
    type Output = OneofMessageField<'a, U>;

    fn upgrade(self) -> Self::Output {
        OneofMessageField(
            self.0
                .upgrade()
                .expect("Failed to upgrade WeakOneofMessageField."),
        )
    }
}
