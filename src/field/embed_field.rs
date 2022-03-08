use std::rc::Rc;

use crate::{
    proto::FieldDescriptor, proto::Syntax, Comments, File, FullyQualified, Message, Name, Package,
    WeakMessage,
};

use super::FieldDetail;

#[derive(Debug)]
pub(crate) struct EmbedFieldDetail<'a, U> {
    pub detail: FieldDetail<'a, U>,
    pub embed: WeakMessage<'a, U>,
}
impl<'a, U> EmbedFieldDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn is_repeated(&self) -> bool {
        self.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.detail.is_map()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.detail.message()
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.detail.util()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.detail.util.replace(util);
    }
    pub fn syntax(&self) -> Syntax {
        self.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.detail.descriptor()
    }
    pub fn embed(&self) -> Message<'a, U> {
        self.embed.clone().into()
    }
}

impl<'a, U> Clone for EmbedFieldDetail<'a, U> {
    fn clone(&self) -> Self {
        Self {
            detail: self.detail.clone(),
            embed: self.embed.clone(),
        }
    }
}

#[derive(Debug)]
pub struct EmbedField<'a, U>(Rc<EmbedFieldDetail<'a, U>>);

impl<'a, U> EmbedField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name.clone()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        self.0.detail.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }

    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
    }
}
impl<'a, U> Clone for EmbedField<'a, U> {
    fn clone(&self) -> Self {
        EmbedField(self.0.clone())
    }
}

impl<'a, U> FullyQualified for EmbedField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fqn.clone()
    }
}
