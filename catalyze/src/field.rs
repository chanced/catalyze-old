mod enum_field;

mod embed_field;
mod map_field;
mod oneof_field;
mod repeated_field;
mod scalar_field;

pub use embed_field::*;
pub use enum_field::*;
pub use map_field::*;
pub use oneof_field::*;
use protobuf::reflect::FieldDescriptor;
pub use repeated_field::*;
pub use scalar_field::*;

use crate::{
    uninterpreted_option::UninterpretedOption, CType, Comments, Enum, Error, File, FileRefs,
    InvalidMapEntryReason, JsType, Kind, Message, Node, Oneof, Package, Scalar, Syntax, Type,
    WeakMessage, WellKnownType,
};
use ::std::option::Option;
use std::{cell::RefCell, convert::From};

#[derive(Debug, Clone, Copy)]
struct Options {
    // message fields
    ///  The ctype option instructs the C++ code generator to use a different
    ///  representation of the field than it normally would.  See the specific
    ///  options below.  This option is not yet implemented in the open source
    ///  release -- sorry, we'll try to include it in a future version!
    // @@protoc_insertion_point(field:google.protobuf.FieldOptions.ctype)
    pub ctype: Option<CType>,
    ///  The packed option can be enabled for repeated primitive fields to enable
    ///  a more efficient representation on the wire. Rather than repeatedly
    ///  writing the tag and type for each element, the entire array is encoded as
    ///  a single length-delimited blob. In proto3, only explicit setting it to
    ///  false will avoid using packed encoding.
    // @@protoc_insertion_point(field:google.protobuf.FieldOptions.packed)
    pub packed: Option<bool>,
    ///  The jstype option determines the JavaScript type used for values of the
    ///  field.  The option is permitted only for 64 bit integral and fixed types
    ///  (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
    ///  is represented as JavaScript string, which avoids loss of precision that
    ///  can happen when a large value is converted to a floating point JavaScript.
    ///  Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
    ///  use the JavaScript "number" type.  The behavior of the default option
    ///  JS_NORMAL is implementation dependent.
    ///
    ///  This option is an enum to permit additional types to be added, e.g.
    ///  goog.math.Integer.
    jstype: Option<protobuf::EnumOrUnknown<protobuf::descriptor::field_options::JSType>>,
    ///  Should this field be parsed lazily?  Lazy applies only to message-type
    ///  fields.  It means that when the outer message is initially parsed, the
    ///  inner message's contents will not be parsed but instead stored in encoded
    ///  form.  The inner message will actually be parsed when it is first accessed.
    ///
    ///  This is only a hint.  Implementations are free to choose whether to use
    ///  eager or lazy parsing regardless of the value of this option.  However,
    ///  setting this option true suggests that the protocol author believes that
    ///  using lazy parsing on this field is worth the additional bookkeeping
    ///  overhead typically needed to implement it.
    ///
    ///  This option does not affect the public interface of any generated code;
    ///  all method signatures remain the same.  Furthermore, thread-safety of the
    ///  interface is not affected by this option; const methods remain safe to
    ///  call from multiple threads concurrently, while non-const methods continue
    ///  to require exclusive access.
    ///
    ///
    ///  Note that implementations may choose not to check required fields within
    ///  a lazy sub-message.  That is, calling IsInitialized() on the outer message
    ///  may return true even if the inner message has missing required fields.
    ///  This is necessary because otherwise the inner message would have to be
    ///  parsed in order to perform the check, defeating the purpose of lazy
    ///  parsing.  An implementation which chooses not to check required fields
    ///  must be consistent about it.  That is, for any particular sub-message, the
    ///  implementation must either *always* check its required fields, or *never*
    ///  check its required fields, regardless of whether or not the message has
    ///  been parsed.
    lazy: Option<bool>,
    ///  Is this field deprecated?
    ///  Depending on the target platform, this can emit Deprecated annotations
    ///  for accessors, or it will be completely ignored; in the very least, this
    ///  is a formalization for deprecating fields.
    deprecated: Option<bool>,
    ///  For Google-internal migration only. Do not use.
    weak: Option<bool>,
    ///  The parser stores options it doesn't recognize here. See above.
    uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
    // special fields
    // @@protoc_insertion_point(special_field:google.protobuf.FieldOptions.special_fields)
    special_fields: protobuf::SpecialFields,
}
impl Options {
    pub(crate) fn new(opts: Option<&'a protobuf::descriptor::FieldOptions>) -> Self {
        let Some(opts) = opts else {
            return Self::default();
        };

        Self {
            ctype: opts.ctype,
            deprecated: opts.deprecated,
            jstype: opts.jstype,
            lazy: opts.lazy,
            packed: opts.packed,
        }
    }
    /// The ctype option instructs the C++ code generator to use a different
    /// representation of the field than it normally would.  See the specific
    /// options below.  This option is not yet implemented in the open source
    /// release -- sorry, we'll try to include it in a future version!
    pub fn ctype(&self) -> CType {
        CType::from(self.opts().ctype())
    }
    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub fn packed(&self) -> bool {
        self.opts().packed()
    }
    /// The jstype option determines the JavaScript type used for values of the
    /// field.  The option is permitted only for 64 bit integral and fixed types
    /// (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
    /// is represented as JavaScript string, which avoids loss of precision that
    /// can happen when a large value is converted to a floating point JavaScript.
    /// Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
    /// use the JavaScript "number" type.  The behavior of the default option
    /// JS_NORMAL is implementation dependent.
    ///
    /// This option is an enum to permit additional types to be added, e.g.
    /// goog.math.Integer.
    pub fn jstype(&self) -> JsType {
        self.opts().jstype().into()
    }
    /// Should this field be parsed lazily?  Lazy applies only to message-type
    /// fields.  It means that when the outer message is initially parsed, the
    /// inner message's contents will not be parsed but instead stored in encoded
    /// form.  The inner message will actually be parsed when it is first accessed.
    ///
    /// This is only a hint.  Implementations are free to choose whether to use
    /// eager or lazy parsing regardless of the value of this option.  However,
    /// setting this option true suggests that the protocol author believes that
    /// using lazy parsing on this field is worth the additional bookkeeping
    /// overhead typically needed to implement it.
    ///
    /// This option does not affect the public interface of any generated code;
    /// all method signatures remain the same.  Furthermore, thread-safety of the
    /// interface is not affected by this option; const methods remain safe to
    /// call from multiple threads concurrently, while non-const methods continue
    /// to require exclusive access.
    ///
    ///
    /// Note that implementations may choose not to check required fields within
    /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
    /// may return true even if the inner message has missing required fields.
    /// This is necessary because otherwise the inner message would have to be
    /// parsed in order to perform the check, defeating the purpose of lazy
    /// parsing.  An implementation which chooses not to check required fields
    /// must be consistent about it.  That is, for any particular sub-message, the
    /// implementation must either *always* check its required fields, or *never*
    /// check its required fields, regardless of whether or not the message has
    /// been parsed.
    pub fn lazy(&self) -> bool {
        self.opts().lazy()
    }
    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// For Google-internal migration only. Do not use.
    pub fn weak(&self) -> bool {
        self.opts().weak()
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> &[UninterpretedOption] {
        (&self.opts().uninterpreted_option).into()
    }

    fn opts(&self) -> Option<&protobuf::descriptor::FieldOptions> {
        self.opts.unwrap_or(&DEFAULT_FIELD_OPTIONS)
    }
}
impl From<Option<&'a protobuf::descriptor::FieldOptions>> for Options {
    fn from(opts: Option<&'a protobuf::descriptor::FieldOptions>) -> Self {
        Self { opts }
    }
}

