use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub struct MapEnumField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> MapEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
