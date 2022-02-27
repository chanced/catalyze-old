mod enum_field;

pub mod descriptor;
mod map_field;
mod message_field;
mod oneof_field;
mod repeated_field;
mod scalar_field;
mod r#type;
mod wkt_field;
mod label;

pub use enum_field::*;
pub use map_field::*;
pub use message_field::*;
pub use oneof_field::*;
use prost_types::FieldDescriptorProto;
pub use r#type::*;
pub use repeated_field::*;
pub use scalar_field::*;
pub use label::*;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
pub use wkt_field::*;

use crate::{
    proto::{Label, Type},
    traits::Upgrade,
    FullyQualified, IntoNode, Message, Name, Node, NodeAtPath, WeakMessage,
};

pub(crate) type FieldList<'a, U> = Rc<RefCell<Vec<Field<'a, U>>>>;

pub enum FieldType {
    Scalar,
    Message,
    Map,
    Repeated,
    WellKnownType,
}

#[derive(Debug, Clone)]
pub enum Field<'a, U> {
    Enum(EnumField<'a, U>),
    Map(MapField<'a, U>),
    Oneof(OneofField<'a, U>),
    Message(MessageField<'a, U>),
    Repeated(RepeatedField<'a, U>),
    Scalar(ScalarField<'a, U>),
    WellKnownType(WellKnownTypeField<'a, U>),
}

#[derive(Debug, Clone)]
pub(crate) struct FieldDetail<'a, U> {
    name: Name<U>,
    descriptor: &'a FieldDescriptorProto,
    fqn: String,
    containing_message: WeakMessage<'a, U>,
}

impl<'a, U> FieldDetail<'a, U> {
    /// For numeric types, contains the original text representation of the
    /// value. For booleans, "true" or "false". For strings, contains the
    /// default text contents (not escaped in any way). For bytes, contains the
    /// C escaped value. All bytes >= 128 are escaped.
    pub fn default_value(&self) -> &str {
        self.descriptor.default_value
    }

    pub fn proto_name(&self) -> &str {
        self.descriptor.name()
    }

    pub fn number(&self) -> u32 {
        self.descriptor.number()
    }

    pub fn label(&self) -> Label {
        self.descriptor.label()
    }
    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
    pub fn proto_type(&self) -> Type {
        self.descriptor.r#type()
    }

    /// For message and enum types, this is the name of the type.  If the name
    /// starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
    /// rules are used to find the type (i.e. first the nested types within this
    /// message are searched, then within the parent, on up to the root
    /// namespace).
    pub fn proto_type_name(&self) -> &str {
        self.descriptor.type_name()
    }

    /// Returns the value of `extendee`, or the default value if `extendee` is
    /// unset.
    pub fn extendee(&self) -> &str {
        self.descriptor.extendee()
    }

    /// If set, gives the index of a oneof in the containing type's oneof_decl
    /// list.  This field is a member of that oneof.
    pub fn oneof_index(&self) -> i32 {
        self.descriptor.oneof_index()
    }

    /// JSON name of this field. The value is set by protocol compiler. If the
    /// user has set a "json_name" option on this field, that option's value
    /// will be used. Otherwise, it's deduced from the field's name by converting
    /// it to camelCase.
    pub fn json_name(&self) -> &str {
        self.descriptor.json_name()
    }

    /// If true, this is a proto3 "optional". When a proto3 field is optional, it
    /// tracks presence regardless of field type.
    ///
    /// When proto3_optional is true, this field must be belong to a oneof to
    /// signal to old proto3 clients that presence is tracked for this field. This
    /// oneof is known as a "synthetic" oneof, and this field must be its sole
    /// member (each proto3 optional field gets its own synthetic oneof). Synthetic
    /// oneofs exist in the descriptor only, and do not generate any API. Synthetic
    /// oneofs must be ordered after all "real" oneofs.
    ///
    /// For message fields, proto3_optional doesn't create any semantic change,
    /// since non-repeated message fields always track presence. However it still
    /// indicates the semantic detail of whether the user wrote "optional" or not.
    /// This can be useful for round-tripping the .proto file. For consistency we
    /// give message fields a synthetic oneof also, even though it is not required
    /// to track presence. This is especially important because the parser can't
    /// tell if a field is a message or an enum, so it must always create a
    /// synthetic oneof.
    ///
    /// Proto2 optional fields do not set this flag, because they already indicate
    /// optional with `LABEL_OPTIONAL`.
    pub fn proto3_optional(&self) -> bool {
        self.descriptor.proto3_optional()
    }

    pub fn name(&self) -> Name<U> {
        self.name.clone()
    }

    pub fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }

    pub fn containing_message(&self) -> Message<'a, U> {
        self.msg.upgrade()
    }
}

impl<'a, U> Field<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {}
    }
}

impl<'a, U> NodeAtPath<'a, U> for Field<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            return Some(self.into_node());
        } else {
            None
        }
    }
}

impl<'a, U> FullyQualified for FieldDetail<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
}

impl<'a, U> FullyQualified for Field<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            Field::Scalar(f) => f.fully_qualified_name(),
            Field::Message(f) => f.fully_qualified_name(),
            Field::Map(f) => f.fully_qualified_name(),
            Field::Repeated(f) => f.fully_qualified_name(),
            Field::Oneof(f) => f.fully_qualified_name(),
            Field::WellKnownType(f) => f.fully_qualified_name(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct WeakField<'a, U>(Weak<FieldDetail<'a, U>>);
impl<'a, U> Clone for WeakField<'a, U> {
    fn clone(&self) -> Self {
        match self {
            WeakField::Scalar(_) => todo!(),
            WeakField::Message(_) => todo!(),
            WeakField::Map(_) => todo!(),
            WeakField::Repeated(_) => todo!(),
            WeakField::Oneof(_) => todo!(),
            WeakField::WellKnownType(_) => todo!(),
        }
    }
}

impl<'a, U> Upgrade for WeakField<'a, U> {
    type Target = Field<'a, U>;
    fn upgrade(self) -> Self::Target {
        match self {
            WeakField::Scalar(f) => Field::Scalar(f.upgrade()),
            WeakField::Message(f) => Field::Message(f.upgrade()),
            WeakField::Map(f) => Field::Map(f.upgrade()),
            WeakField::Repeated(_) => todo!(),
            WeakField::Oneof(_) => todo!(),
            WeakField::WellKnownType(_) => todo!(),
        }
    }
}

enum WeakField<'a, U> {
    Scalar(WeakScalarFieldDetail<'a, U>),
    Message(WeakMessageField<'a, U>),
    Map(WeakMapField<'a, U>),
    Repeated(WeakRepeatedField<'a, U>),
    Oneof(WeakOneofField<'a, U>),
    WellKnownType(WeakWellKnownTypeField<'a, U>),
}
