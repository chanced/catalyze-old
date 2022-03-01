use std::rc::{Rc, Weak};

use crate::{
    descriptor::Scalar,
    traits::{Downgrade, Upgrade},
    FullyQualified, Message, Name, Named, OneofFieldDetail,
};

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

    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }

    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }
}

impl<'a, U> FullyQualified for OneofScalarField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}

impl<'a, U> Named<U> for OneofScalarField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
}

impl<'a, U> Clone for OneofScalarField<'a, U> {
    fn clone(&self) -> Self {
        OneofScalarField(self.0.clone())
    }
}
impl<'a, U> Downgrade for OneofScalarField<'a, U> {
    type Output = WeakOneofScalarField<'a, U>;
    fn downgrade(self) -> WeakOneofScalarField<'a, U> {
        WeakOneofScalarField(Rc::downgrade(&self.0))
    }
}

#[derive(Debug)]
pub(crate) struct WeakOneofScalarField<'a, U>(Weak<OneofScalarFieldDetail<'a, U>>);
impl<'a, U> Clone for WeakOneofScalarField<'a, U> {
    fn clone(&self) -> Self {
        WeakOneofScalarField(self.0.clone())
    }
}
impl<'a, U> Upgrade for WeakOneofScalarField<'a, U> {
    type Output = OneofScalarField<'a, U>;

    fn upgrade(self) -> Self::Output {
        OneofScalarField(
            self.0
                .upgrade()
                .expect("Failed to upgrade WeakOneofScalarField"),
        )
    }
}
