use std::cell::RefCell;
use std::rc::Rc;

use crate::field::descriptor::{Label, Type};
use crate::proto::Syntax;
use crate::WellKnownType;

use super::{FieldOptions, Scalar};

/// Describes a field within a message.
#[derive(Debug)]
pub struct FieldDescriptor<'a, U> {
    desc: &'a prost_types::FieldDescriptorProto,
    opts: FieldOptions<'a, U>,
}

impl<'a, U> FieldDescriptor<'a, U> {
    pub fn new(desc: &'a prost_types::FieldDescriptorProto) -> Self {
        let opts = FieldOptions::new(desc.options.as_ref().expect("Field did not have options"));
        Self { desc, opts }
    }

    pub fn name(&self) -> &str {
        self.desc.name()
    }
    pub fn number(&self) -> i32 {
        self.desc.number()
    }
    pub fn label(&self) -> Label {
        Label::from(self.desc.label())
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        todo!()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.well_known_type().is_some()
    }

    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of Enum, Message or Group.
    pub fn r#type(&self) -> Type {
        Type::from(self.desc.r#type())
    }

    pub fn is_embed(&self) -> bool {
        matches!(self.r#type(), Type::Message)
    }

    pub fn is_enum(&self) -> bool {
        matches!(self.r#type(), Type::Enum)
    }

    pub fn is_scalar(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Double))
    }
    pub fn is_float(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Float))
    }

    pub fn is_int64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Int64))
    }

    pub fn is_uint64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Uint64))
    }

    pub fn is_int32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Int32))
    }

    pub fn is_fixed64(&self) -> bool {
        if let Type::Scalar(Scalar::Fixed64) = self.r#type() {
            true
        } else {
            false
        }
    }

    pub fn is_fixed32(&self) -> bool {
        if let Type::Scalar(Scalar::Fixed32) = self.r#type() {
            true
        } else {
            false
        }
    }

    pub fn is_bool(&self) -> bool {
        if let Type::Scalar(Scalar::Bool) = self.r#type() {
            true
        } else {
            false
        }
    }

    pub fn is_string(&self) -> bool {
        if let Type::Scalar(Scalar::String) = self.r#type() {
            true
        } else {
            false
        }
    }

    pub fn is_bytes(&self) -> bool {
        if let Type::Scalar(Scalar::Bytes) = self.r#type() {
            true
        } else {
            false
        }
    }

    pub fn is_uint32(&self) -> bool {
        if let Type::Scalar(Scalar::Uint32) = self.r#type() {
            true
        } else {
            false
        }
    }

    pub fn is_sfixed32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sfixed32))
    }

    pub fn is_sfixed64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sfixed64))
    }

    pub fn is_sint32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sint32))
    }

    pub fn is_sint64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sint64))
    }
    pub fn is_message(&self) -> bool {
        matches!(self.r#type(), Type::Message)
    }

    pub fn is_repeated(&self) -> bool {
        self.label() == Label::Repeated
    }

    /// alias for `r#type`
    ///
    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of Enum, Message or Group.
    pub fn proto_type(&self) -> Type {
        return self.r#type();
    }

    /// For message and enum types, this is the name of the type.  If the name
    /// starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
    /// rules are used to find the type (i.e. first the nested types within this
    /// message are searched, then within the parent, on up to the root
    /// namespace).
    pub fn type_name(&self) -> &str {
        self.desc.type_name()
    }

    /// For extensions, this is the name of the type being extended.  It is
    /// resolved in the same manner as `proto_type_name`.
    pub fn extendee(&self) -> &str {
        self.desc.extendee()
    }
    /// For numeric types, contains the original text representation of the value.
    /// For booleans, "true" or "false".
    /// For strings, contains the default text contents (not escaped in any way).
    /// For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
    pub fn default_value(&self) -> &str {
        self.desc.default_value()
    }
    /// If set, gives the index of a oneof in the containing type's oneof_decl
    /// list.
    ///
    /// This field is a member of that oneof.
    pub fn oneof_index(&self) -> i32 {
        self.desc.oneof_index()
    }

    /// JSON name of this field. The value is set by protocol compiler. If the
    /// user has set a "json_name" option on this field, that option's value
    /// will be used. Otherwise, it's deduced from the field's name by converting
    /// it to camelCase.
    pub fn json_name(&self) -> &str {
        self.desc.json_name()
    }
    pub fn options(&self) -> FieldOptions<'a, U> {
        self.opts.clone()
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
        self.desc.proto3_optional()
    }
    /// returns `true` if:
    ///
    /// - `syntax` is `Syntax::Proto3` and `proto3_optional` is `true`
    /// - `syntax` is `Syntax::Proto2` and `label` is `Label::Optional`.
    pub fn is_marked_optional(&self, syntax: Syntax) -> bool {
        match syntax {
            Syntax::Proto2 => self.label() == Label::Optional,
            Syntax::Proto3 => self.proto3_optional(),
        }
    }
    pub fn is_required(&self, syntax: Syntax) -> bool {
        return syntax.supports_required_prefix() && self.label() == Label::Required;
    }
}

impl<'a, U> Clone for FieldDescriptor<'a, U> {
    fn clone(&self) -> Self {
        FieldDescriptor {
            desc: self.desc,
            opts: self.opts.clone(),
        }
    }
}
