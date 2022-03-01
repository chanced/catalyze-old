use std::rc::{Rc, Weak};

use crate::{
    descriptor::FieldDescriptor, proto::Syntax, util::Util, Field, FullyQualified, MapField,
    MapFieldDetail, Message, Name, Named, WeakMessage,
};

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
    pub fn is_repeated(&self) -> bool {
        self.0.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.0.detail.is_map()
    }
    pub fn container(&self) -> Message<'a, U> {
        self.0.detail.container()
    }
    pub fn util(&self) -> Util<U> {
        self.0.detail.util()
    }
    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.0.detail.descriptor()
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
