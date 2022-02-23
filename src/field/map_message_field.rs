use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub struct MapMessageField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> MapMessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
