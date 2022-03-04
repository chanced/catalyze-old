use std::rc::Rc;

use crate::{
    proto::{FieldDescriptor, Syntax},
    Comments, EmbedFieldDetail, EnumFieldDetail, FullyQualified, Message, Name, ScalarFieldDetail,
};

/// Represents a field marked as `repeated`. The field can hold
/// a scalar value, an enum, or a message.
#[derive(Debug)]
pub enum RepeatedField<'a, U> {
    Scalar(RepeatedScalarField<'a, U>),
    Enum(RepeatedEnumField<'a, U>),
    Message(RepeatedEmbedField<'a, U>),
}

impl<'a, U> RepeatedField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            RepeatedField::Scalar(f) => f.name(),
            RepeatedField::Enum(f) => f.name(),
            RepeatedField::Message(f) => f.name(),
        }
    }
    pub fn fully_qualified_name(&self) -> String {
        match self {
            RepeatedField::Scalar(f) => f.fully_qualified_name(),
            RepeatedField::Enum(f) => f.fully_qualified_name(),
            RepeatedField::Message(f) => f.fully_qualified_name(),
        }
    }
    /// Returns the Message containing this RepeatedField
    pub fn message(&self) -> Message<'a, U> {
        match self {
            RepeatedField::Scalar(f) => f.message(),
            RepeatedField::Enum(f) => f.message(),
            RepeatedField::Message(f) => f.message(),
        }
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        match self {
            RepeatedField::Scalar(f) => f.util(),
            RepeatedField::Enum(f) => f.util(),
            RepeatedField::Message(f) => f.util(),
        }
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        match self {
            RepeatedField::Scalar(s) => s.replace_util(util),
            RepeatedField::Enum(e) => e.replace_util(util),
            RepeatedField::Message(m) => m.replace_util(util),
        }
    }
    pub fn syntax(&self) -> Syntax {
        match self {
            RepeatedField::Scalar(f) => f.syntax(),
            RepeatedField::Enum(f) => f.syntax(),
            RepeatedField::Message(f) => f.syntax(),
        }
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        match self {
            RepeatedField::Scalar(f) => f.descriptor(),
            RepeatedField::Enum(f) => f.descriptor(),
            RepeatedField::Message(f) => f.descriptor(),
        }
    }
    pub fn comments(&self) -> Comments<'a, U> {
        match self {
            RepeatedField::Scalar(f) => f.comments(),
            RepeatedField::Enum(f) => f.comments(),
            RepeatedField::Message(f) => f.comments(),
        }
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        match self {
            RepeatedField::Scalar(f) => f.set_comments(comments),
            RepeatedField::Enum(f) => f.set_comments(comments),
            RepeatedField::Message(f) => f.set_comments(comments),
        }
    }
    pub fn is_map(&self) -> bool {
        false
    }

    pub fn is_repeated(&self) -> bool {
        true
    }

    pub fn is_scalar(&self) -> bool {
        matches!(self, RepeatedField::Scalar(_))
    }

    pub fn is_marked_optional(&self) -> bool {
        false
    }

    pub fn has_presence(&self) -> bool {
        matches!(self, RepeatedField::Message(_))
    }
}
impl<'a, U> Clone for RepeatedField<'a, U> {
    fn clone(&self) -> Self {
        match self {
            RepeatedField::Scalar(f) => RepeatedField::Scalar(f.clone()),
            RepeatedField::Enum(f) => RepeatedField::Enum(f.clone()),
            RepeatedField::Message(f) => RepeatedField::Message(f.clone()),
        }
    }
}
impl<'a, U> FullyQualified for RepeatedField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            RepeatedField::Scalar(f) => f.fully_qualified_name(),
            RepeatedField::Enum(f) => f.fully_qualified_name(),
            RepeatedField::Message(f) => f.fully_qualified_name(),
        }
    }
}

impl<'a, U> From<RepeatedScalarField<'a, U>> for RepeatedField<'a, U> {
    fn from(f: RepeatedScalarField<'a, U>) -> Self {
        RepeatedField::Scalar(f)
    }
}

impl<'a, U> From<&RepeatedScalarField<'a, U>> for RepeatedField<'a, U> {
    fn from(f: &RepeatedScalarField<'a, U>) -> Self {
        RepeatedField::Scalar(f.clone())
    }
}

impl<'a, U> From<RepeatedEnumField<'a, U>> for RepeatedField<'a, U> {
    fn from(f: RepeatedEnumField<'a, U>) -> Self {
        RepeatedField::Enum(f)
    }
}

impl<'a, U> From<&RepeatedEnumField<'a, U>> for RepeatedField<'a, U> {
    fn from(f: &RepeatedEnumField<'a, U>) -> Self {
        RepeatedField::Enum(f.clone())
    }
}

impl<'a, U> From<RepeatedEmbedField<'a, U>> for RepeatedField<'a, U> {
    fn from(f: RepeatedEmbedField<'a, U>) -> Self {
        RepeatedField::Message(f)
    }
}

impl<'a, U> From<&RepeatedEmbedField<'a, U>> for RepeatedField<'a, U> {
    fn from(f: &RepeatedEmbedField<'a, U>) -> Self {
        RepeatedField::Message(f.clone())
    }
}

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
    pub fn comments(&self) -> Comments<'a, U> {
        self.0.detail.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }
}

impl<'a, U> Clone for RepeatedEmbedField<'a, U> {
    fn clone(&self) -> Self {
        RepeatedEmbedField(self.0.clone())
    }
}

#[derive(Debug)]
pub struct RepeatedEnumField<'a, U>(Rc<EnumFieldDetail<'a, U>>);

impl<'a, U> RepeatedEnumField<'a, U> {
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
    pub fn is_marked_optional(&self) -> bool {
        self.0.detail.is_marked_optional()
    }
    pub fn is_required(&self) -> bool {
        self.0.detail.is_required()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        self.0.detail.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }
}
impl<'a, U> Clone for RepeatedEnumField<'a, U> {
    fn clone(&self) -> Self {
        RepeatedEnumField(self.0.clone())
    }
}
impl<'a, U> FullyQualified for RepeatedEnumField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RepeatedScalarFieldDetail<'a, U> {
    detail: ScalarFieldDetail<'a, U>,
}

#[derive(Debug)]
pub struct RepeatedScalarField<'a, U>(Rc<RepeatedScalarFieldDetail<'a, U>>);

impl<'a, U> RepeatedScalarField<'a, U> {
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
    pub fn comments(&self) -> Comments<'a, U> {
        self.0.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }
}

impl<'a, U> FullyQualified for RepeatedScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}

impl<'a, U> Clone for RepeatedScalarField<'a, U> {
    fn clone(&self) -> Self {
        RepeatedScalarField(self.0.clone())
    }
}
