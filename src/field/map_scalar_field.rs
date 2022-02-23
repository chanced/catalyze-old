use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub struct MapScalarField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> MapScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
