use std::rc::Rc;

use crate::{
    name::Named, traits::Upgrade, FullyQualified, Name, Oneof, WellKnownEnum, WellKnownMessage,
    WellKnownType,
};

use super::{EnumField, MessageField};

/// Google provided, Well-Known Types
///
/// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf
#[derive(Clone, Debug)]
pub enum WellKnownTypeField<'a, U> {
    Message(WellKnownMessageField<'a, U>),
    Enum(WellKnownEnumField<'a, U>),
}

impl<'a, U> WellKnownTypeField<'a, U> {
    fn name(&self) -> Name<U> {
        match self {
            WellKnownTypeField::Message(m) => m.name(),
            WellKnownTypeField::Enum(e) => e.name(),
        }
    }
    pub fn fully_qualified_name(&self) -> String {
        match self {
            WellKnownTypeField::Message(m) => m.fully_qualified_name(),
            WellKnownTypeField::Enum(e) => e.fully_qualified_name(),
        }
    }
}
impl<'a, U> FullyQualified for WellKnownTypeField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            WellKnownTypeField::Message(m) => m.fully_qualified_name(),
            WellKnownTypeField::Enum(e) => e.fully_qualified_name(),
        }
    }
}
impl<'a, U> Named<U> for WellKnownTypeField<'a, U> {
    fn name(&self) -> Name<U> {
        match self {
            WellKnownTypeField::Message(m) => m.name(),
            WellKnownTypeField::Enum(e) => e.name(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct WellKnownMsgFieldDetail<'a, U> {
    well_known_msg: WellKnownMessage,
    msg_fld: MessageField<'a, U>,
    descriptor: &'a prost_types::FieldDescriptorProto,
    util: U,
}

#[derive(Debug)]
pub struct WellKnownMessageField<'a, U>(Rc<WellKnownMsgFieldDetail<'a, U>>);

impl<'a, U> WellKnownMessageField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.msg_fld.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.msg_fld.fully_qualified_name()
    }
}
impl<'a, U> FullyQualified for WellKnownMessageField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.msg_fld.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for WellKnownMessageField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.msg_fld.name()
    }
}
impl<'a, U> Clone for WellKnownMessageField<'a, U> {
    fn clone(&self) -> Self {
        WellKnownMessageField(self.0.clone())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct WellKnownEnumFieldDetail<'a, U> {
    enum_field: EnumField<'a, U>,
    well_known_enum: WellKnownEnum,
    descriptor: &'a prost_types::FieldDescriptorProto,
    util: U,
}

#[derive(Debug)]
pub struct WellKnownEnumField<'a, U>(Rc<WellKnownMsgFieldDetail<'a, U>>);
impl<'a, U> WellKnownEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.name()
    }
    pub fn well_known_enum(&self) -> WellKnownEnum {
        self.0.well_known_enum
    }
    pub fn well_known_type(&self) -> WellKnownType {
        WellKnownType::Enum(self.well_known_enum())
    }
    pub fn oneof(&self) -> Oneof<'a, U> {
        self.0.oneof.upgrade()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for WellKnownEnumField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.name()
    }
}
impl<'a, U> FullyQualified for WellKnownEnumField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fully_qualified_name()
    }
}

impl<'a, U> Clone for WellKnownEnumField<'a, U> {
    fn clone(&self) -> Self {
        WellKnownEnumField(self.0.clone())
    }
}
