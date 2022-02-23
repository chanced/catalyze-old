use std::marker::PhantomData;

use crate::Field;

pub struct FieldIter<'a, U> {
    fields: &'a [Field<'a, U>],
    idx: usize,
}

impl<'a, U> Iterator for FieldIter<'a, U> {
    type Item = &'a Field<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        self.fields.get(self.idx - 1)
    }
}
impl<'a, U> From<&'a [Field<'a, U>]> for FieldIter<'a, U> {
    fn from(fields: &'a [Field<'a, U>]) -> Self {
        Self { fields, idx: 0 }
    }
}
