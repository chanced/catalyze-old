use std::rc::Rc;

use crate::{FullyQualified, Message, Name, OneofFieldDetail, ScalarField};

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
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }
}
impl<'a, U> FullyQualified for OneofEnumField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}
impl<'a, U> Clone for OneofEnumField<'a, U> {
    fn clone(&self) -> Self {
        OneofEnumField(self.0.clone())
    }
}
