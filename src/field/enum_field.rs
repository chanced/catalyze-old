use std::rc::Rc;

use crate::{
    proto::FieldDescriptor, proto::Syntax, Comments, Enum, FieldDetail, File, FullyQualified,
    Message, Name, Package, WeakEnum,
};

#[derive(Debug, Clone)]
pub(crate) struct EnumFieldDetail<'a, U> {
    pub detail: FieldDetail<'a, U>,
    pub e: WeakEnum<'a, U>,
}

impl<'a, U> EnumFieldDetail<'a, U> {
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
    pub fn r#enum(&self) -> Enum<'a, U> {
        self.e.clone().into()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        self.detail.comments()
    }

    pub fn set_comments(&self, comments: Comments<'a, U>) {
        self.detail.set_comments(comments)
    }
    pub fn file(&self) -> File<'a, U> {
        self.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.detail.package()
    }
    pub fn extern_dep(&self) -> Option<File<'a, U>> {
        if self.e.file() != self.detail.file() {
            Some(self.e.file().clone())
        } else {
            None
        }
    }
    pub fn has_extern_dep(&self) -> bool {
        self.e.file() != self.detail.file()
    }
}

#[derive(Debug)]
pub struct EnumField<'a, U>(Rc<EnumFieldDetail<'a, U>>);

impl<'a, U> EnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    /// Returns the `Enum` of this `EnumField`.
    pub fn r#enum(&self) -> Enum<'a, U> {
        self.0.e.clone().into()
    }
    /// alias for `r#enum`
    ///
    /// Returns the `Enum` of this `EnumField`.
    pub fn enumeration(&self) -> Enum<'a, U> {
        self.r#enum()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        self.0.detail.comments()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
    }
    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }

    pub(crate) fn is_well_known_type(&self) -> bool {
        self.0.e.is_well_known_type()
    }

    pub(crate) fn has_extern_dep(&self) -> bool {
        todo!()
    }
}

impl<'a, U> FullyQualified for EnumField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}

impl<'a, U> Clone for EnumField<'a, U> {
    fn clone(&self) -> Self {
        EnumField(self.0.clone())
    }
}
