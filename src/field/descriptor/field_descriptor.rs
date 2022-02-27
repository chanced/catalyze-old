use crate::field::descriptor::{Label, Type};
use crate::proto::Syntax;
use crate::util::Util;

use super::FieldOptions;

/// Describes a field within a message.
#[derive(Debug)]
pub struct FieldDescriptor<'a, U> {
    desc: &'a prost_types::FieldDescriptorProto,
    util: Util<U>,
}

impl<'a, U> FieldDescriptor<'a, U> {
    pub fn new(desc: &'a prost_types::FieldDescriptorProto, util: Util<U>) -> Self {
        Self { desc, util }
    }

    pub fn name(&self) -> &str {
        self.desc.name()
    }
    pub fn number(&self) -> i32 {
        self.desc.number()
    }
    pub fn label(&self) -> Label {
        self.desc.label()
    }
    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
    pub fn r#type(&self) -> Type {
        Type::from(self.desc.r#type())
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
        let x = self.desc.options();
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
    pub fn is_optional(&self, syntax: Syntax) -> bool {
        match syntax {
            Syntax::Proto2 => self.label() == Label::Optional,
            Syntax::Proto3 => self.proto3_optional(),
        }
    }
}

impl<'a, U> Clone for FieldDescriptor<'a, U> {
    fn clone(&self) -> Self {
        FieldDescriptor {
            desc: self.desc,
            util: self.util.clone(),
        }
    }
}
