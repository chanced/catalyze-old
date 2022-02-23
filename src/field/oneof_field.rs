use std::rc::Rc;

use super::FieldDetail;

#[derive(Debug, Clone)]
pub enum OneofField<'a, U> {
    Scalar(Rc<OneofScalarField<'a, U>>),
    Enum(Rc<OneofEnumField<'a, U>>),
    Message(Rc<OneofMessageField<'a, U>>),
}
#[derive(Debug, Clone)]
pub struct OneofScalarField<'a, U> {
    detail: FieldDetail<'a, U>,
}
#[derive(Debug, Clone)]
pub struct OneofEnumField<'a, U> {
    detail: FieldDetail<'a, U>,
}

#[derive(Debug, Clone)]
pub struct OneofMessageField<'a, U> {
    detail: FieldDetail<'a, U>,
}
