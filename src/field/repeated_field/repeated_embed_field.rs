use std::rc::{Rc, Weak};

use crate::{
    descriptor::FieldDescriptor, proto::Syntax, EmbedFieldDetail, FullyQualified, Message, Name,
};

#[derive(Debug)]
pub struct RepeatedEmbedField<'a, U>(Rc<EmbedFieldDetail<'a, U>>);

impl<'a, U> RepeatedEmbedField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn is_repeated(&self) -> bool {
        self.0.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.0.detail.is_map()
    }
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
}

impl<'a, U> Clone for RepeatedEmbedField<'a, U> {
    fn clone(&self) -> Self {
        RepeatedEmbedField(self.0.clone())
    }
}
