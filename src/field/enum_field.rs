use crate::{FieldDetail, Name};

#[derive(Debug, Clone)]
pub struct EnumField<'a, U> {
    detail: FieldDetail<'a, U>,
}
impl<'a, U> EnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
