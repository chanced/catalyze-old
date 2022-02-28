use std::rc::{Rc, Weak};

use crate::{
    name::Named,
    traits::{Downgrade, Upgrade},
    FullyQualified, Name, Oneof, WellKnownEnum, WellKnownMessage, WellKnownType,
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
    pub fn fully_qualified_name(&self) -> &str {
        match self {
            WellKnownTypeField::Message(m) => m.fully_qualified_name(),
            WellKnownTypeField::Enum(e) => e.fully_qualified_name(),
        }
    }
}
impl<'a, U> FullyQualified for WellKnownTypeField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
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
impl<'a, U> Downgrade for WellKnownTypeField<'a, U> {
    type Target = WeakWellKnownTypeField<'a, U>;

    fn downgrade(self) -> Self::Target {
        match self {
            WellKnownTypeField::Message(m) => WeakWellKnownTypeField::Message(m.downgrade()),
            WellKnownTypeField::Enum(e) => WeakWellKnownTypeField::Enum(e.downgrade()),
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
    pub fn fully_qualified_name(&self) -> &str {
        self.0.msg_fld.fully_qualified_name()
    }
}
impl<'a, U> FullyQualified for WellKnownMessageField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
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
impl<'a, U> Downgrade for WellKnownMessageField<'a, U> {
    type Target = WeakWellKnownMessageField<'a, U>;

    fn downgrade(self) -> Self::Target {
        WeakWellKnownMessageField(Rc::downgrade(&self.0))
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
pub struct WellKnownEnumField<'a, U>(Rc<WellKnownEnumFieldDetail<'a, U>>);
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
    pub fn fully_qualified_name(&self) -> &str {
        self.0.fully_qualified_name()
    }
}
impl<'a, U> Named<U> for WellKnownEnumField<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.enum_field.fully_qualified_name()
    }
}
impl<'a, U> FullyQualified for WellKnownEnumField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        self.0.enum_field.fully_qualified_name()
    }
}

impl<'a, U> Clone for WellKnownEnumField<'a, U> {
    fn clone(&self) -> Self {
        WellKnownEnumField(self.0.clone())
    }
}

#[derive(Clone, Debug)]
pub(crate) enum WeakWellKnownTypeField<'a, U> {
    Message(WeakWellKnownMessageField<'a, U>),
    Enum(WeakWellKnownEnumField<'a, U>),
}
impl<'a, U> Upgrade for WeakWellKnownTypeField<'a, U> {
    type Target = WellKnownTypeField<'a, U>;

    fn upgrade(self) -> Self::Target {
        match self {
            WeakWellKnownTypeField::Message(m) => WellKnownTypeField::Message(m.upgrade()),
            WeakWellKnownTypeField::Enum(e) => WellKnownTypeField::Enum(e.upgrade()),
        }
    }
}

#[derive(Debug)]
pub struct WeakWellKnownEnumField<'a, U>(Weak<WellKnownEnumFieldDetail<'a, U>>);
impl<'a, U> Clone for WeakWellKnownEnumField<'a, U> {
    fn clone(&self) -> Self {
        WeakWellKnownEnumField(self.0.clone())
    }
}

impl<'a, U> Upgrade for WeakWellKnownEnumField<'a, U> {
    type Target = WellKnownMessageField<'a, U>;

    fn upgrade(self) -> Self::Target {
        WellKnownMessageField(
            self.0
                .upgrade()
                .expect("failed to upgrade Well-Known Message Field"),
        )
    }
}

#[derive(Debug)]
pub(crate) struct WeakWellKnownMessageField<'a, U>(Weak<WellKnownMsgFieldDetail<'a, U>>);

impl<'a, U> Clone for WeakWellKnownMessageField<'a, U> {
    fn clone(&self) -> Self {
        WeakWellKnownMessageField(self.0.clone())
    }
}
impl<'a, U> Upgrade for WeakWellKnownMessageField<'a, U> {
    type Target = WellKnownMessageField<'a, U>;

    fn upgrade(self) -> Self::Target {
        WellKnownMessageField(
            self.0
                .upgrade()
                .expect("failed to upgrade Well-Known Message Field"),
        )
    }
}
