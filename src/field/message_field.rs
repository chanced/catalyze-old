use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub struct MessageField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> MessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
