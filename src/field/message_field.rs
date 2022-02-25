use crate::{FieldDetail, FullyQualified, Name};

#[derive(Debug, Clone)]
pub struct MessageField<'a, U> {
    detail: FieldDetail<'a, U>,
}

impl<'a, U> MessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
}
impl<'a, U> FullyQualified for MessageField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}