#[derive(Debug, Clone)]
pub enum Field {
    Embed(EmbedField),
    Enum(EnumField),
    Map(MapField),
    Oneof(OneofField),
    Repeated(RepeatedField),
    Scalar(ScalarField),
}

impl From<EmbedField> for Field {
    fn from(v: EmbedField) -> Self {
        Self::Embed(v)
    }
}

impl From<OneofField> for Field {
    fn from(v: OneofField) -> Self {
        Self::Oneof(v)
    }
}

impl From<RepeatedField> for Field {
    fn from(v: RepeatedField) -> Self {
        Self::Repeated(v)
    }
}

impl From<ScalarField> for Field {
    fn from(f: ScalarField) -> Self {
        Field::Scalar(f)
    }
}

impl From<EnumField> for Field {
    fn from(f: EnumField) -> Self {
        Field::Enum(f)
    }
}
impl From<MapField> for Field {
    fn from(f: MapField) -> Self {
        Field::Map(f)
    }
}
impl From<&ScalarField> for Field {
    fn from(f: &ScalarField) -> Self {
        f.clone().into()
    }
}

impl From<&EnumField> for Field {
    fn from(f: &EnumField) -> Self {
        f.clone().into()
    }
}
impl From<&MapField> for Field {
    fn from(f: &MapField) -> Self {
        f.clone().into()
    }
}

