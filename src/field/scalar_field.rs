use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub struct ScalarField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> ScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
