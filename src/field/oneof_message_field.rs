use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub struct OneofMessageField<'a, U> {
    detail: FieldDetail<'a, U>,
}
impl<'a, U> OneofMessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