impl Field {
    pub fn new(desc: FieldDescriptor, msg: Message, oneof: Option<Oneof>) -> Result<Field, Error> {
        let detail = FieldDetail::new(desc, msg, None);
        if desc.type_().is_group() {
            return Err(Error::group_not_supported(detail.fully_qualified_name()));
        }
        if let Some(oneof) = oneof {
            OneofField::new(detail, oneof)
        } else if detail.is_repeated() {
            RepeatedField::new(detail)
        } else {
            match detail.value_type() {
                Type::Scalar(_) => Ok(ScalarField::new(detail)),
                Type::Enum(_) => Ok(EnumField::new(detail)),
                Type::Message(_) => Ok(EmbedField::new(detail)),
                Type::Group => Err(Error::group_not_supported(detail.fully_qualified_name())),
            }
        }
    }
    #[inline]
    pub fn fully_qualified_name(&self) -> &str {
        match self {
            Field::Enum(f) => f.fully_qualified_name(),
            Field::Map(f) => f.fully_qualified_name(),
            Field::Embed(f) => f.fully_qualified_name(),
            Field::Oneof(f) => f.fully_qualified_name(),
            Field::Repeated(f) => f.fully_qualified_name(),
            Field::Scalar(f) => f.fully_qualified_name(),
        }
    }
    #[inline]
    pub fn message(&self) -> Message {
        match self {
            Self::Embed(f) => f.message(),
            Self::Enum(f) => f.message(),
            Self::Map(f) => f.message(),
            Self::Oneof(f) => f.message(),
            Self::Repeated(f) => f.message(),
            Self::Scalar(f) => f.message(),
        }
    }
    pub fn value_type(&self) -> Type {
        match self {
            Self::Embed(f) => f.value_type(),
            Self::Enum(f) => f.value_type(),
            Self::Map(f) => f.value_type(),
            Self::Oneof(f) => f.value_type(),
            Self::Repeated(f) => f.value_type(),
            Self::Scalar(f) => f.value_type(),
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Field::Embed(f) => f.name(),
            Field::Enum(f) => f.name(),
            Field::Map(f) => f.name(),
            Field::Oneof(f) => f.name(),
            Field::Repeated(f) => f.name(),
            Field::Scalar(f) => f.name(),
        }
    }
    pub fn descriptor(&self) -> FieldDescriptor {
        match self {
            Field::Embed(f) => f.descriptor(),
            Field::Enum(f) => f.descriptor(),
            Field::Map(f) => f.descriptor(),
            Field::Oneof(f) => f.descriptor(),
            Field::Repeated(f) => f.descriptor(),
            Field::Scalar(f) => f.descriptor(),
        }
    }

