use std::rc::{Rc, Weak};

use crate::{
    descriptor::FieldDescriptor, proto::Syntax, Enum, FullyQualified, MapFieldDetail, Message,
    Name, WeakEnum, WellKnownEnum,
};

#[derive(Debug, Clone)]
pub struct MappedEnumFieldDetail<'a, U> {
    enm: WeakEnum<'a, U>,
    detail: MapFieldDetail<'a, U>,
}

#[derive(Debug)]
pub struct MappedEnumField<'a, U>(Rc<MappedEnumFieldDetail<'a, U>>);
impl<'a, U> Clone for MappedEnumField<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<'a, U> MappedEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }

    pub fn is_repeated(&self) -> bool {
        self.0.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.0.detail.is_map()
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

    /// Alias for `r#enum()`
    pub fn enumeration(&self) -> Enum<'a, U> {
        self.r#enum()
    }
    pub fn r#enum(&self) -> Enum<'a, U> {
        self.0.enm.clone().into()
    }

    pub fn well_known_type(&self) -> Option<WellKnownEnum> {
        self.0.detail.well_known_type().map(|wkt| match wkt {
            crate::WellKnownType::Enum(wke) => wke,
            _ => unreachable!(),
        })
    }
}

impl<'a, U> FullyQualified for MappedEnumField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}
