use std::rc::Rc;

use super::FieldDetail;
use crate::{
    proto::Syntax,
    proto::{FieldDescriptor, Scalar},
    Comments, File, FullyQualified, Message, Name, Package,
};

#[derive(Debug)]
pub(crate) struct ScalarFieldDetail<'a, U> {
    detail: FieldDetail<'a, U>,
    scalar: Scalar,
}

impl<'a, U> ScalarFieldDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn build_target(&self) -> bool {
        self.detail.build_target()
    }
    pub fn scalar(&self) -> Scalar {
        self.scalar
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

    pub fn comments(&self) -> Comments<'a, U> {
        self.detail.comments()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.detail.package()
    }
    pub fn file(&self) -> File<'a, U> {
        self.detail.file()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.detail.set_comments(comments);
    }
}

impl<'a, U> Clone for ScalarFieldDetail<'a, U> {
    fn clone(&self) -> Self {
        Self {
            detail: self.detail.clone(),
            scalar: self.scalar.clone(),
        }
    }
}
#[derive(Debug)]
pub struct ScalarField<'a, U>(Rc<ScalarFieldDetail<'a, U>>);

impl<'a, U> ScalarField<'a, U> {
    pub fn scalar(&self) -> Scalar {
        self.0.scalar
    }
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        self.0.detail.comments()
    }
    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
    }
    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }

    pub fn build_target(&self) -> bool {
        self.0.detail.build_target()
    }

    pub fn descriptor(&self) -> FieldDescriptor {
        self.0.descriptor()
    }
}

impl<'a, U> FullyQualified for ScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}
impl<'a, U> Clone for ScalarField<'a, U> {
    fn clone(&self) -> Self {
        ScalarField(self.0.clone())
    }
}

#[cfg(test)]
impl<'a> Default for ScalarField<'a, crate::util::Generic> {
    fn default() -> Self {
        Self(Rc::new(ScalarFieldDetail {
            detail: Default::default(),
            scalar: Scalar::String,
        }))
    }
}

// #[derive(Debug, Clone)]
// pub(crate) struct WeakScalarField<'a, U>(Weak<MappedScalarFieldDetail<'a, U>>);