    pub fn syntax(&self) -> Syntax {
        match self {
            Field::Embed(f) => f.syntax(),
            Field::Enum(f) => f.syntax(),
            Field::Map(f) => f.syntax(),
            Field::Oneof(f) => f.syntax(),
            Field::Repeated(f) => f.syntax(),
            Field::Scalar(f) => f.syntax(),
        }
    }

    pub fn comments(&self) -> Comments {
        match self {
            Field::Embed(f) => f.comments(),
            Field::Enum(f) => f.comments(),
            Field::Map(f) => f.comments(),
            Field::Oneof(f) => f.comments(),
            Field::Repeated(f) => f.comments(),
            Field::Scalar(f) => f.comments(),
        }
    }
    pub fn has_import(&self) -> bool {
        match self {
            Field::Embed(f) => f.has_import(),
            Field::Enum(f) => f.has_import(),
            Field::Map(f) => f.has_import(),
            Field::Oneof(f) => f.has_import(),
            Field::Repeated(f) => f.has_import(),
            Field::Scalar(_) => false,
        }
    }

    pub fn imports(&self) -> FileRefs {
        match self {
            Field::Embed(f) => f.imports(),
            Field::Enum(f) => f.imports(),
            Field::Map(f) => f.imports(),
            Field::Oneof(f) => f.imports(),
            Field::Repeated(f) => f.imports(),
            Field::Scalar(_) => FileRefs::empty(),
        }
    }
    pub fn number(&self) -> i32 {
        match self {
            Field::Embed(f) => f.number(),
            Field::Enum(f) => f.number(),
            Field::Map(f) => f.number(),
            Field::Oneof(f) => f.number(),
            Field::Repeated(f) => f.number(),
            Field::Scalar(f) => f.number(),
        }
    }
    pub fn build_target(&self) -> bool {
        match self {
            Field::Embed(f) => f.build_target(),
            Field::Enum(f) => f.build_target(),
            Field::Map(f) => f.build_target(),
            Field::Oneof(f) => f.build_target(),
            Field::Repeated(f) => f.build_target(),
            Field::Scalar(f) => f.build_target(),
        }
    }

    /// Returns `Option::Some(Enum)` if this field contains an `Enum`.
    pub fn enum_(&self) -> Option<Enum> {
        match self {
            Field::Enum(f) => Some(f.enum_()),
            Field::Map(f) => f.enum_(),
            Field::Oneof(f) => f.enum_(),
            Field::Repeated(f) => f.enum_(),
            _ => None,
        }
    }
    /// Returns `Option::Some(Message)` if this field should hold an embedded
    /// `Message`.
    pub fn embed(&self) -> Option<Message> {
        match self {
            Field::Embed(f) => Some(f.embed()),
            Field::Map(f) => f.embed(),
            Field::Oneof(f) => f.embed(),
            Field::Repeated(f) => f.embed(),
            _ => None,
        }
    }
    /// Returns `true` if this `Field` is a [`Scalar`] type.
    pub fn scalar(&self) -> Option<Scalar> {
        match self {
            Field::Map(f) => f.scalar(),
            Field::Oneof(f) => f.scalar(),
            Field::Repeated(f) => f.scalar(),
            Field::Scalar(f) => Some(f.scalar()),
            _ => None,
        }
    }

    /// Returns `true` if this field is marked repeated.
    pub fn is_repeated(&self) -> bool {
        matches!(self, Field::Repeated(_))
    }

    /// Returns `true` if this field should hold an embedded `Message`.
    pub fn is_embed(&self) -> bool {
        match self {
            Field::Embed(_) => true,
            Field::Map(f) => f.is_embed(),
            Field::Oneof(f) => f.is_embed(),
            Field::Repeated(f) => f.is_embed(),
            _ => false,
        }
    }
    pub fn is_in_oneof(&self) -> bool {
        matches!(self, Field::Oneof(_))
    }

