use std::rc::{Rc, Weak};

use crate::{
    traits::{Downgrade, Upgrade},
    FullyQualified, Message, Name, Named, OneofFieldDetail, ScalarField,
};

#[derive(Debug, Clone)]
pub struct OneofEnumFieldDetail<'a, U> {
    detail: OneofFieldDetail<'a, U>,
    scalar: ScalarField<'a, U>,
}
#[derive(Debug)]
pub struct OneofEnumField<'a, U>(Rc<OneofEnumFieldDetail<'a, U>>);

impl<'a, U> OneofEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
    pub fn container(&self) -> Message<'a, U> {
        self.0.detail.container()
    }
    pub fn containing_message(&self) -> Message<'a, U> {
        self.container()
    }
}
impl<'a, U> FullyQualified for OneofEnumField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for OneofEnumField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
}
impl<'a, U> Clone for OneofEnumField<'a, U> {
    fn clone(&self) -> Self {
        OneofEnumField(self.0.clone())
    }
}
impl<'a, U> Downgrade for OneofEnumField<'a, U> {
    type Target = WeakOneofEnumField<'a, U>;
    fn downgrade(self) -> WeakOneofEnumField<'a, U> {
        WeakOneofEnumField(Rc::downgrade(&self.0))
    }
}

#[derive(Debug)]
pub(crate) struct WeakOneofEnumField<'a, U>(Weak<OneofEnumFieldDetail<'a, U>>);
impl<'a, U> Clone for WeakOneofEnumField<'a, U> {
    fn clone(&self) -> Self {
        WeakOneofEnumField(self.0.clone())
    }
}
impl<'a, U> Upgrade for WeakOneofEnumField<'a, U> {
    type Target = OneofEnumField<'a, U>;

    fn upgrade(self) -> Self::Target {
        OneofEnumField(self.0.upgrade().expect("Failed to upgrade WeakOneofEnumField"))
    }
}
