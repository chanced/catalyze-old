use std::rc::Rc;

use crate::{descriptor::Scalar, FullyQualified, Message, Name, OneofFieldDetail};

#[derive(Debug, Clone)]
pub struct OneofScalarFieldDetail<'a, U> {
    scalar: Scalar,
    detail: OneofFieldDetail<'a, U>,
}
#[derive(Debug)]
pub struct OneofScalarField<'a, U>(Rc<OneofScalarFieldDetail<'a, U>>);

impl<'a, U> OneofScalarField<'a, U> {
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

impl<'a, U> FullyQualified for OneofScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}

impl<'a, U> Clone for OneofScalarField<'a, U> {
    fn clone(&self) -> Self {
        OneofScalarField(self.0.clone())
    }
}
