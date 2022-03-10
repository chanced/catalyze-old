use std::rc::Rc;

use crate::{
    file, proto::FieldDescriptor, proto::Syntax, Comments, Enum, FieldDetail, File, Files,
    FullyQualified, Message, Name, Package, WeakEnum, WellKnownEnum, WellKnownType,
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
    pub fn is_marked_required(&self) -> bool {
        self.detail.is_marked_required()
    }
    pub fn is_marked_optional(&self) -> bool {
        self.detail.is_marked_optional()
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
    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }
    pub fn enumeration(&self) -> Enum<'a, U> {
        self.r#enum()
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
    pub fn imports(&self) -> Files<'a, U> {
        if self.file() != self.e.file() {
            Files::from(self.e.weak_file())
        } else {
            Files::empty()
        }
    }

    pub fn has_import(&self) -> bool {
        self.e.file() != self.detail.file()
    }
    pub fn is_well_known_type(&self) -> bool {
        self.e.is_well_known_type()
    }
    pub fn well_known_enum(&self) -> Option<WellKnownEnum> {
        self.e.well_known_enum()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.e.well_known_type()
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

    pub fn build_target(&self) -> bool {
        self.0.build_target()
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
    pub fn has_presence(&self) -> bool {
        self.syntax() == Syntax::Proto2 || self.descriptor().is_marked_optional(self.syntax())
    }
    pub fn syntax(&self) -> Syntax {
        self.0.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.descriptor()
    }

    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
    }
    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }

    pub fn set_comments(&self, comments: Comments<'a, U>) {
        self.0.detail.set_comments(comments);
    }

    pub fn is_well_known_type(&self) -> bool {
        self.0.is_well_known_type()
    }

    pub fn has_import(&self) -> bool {
        self.0.has_import()
    }
    pub fn imports(&self) -> Files<'a, U> {
        self.0.imports()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.is_marked_required()
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.0.well_known_type()
    }

    pub fn is_marked_optional(&self) -> bool {
        self.0.is_marked_optional()
    }

    pub fn util(&self) -> Rc<U> {
        self.0.detail.util()
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
