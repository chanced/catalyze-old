use std::rc::{Rc, Weak};

use crate::{Field, FullyQualified, MapField, MapFieldDetail, Name, Named, WeakMessage};

#[derive(Debug)]
pub struct MapMessageFieldDetail<'a, U> {
    message: WeakMessage<'a, U>,
    detail: MapFieldDetail<'a, U>,
}

#[derive(Debug)]
pub struct MapMessageField<'a, U>(Rc<MapMessageFieldDetail<'a, U>>);

impl<'a, U> MapMessageField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}

impl<'a, U> FullyQualified for MapMessageField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}

impl<'a, U> Named<U> for MapMessageField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
}

impl<'a, U> Clone for MapMessageField<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[derive(Debug)]
pub(crate) struct WeakMapMessageField<'a, U>(Weak<MapMessageFieldDetail<'a, U>>);

impl<'a, U> Clone for WeakMapMessageField<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
