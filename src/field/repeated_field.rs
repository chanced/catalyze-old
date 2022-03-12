#![allow(clippy::new_ret_no_self)]
use std::{cell::RefCell, rc::Rc};

use anyhow::bail;

use crate::{
    proto::{FieldDescriptor, Scalar, Syntax},
    Comments, EmbedFieldDetail, Enum, EnumFieldDetail, Field, FieldDetail, File, Files,
    FullyQualified, Message, Name, Node, Package, ScalarFieldDetail, Type, UninterpretedOptions,
    WeakEnum, WeakMessage, WellKnownEnum, WellKnownMessage, WellKnownType,
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
    pub fn comments(&self) -> Comments<'a> {
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
    pub fn imports(&self) -> Files<'a, U> {
        match self {
            RepeatedField::Enum(f) => f.imports(),
            RepeatedField::Embed(f) => f.imports(),
            RepeatedField::Scalar(_) => Files::empty(),
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

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        match self {
            RepeatedField::Scalar(f) => f.set_comments(comments),
            RepeatedField::Enum(f) => f.set_comments(comments),
            RepeatedField::Embed(f) => f.set_comments(comments),
        }
    }

    pub fn is_marked_required(&self) -> bool {
        match self {
            RepeatedField::Scalar(f) => f.is_marked_required(),
            RepeatedField::Enum(f) => f.is_marked_required(),
            RepeatedField::Embed(f) => f.is_marked_required(),
        }
    }

    pub fn is_embed(&self) -> bool {
        matches!(self, RepeatedField::Embed(_))
    }

    pub(crate) fn is_well_known_type(&self) -> bool {
        match self {
            RepeatedField::Enum(f) => f.is_well_known_type(),
            RepeatedField::Embed(f) => f.is_well_known_type(),
            RepeatedField::Scalar(_f) => false,
        }
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        match self {
            RepeatedField::Enum(f) => f.well_known_type(),
            RepeatedField::Embed(f) => f.well_known_type(),
            RepeatedField::Scalar(_) => None,
        }
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, RepeatedField::Enum(_))
    }

    pub(crate) fn set_value(&self, value: Node<'a, U>) -> Result<(), anyhow::Error> {
        match self {
            RepeatedField::Enum(f) => f.set_value(value),
            RepeatedField::Embed(f) => f.set_value(value),
            _ => unreachable!(),
        }
    }

    pub fn value_type(&self) -> Type<'a> {
        self.descriptor().proto_type()
    }

    pub(crate) fn new(detail: FieldDetail<'a, U>) -> Result<Field<'a, U>, anyhow::Error> {
        match detail.value_type() {
            Type::Scalar(s) => Ok(Field::Repeated(RepeatedField::Scalar(RepeatedScalarField(
                Rc::new(ScalarFieldDetail::new(detail, s)),
            )))),
            Type::Enum(_) => Ok(Field::Repeated(RepeatedField::Enum(RepeatedEnumField(
                Rc::new(EnumFieldDetail {
                    detail,
                    enumeration: RefCell::new(WeakEnum::empty()),
                }),
            )))),
            Type::Message(_) => Ok(Field::Repeated(RepeatedField::Embed(RepeatedEmbedField(
                Rc::new(EmbedFieldDetail {
                    detail,
                    embed: RefCell::new(WeakMessage::empty()),
                }),
            )))),
            Type::Group => bail!("Group is not supported. Use an embedded message instead."),
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
    pub fn jstype(&self) -> crate::JsType {
        match self {
            RepeatedField::Scalar(f) => f.jstype(),
            RepeatedField::Enum(f) => f.jstype(),
            RepeatedField::Embed(f) => f.jstype(),
        }
    }

    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub fn is_packed(&self) -> bool {
        match self {
            RepeatedField::Scalar(f) => f.is_packed(),
            RepeatedField::Enum(f) => f.is_packed(),
            RepeatedField::Embed(f) => f.is_packed(),
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
            RepeatedField::Scalar(f) => f.is_lazy(),
            RepeatedField::Enum(f) => f.is_lazy(),
            RepeatedField::Embed(f) => f.is_lazy(),
        }
    }

    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub fn is_deprecated(&self) -> bool {
        match self {
            RepeatedField::Scalar(f) => f.is_deprecated(),
            RepeatedField::Enum(f) => f.is_deprecated(),
            RepeatedField::Embed(f) => f.is_deprecated(),
        }
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        match self {
            RepeatedField::Scalar(f) => f.uninterpreted_options(),
            RepeatedField::Enum(f) => f.uninterpreted_options(),
            RepeatedField::Embed(f) => f.uninterpreted_options(),
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

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }

    pub fn has_import(&self) -> bool {
        self.file() != self.0.embed().file()
    }
    pub fn imports(&self) -> Files<'a, U> {
        if self.has_import() {
            Files::from(self.0.embed().weak_file())
        } else {
            Files::empty()
        }
    }

    pub fn embed(&self) -> Message<'a, U> {
        self.0.embed()
    }

    pub fn build_target(&self) -> bool {
        self.0.detail.build_target()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.0.embed().is_well_known_type()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.0.embed().well_known_type()
    }
    pub fn well_known_message(&self) -> Option<WellKnownMessage> {
        self.0.embed().well_known_message()
    }

    fn set_value(&self, value: Node<'a, U>) -> Result<(), anyhow::Error> {
        self.0.set_value(value)
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
    pub fn jstype(&self) -> crate::JsType {
        self.descriptor().options().jstype()
    }

    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub fn is_packed(&self) -> bool {
        self.descriptor().options().packed()
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
        self.descriptor().options().is_lazy()
    }

    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub fn is_deprecated(&self) -> bool {
        self.descriptor().options().is_deprecated()
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        self.descriptor().options().uninterpreted_options()
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

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }
    pub fn is_marked_optional(&self) -> bool {
        self.0.detail.is_marked_optional()
    }
    pub fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }

    pub fn has_import(&self) -> bool {
        self.0.has_import()
    }
    pub fn imports(&self) -> Files<'a, U> {
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

    pub fn is_well_known_type(&self) -> bool {
        self.0.enumeration().is_well_known_type()
    }
    pub fn well_known_enum(&self) -> Option<WellKnownEnum> {
        self.0.enumeration().well_known_enum()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.0.enumeration().well_known_type()
    }

    fn set_value(&self, value: Node<'a, U>) -> Result<(), anyhow::Error> {
        self.0.set_value(value)
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
    pub fn jstype(&self) -> crate::JsType {
        self.descriptor().options().jstype()
    }

    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub fn is_packed(&self) -> bool {
        self.descriptor().options().packed()
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
        self.descriptor().options().is_lazy()
    }

    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub fn is_deprecated(&self) -> bool {
        self.descriptor().options().is_deprecated()
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        self.descriptor().options().uninterpreted_options()
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

#[derive(Debug)]
pub struct RepeatedScalarField<'a, U>(Rc<ScalarFieldDetail<'a, U>>);

impl<'a, U> RepeatedScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.fully_qualified_name()
    }

    pub fn file(&self) -> File<'a, U> {
        self.0.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.package()
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

    pub fn syntax(&self) -> Syntax {
        self.0.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.descriptor()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.set_comments(comments);
    }

    pub fn scalar(&self) -> Scalar {
        self.0.scalar()
    }

    pub fn build_target(&self) -> bool {
        self.0.build_target()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.is_marked_required()
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
    pub fn jstype(&self) -> crate::JsType {
        self.descriptor().options().jstype()
    }

    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub fn is_packed(&self) -> bool {
        self.descriptor().options().packed()
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
        self.descriptor().options().is_lazy()
    }

    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub fn is_deprecated(&self) -> bool {
        self.descriptor().options().is_deprecated()
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        self.descriptor().options().uninterpreted_options()
    }
}

impl<'a, U> FullyQualified for RepeatedScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fully_qualified_name()
    }
}

impl<'a, U> Clone for RepeatedScalarField<'a, U> {
    fn clone(&self) -> Self {
        RepeatedScalarField(self.0.clone())
    }
}
