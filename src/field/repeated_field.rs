use std::rc::Rc;

use crate::{
    proto::{FieldDescriptor, Scalar, Syntax},
    Comments, EmbedFieldDetail, Enum, EnumFieldDetail, File, FullyQualified, Message, Name,
    Package, ScalarFieldDetail,
};

/// Represents a field marked as `repeated`. The field can hold
/// a scalar value, an enum, or a message.
#[derive(Debug)]
pub enum RepeatedField<'a, U> {
    Scalar(RepeatedScalarField<'a, U>),
    Enum(RepeatedEnumField<'a, U>),
    Embed(RepeatedEmbedField<'a, U>),
}

impl<'a, U> RepeatedField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            RepeatedField::Scalar(f) => f.name(),
            RepeatedField::Enum(f) => f.name(),
            RepeatedField::Embed(f) => f.name(),
        }
    }

    pub fn file(&self) -> File<'a, U> {
        match self {
            RepeatedField::Scalar(f) => f.file(),
            RepeatedField::Enum(f) => f.file(),
            RepeatedField::Embed(f) => f.file(),
        }
    }

    pub fn package(&self) -> Package<'a, U> {
        match self {
            RepeatedField::Scalar(f) => f.package(),
            RepeatedField::Enum(f) => f.package(),
            RepeatedField::Embed(f) => f.package(),
        }
    }

    pub fn fully_qualified_name(&self) -> String {
        match self {
            RepeatedField::Scalar(f) => f.fully_qualified_name(),
            RepeatedField::Enum(f) => f.fully_qualified_name(),
            RepeatedField::Embed(f) => f.fully_qualified_name(),
        }
    }
    /// Returns the Message containing this RepeatedField
    pub fn message(&self) -> Message<'a, U> {
        match self {
            RepeatedField::Scalar(f) => f.message(),
            RepeatedField::Enum(f) => f.message(),
            RepeatedField::Embed(f) => f.message(),
        }
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        match self {
            RepeatedField::Scalar(f) => f.util(),
            RepeatedField::Enum(f) => f.util(),
            RepeatedField::Embed(f) => f.util(),
        }
    }

    pub fn syntax(&self) -> Syntax {
        match self {
            RepeatedField::Scalar(f) => f.syntax(),
            RepeatedField::Enum(f) => f.syntax(),
            RepeatedField::Embed(f) => f.syntax(),
        }
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        match self {
            RepeatedField::Scalar(f) => f.descriptor(),
            RepeatedField::Enum(f) => f.descriptor(),
            RepeatedField::Embed(f) => f.descriptor(),
        }
    }
    pub fn comments(&self) -> Comments<'a, U> {
        match self {
            RepeatedField::Scalar(f) => f.comments(),
            RepeatedField::Enum(f) => f.comments(),
            RepeatedField::Embed(f) => f.comments(),
        }
    }

    pub fn has_import(&self) -> bool {
        match self {
            RepeatedField::Enum(f) => f.has_import(),
            RepeatedField::Embed(f) => f.has_import(),
            RepeatedField::Scalar(_) => false,
        }
    }
    pub fn imports(&self) -> Option<File<'a, U>> {
        match self {
            RepeatedField::Enum(f) => f.imports(),
            RepeatedField::Embed(f) => f.imports(),
            RepeatedField::Scalar(_) => None,
        }
    }
    pub fn build_target(&self) -> bool {
        match self {
            RepeatedField::Scalar(f) => f.build_target(),
            RepeatedField::Enum(f) => f.build_target(),
            RepeatedField::Embed(f) => f.build_target(),
        }
    }
    pub fn r#enum(&self) -> Option<Enum<'a, U>> {
        match self {
            RepeatedField::Enum(f) => Some(f.r#enum()),
            _ => None,
        }
    }
    pub fn enumeration(&self) -> Option<Enum<'a, U>> {
        match self {
            RepeatedField::Enum(f) => Some(f.enumeration()),
            _ => None,
        }
    }
    pub fn embed(&self) -> Option<Message<'a, U>> {
        match self {
            RepeatedField::Embed(f) => Some(f.embed()),
            _ => None,
        }
    }
    pub fn scalar(&self) -> Option<Scalar> {
        match self {
            RepeatedField::Scalar(f) => Some(f.scalar()),
            _ => None,
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
        matches!(self, RepeatedField::Embed(_))
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        match self {
            RepeatedField::Scalar(f) => f.set_comments(comments),
            RepeatedField::Enum(f) => f.set_comments(comments),
            RepeatedField::Embed(f) => f.set_comments(comments),
        }
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        match self {
            RepeatedField::Scalar(s) => s.replace_util(util),
            RepeatedField::Enum(e) => e.replace_util(util),
            RepeatedField::Embed(m) => m.replace_util(util),
        }
    }
}
impl<'a, U> Clone for RepeatedField<'a, U> {
    fn clone(&self) -> Self {
        match self {
            RepeatedField::Scalar(f) => RepeatedField::Scalar(f.clone()),
            RepeatedField::Enum(f) => RepeatedField::Enum(f.clone()),
            RepeatedField::Embed(f) => RepeatedField::Embed(f.clone()),
        }
    }
}
impl<'a, U> FullyQualified for RepeatedField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            RepeatedField::Scalar(f) => f.fully_qualified_name(),
            RepeatedField::Enum(f) => f.fully_qualified_name(),
            RepeatedField::Embed(f) => f.fully_qualified_name(),
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
        RepeatedField::Embed(f)
    }
}

impl<'a, U> From<&RepeatedEmbedField<'a, U>> for RepeatedField<'a, U> {
    fn from(f: &RepeatedEmbedField<'a, U>) -> Self {
        RepeatedField::Embed(f.clone())
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
    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
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
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        self.0.detail.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }

    pub fn has_import(&self) -> bool {
        self.file() != self.0.embed().file()
    }
    pub fn imports(&self) -> Option<File<'a, U>> {
        if self.has_import() {
            Some(self.file())
        } else {
            None
        }
    }

    pub fn embed(&self) -> Message<'a, U> {
        self.0.embed()
    }

    pub fn build_target(&self) -> bool {
        self.0.detail.build_target()
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
    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
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
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
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

    pub fn has_import(&self) -> bool {
        self.0.has_import()
    }
    pub fn imports(&self) -> Option<File<'a, U>> {
        self.0.imports()
    }

    pub fn r#enum(&self) -> Enum<'a, U> {
        self.0.r#enum()
    }
    pub fn enumeration(&self) -> Enum<'a, U> {
        self.0.enumeration()
    }
    pub fn build_target(&self) -> bool {
        self.0.build_target()
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

    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
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
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }
    pub fn comments(&self) -> Comments<'a, U> {
        self.0.detail.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }

    pub fn scalar(&self) -> Scalar {
        self.0.detail.scalar()
    }

    pub fn build_target(&self) -> bool {
        self.0.detail.build_target()
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
