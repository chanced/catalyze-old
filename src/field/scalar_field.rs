use std::rc::Rc;

use super::{FieldDetail, MapScalarFieldDetail, WeakField};
use crate::{Field, FullyQualified, Name};

pub enum Scalar {
    /// 0 is reserved for errors.
    /// Order is weird for historical reasons.
    Double = 1,
    Float = 2,
    /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT64 if
    /// negative values are likely.
    Int64 = 3,
    Uint64 = 4,
    /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT32 if
    /// negative values are likely.
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    Bool = 8,
    String = 9,
    /// New in version 2.
    Bytes = 12,
    Uint32 = 13,
    Enum = 14,
    Sfixed32 = 15,
    Sfixed64 = 16,
    /// Uses ZigZag encoding.
    Sint32 = 17,
    /// Uses ZigZag encoding.
    Sint64 = 18,
}

#[derive(Debug, Clone)]
pub(crate) struct ScalarFieldDetail<'a, U> {
    detail: FieldDetail<'a, U>,
    field: WeakField<'a, U>,
}

#[derive(Debug)]
pub struct ScalarField<'a, U>(Rc<MapScalarFieldDetail<'a, U>>);

impl<'a, U> Clone for ScalarField<'a, U> {
    fn clone(&self) -> Self {
        ScalarField(self.0.clone())
    }
}

impl<'a, U> ScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn field(&self) -> Field<'a, U> {
        self.field.upgrade()
    }
}

impl<'a, U> FullyQualified for ScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
}