    pub fn is_in_real_oneof(&self) -> bool {
        match self {
            Field::Oneof(f) => f.is_in_real_oneof(),
            _ => false,
        }
    }
    pub fn is_in_synthetic_oneof(&self) -> bool {
        match self {
            Field::Oneof(f) => f.is_in_synthetic_oneof(),
            _ => false,
        }
    }
    pub fn is_well_known_type(&self) -> bool {
        match self {
            Field::Embed(f) => f.is_well_known_type(),
            Field::Enum(f) => f.is_well_known_type(),
            Field::Map(f) => f.is_well_known_type(),
            Field::Oneof(f) => f.is_well_known_type(),
            Field::Repeated(f) => f.is_well_known_type(),
            _ => false,
        }
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        match self {
            Field::Embed(f) => f.well_known_type(),
            Field::Enum(f) => f.well_known_type(),
            Field::Map(f) => f.well_known_type(),
            Field::Oneof(f) => f.well_known_type(),
            Field::Repeated(f) => f.well_known_type(),
            _ => None,
        }
    }
    /// Indicates whether or not the field is labeled as a required field. This
    /// will only be `true` if the syntax is proto2.
    pub fn is_marked_required(&self) -> bool {
        match self {
            Field::Embed(f) => f.is_marked_required(),
            Field::Enum(f) => f.is_marked_required(),
            Field::Map(f) => f.is_marked_required(),
            Field::Oneof(f) => f.is_marked_required(),
            Field::Repeated(f) => f.is_marked_required(),
            Field::Scalar(f) => f.is_marked_required(),
        }
    }
    pub fn is_marked_optional(&self) -> bool {
        match self {
            Field::Embed(f) => f.is_marked_optional(),
            Field::Enum(f) => f.is_marked_optional(),
            Field::Scalar(f) => f.is_marked_optional(),
            _ => false,
        }
    }
    pub fn is_scalar(&self) -> bool {
        match self {
            Field::Map(f) => f.is_scalar(),
            Field::Oneof(f) => f.is_scalar(),
            Field::Repeated(f) => f.is_scalar(),
            Field::Scalar(_) => true,
            _ => false,
        }
    }
    pub fn is_enum(&self) -> bool {
        match self {
            Field::Enum(_) => true,
            Field::Map(f) => f.is_enum(),
            Field::Oneof(f) => f.is_enum(),
            Field::Repeated(f) => f.is_enum(),
            _ => false,
        }
    }

    /// Returns `true` for all fields that have explicit presence.
    ///
    /// ---
    /// ## Proto2
    /// Field type                                   | Explicit Presence
    /// -------------------------------------------- | :-----------------:
    /// Singular numeric (integer or floating point) | ✔️
    /// Singular enum                                | ✔️
    /// Singular string or bytes                     | ✔️
    /// Singular message                             | ✔️
    /// Repeated                                     |
    /// Oneofs                                       | ✔️
    /// Maps                                         |
    ///
    /// ---
    /// ## Proto3
    /// Field type                                   | `optional` | Explicit Presence
    /// -------------------------------------------- |:----------: | :-----------------:
    /// Singular numeric (integer or floating point) | No         |
    /// Singular enum                                | No         |
    /// Singular string or bytes                     | No         |
    /// Singular numeric (integer or floating point) | Yes        | ✔️
    /// Singular enum                                | Yes        | ✔️
    /// Singular string or bytes                     | Yes        | ✔️
    /// Singular message                             | Yes        | ✔️
    /// Singular message                             | No         | ✔️
    /// Repeated                                     | N/A        |
    /// Oneofs                                       | N/A        | ✔️
    /// Maps                                         | N/A        |
    ///
    /// See:
    /// - <https://github.com/protocolbuffers/protobuf/blob/v3.17.0/docs/field_presence.md>
    /// - <https://github.com/protocolbuffers/protobuf/blob/master/docs/implementing_proto3_presence.md>
    pub fn has_presence(&self) -> bool {
        match self {
            Field::Embed(f) => f.has_presence(),
            Field::Enum(f) => f.has_presence(),
            Field::Map(f) => f.has_presence(),
            Field::Oneof(f) => f.has_presence(),
            Field::Repeated(f) => f.has_presence(),
            Field::Scalar(f) => f.has_presence(),
        }
    }
    pub fn is_map(&self) -> bool {
        matches!(self, Field::Map(_))
    }

