use std::rc::{Rc, Weak};

use crate::{FullyQualified, MapFieldDetail, Message, Name, Named, WeakEnum};

#[derive(Debug, Clone)]
pub struct MapEnumFieldDetail<'a, U> {
    enm: WeakEnum<'a, U>,
    detail: MapFieldDetail<'a, U>,
}

#[derive(Debug)]
pub struct MapEnumField<'a, U>(Rc<MapEnumFieldDetail<'a, U>>);
impl<'a, U> Clone for MapEnumField<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<'a, U> MapEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
    pub fn container(&self) -> Message<'a, U> {
        self.0.detail.container()
    }
    pub fn containing_message(&self) -> Message<'a, U> {
        self.container()
    }
}

impl<'a, U> FullyQualified for MapEnumField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for MapEnumField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
}

#[derive(Debug)]
pub(crate) struct WeakMapEnumField<'a, U>(Weak<MapEnumFieldDetail<'a, U>>);
impl<'a, U> Clone for WeakMapEnumField<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
