use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub struct RepeatedScalarField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> RepeatedScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
