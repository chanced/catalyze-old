use std::rc::{Rc, Weak};

use crate::{
    descriptor::FieldDescriptor, proto::Syntax, traits::Upgrade, Field, FullyQualified, MapField,
    MapFieldDetail, Message, Name, Named, WeakMessage,
};

#[derive(Debug)]
pub struct MappedEmbedFieldDetail<'a, U> {
    embed: WeakMessage<'a, U>,
    detail: MapFieldDetail<'a, U>,
}
impl<'a, U> Clone for MappedEmbedFieldDetail<'a, U> {
    fn clone(&self) -> Self {
        Self {
            embed: self.embed.clone(),
            detail: self.detail.clone(),
        }
    }
}

#[derive(Debug)]
pub struct MappedEmbedField<'a, U>(Rc<MappedEmbedFieldDetail<'a, U>>);

impl<'a, U> MappedEmbedField<'a, U> {
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
    /// Returns the Message which contains this field.
    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.0.detail.util()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.0.detail.replace_util(util);
    }
    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.0.detail.descriptor()
    }
    /// Returns the embedded message.
    pub fn embed(&self) -> Message<'a, U> {
        self.0.embed.upgrade()
    }

    pub fn has_presence(&self) -> bool {
        true
    }
}

impl<'a, U> FullyQualified for MappedEmbedField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}

impl<'a, U> Named<U> for MappedEmbedField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
}

impl<'a, U> Clone for MappedEmbedField<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[derive(Debug)]
pub(crate) struct WeakMappedEmbedField<'a, U>(Weak<MappedEmbedFieldDetail<'a, U>>);

impl<'a, U> Clone for WeakMappedEmbedField<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
