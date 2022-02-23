use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub struct RepeatedEnumField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> RepeatedEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
