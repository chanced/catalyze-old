use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub struct OneofEnumField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> OneofEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
