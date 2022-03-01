use std::rc::{Rc, Weak};

use crate::{traits::Upgrade, FullyQualified, Name, Named, ScalarFieldDetail};

#[derive(Debug, Clone)]
pub(crate) struct RepeatedScalarFieldDetail<'a, U> {
    detail: ScalarFieldDetail<'a, U>,
}

#[derive(Debug)]
pub(crate) struct RepeatedScalarField<'a, U>(Rc<RepeatedScalarFieldDetail<'a, U>>);

impl<'a, U> RepeatedScalarField<'a, U> {}

impl<'a, U> RepeatedScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}

impl<'a, U> FullyQualified for RepeatedScalarField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for RepeatedScalarField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
}
impl<'a, U> Clone for RepeatedScalarField<'a, U> {
    fn clone(&self) -> Self {
        RepeatedScalarField(self.0.clone())
    }
}

#[derive(Debug)]
pub(crate) struct WeakRepeatedScalarField<'a, U>(Weak<RepeatedScalarFieldDetail<'a, U>>);
impl<'a, U> Clone for WeakRepeatedScalarField<'a, U> {
    fn clone(&self) -> Self {
        WeakRepeatedScalarField(self.0.clone())
    }
}

impl<'a, U> Upgrade for WeakRepeatedScalarField<'a, U> {
    type Target = RepeatedScalarField<'a, U>;

    fn upgrade(self) -> Self::Target {
        RepeatedScalarField(
            self.0
                .upgrade()
                .expect("Failed to upgrade WeakRepeatedScalarField"),
        )
    }
}
