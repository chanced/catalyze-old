#![allow(clippy::new_ret_no_self)]

use std::{cell::RefCell, rc::Rc};

use anyhow::bail;

use crate::{
    proto::FieldDescriptor, proto::Syntax, Comments, Enum, Field, FieldDetail, File, Files,
    FullyQualified, Message, Name, Node, Package, Type, WeakEnum, WellKnownEnum, WellKnownType,
};

#[derive(Debug, Clone)]
pub(crate) struct EnumFieldDetail<'a, U> {
    pub detail: FieldDetail<'a, U>,
    pub enumeration: RefCell<WeakEnum<'a, U>>,
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
    pub fn comments(&self) -> Comments<'a> {
        self.detail.comments()
    }

    pub fn set_comments(&self, comments: Comments<'a>) {
        self.detail.set_comments(comments)
    }
    pub fn package(&self) -> Package<'a, U> {
        self.detail.package()
    }
    pub fn well_known_enum(&self) -> Option<WellKnownEnum> {
        self.enumeration().well_known_enum()
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
        self.enumeration.borrow().clone().into()
    }
    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }
    pub fn enumeration(&self) -> Enum<'a, U> {
        self.r#enum()
    }

    pub fn file(&self) -> File<'a, U> {
        self.detail.file()
    }

    pub fn imports(&self) -> Files<'a, U> {
        let e = self.r#enum();
        if self.file() != e.file() {
            Files::from(e.weak_file())
        } else {
            Files::empty()
        }
    }

    pub fn has_import(&self) -> bool {
        self.enumeration().file() != self.detail.file()
    }
    pub fn is_well_known_type(&self) -> bool {
        self.enumeration().is_well_known_type()
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.enumeration().well_known_type()
    }

    pub(crate) fn set_value(&self, node: Node<'a, U>) -> Result<(), anyhow::Error> {
        match node {
            Node::Enum(v) => {
                self.enumeration.replace(v.into());
                Ok(())
            }
            _ => bail!("expected Enum, received {}", node),
        }
    }
}

#[derive(Debug)]
pub struct EnumField<'a, U>(Rc<EnumFieldDetail<'a, U>>);

impl<'a, U> EnumField<'a, U> {
    pub(crate) fn new(detail: FieldDetail<'a, U>) -> Result<Field<'a, U>, anyhow::Error> {
        if matches!(detail.value_type(), Type::Enum(_)) {
            bail!("expected Enum, received {}", detail.value_type());
        }
        let e = EnumFieldDetail {
            detail,
            enumeration: RefCell::new(WeakEnum::empty()),
        };
        let f = EnumField(Rc::new(e));
        Ok(Field::Enum(f))
    }

    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn well_known_enum(&self) -> Option<WellKnownEnum> {
        self.0.well_known_enum()
    }
    /// Returns the `Enum` of this `EnumField`.
    pub fn r#enum(&self) -> Enum<'a, U> {
        self.0.enumeration.borrow().clone().into()
    }

    pub fn build_target(&self) -> bool {
        self.0.build_target()
    }

    pub fn is_repeated(&self) -> bool {
        self.0.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.0.is_map()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.0.message()
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.0.util()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.comments()
    }

    pub fn set_comments(&self, comments: Comments<'a>) {
        self.0.set_comments(comments)
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.package()
    }

    /// alias for `r#enum`
    ///
    /// Returns the `Enum` of this `EnumField`.
    pub fn enumeration(&self) -> Enum<'a, U> {
        self.r#enum()
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

    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
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

    pub(crate) fn set_value(&self, value: Node<'a, U>) -> Result<(), anyhow::Error> {
        self.0.set_value(value)
    }

    pub fn value_type(&self) -> Type<'a> {
        self.descriptor().proto_type()
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
