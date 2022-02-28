use std::rc::{Rc, Weak};

use crate::{
    Field, FieldDetail, FullyQualified, MapField, MapFieldDetail, MapKey, Name, Named, ScalarField,
};

#[derive(Debug, Clone)]
pub struct MapScalarFieldDetail<'a, U> {
    detail: MapFieldDetail<'a, U>,
    scalar_field: ScalarField<'a, U>,
}

#[derive(Debug)]
pub struct MapScalarField<'a, U>(Rc<MapScalarFieldDetail<'a, U>>);
impl<'a, U> Clone for MapScalarField<'a, U> {
    fn clone(&self) -> Self {
        MapScalarField(self.0.clone())
    }
}

impl<'a, U> MapScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}

impl<'a, U> Named<U> for MapScalarField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
}

impl<'a, U> FullyQualified for MapScalarField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}

#[derive(Debug)]
pub(crate) struct WeakMapScalarField<'a, U>(Weak<MapScalarFieldDetail<'a, U>>);
impl<'a, U> Clone for WeakMapScalarField<'a, U> {
    fn clone(&self) -> Self {
        WeakMapScalarField(self.0.clone())
    }
}
