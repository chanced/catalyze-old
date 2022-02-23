use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub struct OneofScalarField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> OneofScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