    pub fn file(&self) -> File {
        match self {
            Field::Embed(f) => f.file(),
            Field::Enum(f) => f.file(),
            Field::Map(f) => f.file(),
            Field::Oneof(f) => f.file(),
            Field::Repeated(f) => f.file(),
            Field::Scalar(f) => f.file(),
        }
    }
    pub fn package(&self) -> Package {
        match self {
            Field::Embed(f) => f.package(),
            Field::Enum(f) => f.package(),
            Field::Map(f) => f.package(),
            Field::Oneof(f) => f.package(),
            Field::Repeated(f) => f.package(),
            Field::Scalar(f) => f.package(),
        }
    }
    pub(crate) fn set_comments(&self, comments: Comments) {
        match self {
            Field::Embed(f) => f.set_comments(comments),
            Field::Enum(f) => f.set_comments(comments),
            Field::Map(f) => f.set_comments(comments),
            Field::Oneof(f) => f.set_comments(comments),
            Field::Repeated(f) => f.set_comments(comments),
            Field::Scalar(f) => f.set_comments(comments),
        }
    }

    pub(crate) fn set_value(&self, value: Node) -> Result<(), Error> {
        match self {
            Field::Embed(f) => f.set_value(value),
            Field::Enum(f) => f.set_value(value),
            Field::Map(f) => f.set_value(value),
            Field::Oneof(f) => f.set_value(value),
            Field::Repeated(f) => f.set_value(value),
            _ => panic!("set_value not supported for {:?}", self),
        }
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node> {
        if path.is_empty() {
            Some(self.into())
        } else {
            None
        }
    }

    // pub(crate) fn is_obj_value(&self) -> bool {
    //     match self {
    //         Field::Embed(_) => true,
    //         Field::Enum(_) => true,
    //         Field::Map(m) => match m {
    //             MapField::Enum(_) => true,
    //             MapField::Embed(_) => true,
    //             MapField::Scalar(_) => false,
    //         },
    //         Field::Oneof(o) => match o {
    //             OneofField::Embed(_) => true,
    //             OneofField::Enum(_) => true,
    //             OneofField::Scalar(_) => false,
    //         },
    //         Field::Repeated(r) => match r {
    //             RepeatedField::Enum(_) => true,
    //             RepeatedField::Embed(_) => true,
    //             RepeatedField::Scalar(_) => false,
    //         },
    //         Field::Scalar(_) => false,
    //     }
    // }

    /// The ctype option instructs the C++ code generator to use a different
    /// representation of the field than it normally would.  See the specific
    /// options below.  This option is not yet implemented in the open source
    /// release -- sorry, we'll try to include it in a future version!
    pub fn ctype(&self) -> CType {
        self.descriptor().options().ctype()
    }
    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub fn is_packed(&self) -> bool {
        match self {
            Field::Embed(f) => f.is_packed(),
            Field::Enum(f) => f.is_packed(),
            Field::Map(f) => f.is_packed(),
            Field::Oneof(f) => f.is_packed(),
            Field::Repeated(f) => f.is_packed(),
            Field::Scalar(f) => f.is_packed(),
        }
    }
    /// The jstype option determines the JavaScript type used for values of the
    /// field.  The option is permitted only for 64 bit integral and fixed types
    /// (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
    /// is represented as JavaScript string, which avoids loss of precision that
    /// can happen when a large value is converted to a floating point JavaScript.
    /// Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
    /// use the JavaScript "number" type.  The behavior of the default option
    /// JS_NORMAL is implementation dependent.
    ///
    /// This option is an enum to permit additional types to be added, e.g.
    /// goog.math.Integer.
    pub fn jstype(&self) -> JsType {
        match self {
            Field::Embed(f) => f.jstype(),
            Field::Enum(f) => f.jstype(),
            Field::Map(f) => f.jstype(),
            Field::Oneof(f) => f.jstype(),
            Field::Repeated(f) => f.jstype(),
            Field::Scalar(f) => f.jstype(),
        }
    }
    /// Should this field be parsed lazily?  Lazy applies only to message-type
    /// fields.  It means that when the outer message is initially parsed, the
    /// inner message's contents will not be parsed but instead stored in encoded
    /// form.  The inner message will actually be parsed when it is first accessed.
    ///
    /// This is only a hint.  Implementations are free to choose whether to use
    /// eager or lazy parsing regardless of the value of this option.  However,
    /// setting this option true suggests that the protocol author believes that
    /// using lazy parsing on this field is worth the additional bookkeeping
    /// overhead typically needed to implement it.
    ///
    /// This option does not affect the public interface of any generated code;
    /// all method signatures remain the same.  Furthermore, thread-safety of the
    /// interface is not affected by this option; const methods remain safe to
    /// call from multiple threads concurrently, while non-const methods continue
    /// to require exclusive access.
    ///
    ///
    /// Note that implementations may choose not to check required fields within
    /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
    /// may return true even if the inner message has missing required fields.
    /// This is necessary because otherwise the inner message would have to be
    /// parsed in order to perform the check, defeating the purpose of lazy
    /// parsing.  An implementation which chooses not to check required fields
    /// must be consistent about it.  That is, for any particular sub-message, the
    /// implementation must either *always* check its required fields, or *never*
    /// check its required fields, regardless of whether or not the message has
    /// been parsed.
    pub fn is_lazy(&self) -> bool {
        match self {
            Field::Embed(f) => f.is_lazy(),
            Field::Enum(f) => f.is_lazy(),
            Field::Map(f) => f.is_lazy(),
            Field::Oneof(f) => f.is_lazy(),
            Field::Repeated(f) => f.is_lazy(),
            Field::Scalar(f) => f.is_lazy(),
        }
    }
    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub fn is_deprecated(&self) -> bool {
        match self {
            Field::Embed(f) => f.is_deprecated(),
            Field::Enum(f) => f.is_deprecated(),
            Field::Map(f) => f.is_deprecated(),
            Field::Oneof(f) => f.is_deprecated(),
            Field::Repeated(f) => f.is_deprecated(),
            Field::Scalar(f) => f.is_deprecated(),
        }
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> &[UninterpretedOption] {
        match self {
            Field::Embed(f) => f.uninterpreted_options(),
            Field::Enum(f) => f.uninterpreted_options(),
            Field::Map(f) => f.uninterpreted_options(),
            Field::Oneof(f) => f.uninterpreted_options(),
            Field::Repeated(f) => f.uninterpreted_options(),
            Field::Scalar(f) => f.uninterpreted_options(),
        }
    }
    pub(crate) fn into_map(self) -> Result<Self, Error> {
        let embed = self.embed().ok_or_else(|| {
            Error::invalid_map_entry(
                InvalidMapEntryReason::FieldNotEmbed,
                self.fully_qualified_name(),
                self.name(),
                self.syntax(),
            )
        })?;
        if !embed.is_map_entry() {
            return Err(Error::invalid_map_entry(
                InvalidMapEntryReason::EmbedNotMap,
                self.fully_qualified_name(),
                self.name(),
                self.syntax(),
            ));
        }
        let msg = self.message();
        let fd = FieldDetail::new(self.descriptor(), msg, Some(embed));
        fd.set_comments(self.comments());
        let mf = Field::Map(MapField::new(fd)?);
        Ok(mf)
    }

    #[must_use]
    pub fn as_embed(&self) -> Option<&EmbedField> {
        if let Self::Embed(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_embed(self) -> Result<EmbedField, Self> {
        if let Self::Embed(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    #[must_use]
    pub fn as_enum(&self) -> Option<&EnumField> {
        if let Self::Enum(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_enum(self) -> Result<EnumField, Self> {
        if let Self::Enum(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    #[must_use]
    pub fn as_map(&self) -> Option<&MapField> {
        if let Self::Map(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_map(self) -> Result<MapField, Self> {
        if let Self::Map(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the field is [`Oneof`].
    ///
    /// [`Oneof`]: Field::Oneof
    #[must_use]
    pub fn is_oneof(&self) -> bool {
        matches!(self, Self::Oneof(..))
    }

    #[must_use]
    pub fn as_oneof(&self) -> Option<&OneofField> {
        if let Self::Oneof(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_oneof(self) -> Result<OneofField, Self> {
        if let Self::Oneof(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    #[must_use]
    pub fn as_repeated(&self) -> Option<&RepeatedField> {
        if let Self::Repeated(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_repeated(self) -> Result<RepeatedField, Self> {
        if let Self::Repeated(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    #[must_use]
    pub fn as_scalar(&self) -> Option<&ScalarField> {
        if let Self::Scalar(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_scalar(self) -> Result<ScalarField, Self> {
        if let Self::Scalar(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FieldDetail {
    msg: WeakMessage,
    fqn: String,
    syntax: Syntax,
    descriptor: FieldDescriptor,
    comments: RefCell<Comments>,
    map_entry: Option<WeakMessage>,
}

impl FieldDetail {
    pub fn new(desc: FieldDescriptor, msg: Message, map_entry: Option<Message>) -> Self {
        let fqn = format!("{}.{}", msg.fully_qualified_name(), desc.name());
        Self {
            fqn,
            map_entry: map_entry.map(Into::into),
            syntax: msg.syntax(),
            descriptor: desc,
            msg: msg.clone().into(),
            comments: RefCell::new(Comments::default()),
        }
    }
    pub fn name(&self) -> &str {
        self.descriptor.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        &self.fqn
    }
    pub fn message(&self) -> Message {
        self.msg.clone().into()
    }

    /// Returns `Option::Some(Message)` if this field is a map entry.
    pub fn map_entry(&self) -> Option<Message> {
        self.map_entry.clone().map(Into::into)
    }
    /// Returns the [`Syntax`] of the `File` containing this field.
    pub fn syntax(&self) -> Syntax {
        self.syntax
    }
    pub fn descriptor(&self) -> FieldDescriptor {
        self.descriptor
    }
    pub fn is_map(&self) -> bool {
        self.map_entry.is_some()
    }
    pub fn is_repeated(&self) -> bool {
        self.descriptor.is_repeated()
    }
    pub fn is_marked_optional(&self) -> bool {
        self.descriptor.is_marked_optional(self.syntax)
    }
    pub fn is_marked_required(&self) -> bool {
        self.descriptor.is_marked_required(self.syntax)
    }
    pub fn value_type(&self) -> Type {
        self.descriptor.type_()
    }

    pub(crate) fn set_comments(&self, comments: Comments) {
        self.comments.replace(comments);
    }
    pub fn comments(&self) -> Comments {
        *self.comments.borrow()
    }
    pub fn file(&self) -> File {
        self.msg.file()
    }
    pub fn package(&self) -> Package {
        self.file().package()
    }
    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }

    // pub fn map_value(&self) -> Result<Field, crate::Error> {
    //     self.map_entry()?
    //         .fields()
    //         .get(1)
    //         .ok_or_else(|| anyhow!("value_type field not found in map entry"))
    // }
}
