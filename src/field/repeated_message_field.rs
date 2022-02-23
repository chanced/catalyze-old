use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub struct RepeatedMessageField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> RepeatedMessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
