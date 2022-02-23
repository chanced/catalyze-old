use crate::FieldDetail;

#[derive(Debug, Clone)]
pub struct EnumField<'a, U> {
    detail: FieldDetail<'a, U>,
}
