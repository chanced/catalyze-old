#![allow(clippy::new_ret_no_self)]

use core::panic;
use std::{cell::RefCell, rc::Rc};

use anyhow::bail;

use crate::{
    proto::{FieldDescriptor, Scalar, Syntax},
    Comments, Enum, Field, File, FileRefs, JsType, Message, Name, Node, Oneof, Package, Type,
    UninterpretedOptions, WeakEnum, WeakMessage, WeakOneof, WellKnownType,
};

use super::FieldDetail;
#[derive(Debug, Clone)]
pub(crate) struct OneofFieldDetail<'a> {
    pub detail: FieldDetail<'a>,
    pub oneof: WeakOneof<'a>,
}
impl<'a> OneofFieldDetail<'a> {
    pub fn name(&self) -> &Name {
        self.detail.name()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.detail.descriptor()
    }

    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message<'a> {
        self.detail.message()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.detail.comments()
    }
    pub fn syntax(&self) -> Syntax {
        self.detail.syntax()
    }
    pub fn file(&self) -> File<'a> {
        self.detail.file()
    }
    pub fn package(&self) -> Package<'a> {
        self.detail.package()
    }
    pub fn oneof(&self) -> Oneof<'a> {
        self.oneof.clone().into()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.detail.set_comments(comments);
    }

    pub fn is_marked_required(&self) -> bool {
        self.detail.is_marked_required()
    }

    pub fn is_in_real_oneof(&self) -> bool {
        self.oneof().is_real()
    }
    pub fn is_in_synthetic_oneof(&self) -> bool {
        self.oneof().is_synthetic()
    }
}

#[derive(Debug, Clone)]
pub enum OneofField<'a> {
    Scalar(OneofScalarField<'a>),
    Enum(OneofEnumField<'a>),
    Embed(OneofEmbedField<'a>),
}

impl<'a> OneofField<'a> {
    pub fn name(&self) -> &Name {
        match self {
            OneofField::Scalar(f) => f.name(),
            OneofField::Enum(f) => f.name(),
            OneofField::Embed(f) => f.name(),
        }
    }
    pub fn fully_qualified_name(&self) -> String {
        match self {
            OneofField::Scalar(f) => f.fully_qualified_name(),
            OneofField::Enum(f) => f.fully_qualified_name(),
            OneofField::Embed(f) => f.fully_qualified_name(),
        }
    }
    pub fn message(&self) -> Message<'a> {
        match self {
            OneofField::Scalar(f) => f.message(),
            OneofField::Enum(f) => f.message(),
            OneofField::Embed(f) => f.message(),
        }
    }
    pub fn comments(&self) -> Comments<'a> {
        match self {
            OneofField::Scalar(f) => f.comments(),
            OneofField::Enum(f) => f.comments(),
            OneofField::Embed(f) => f.comments(),
        }
    }

    pub fn file(&self) -> File<'a> {
        match self {
            OneofField::Scalar(f) => f.file(),
            OneofField::Enum(f) => f.file(),
            OneofField::Embed(f) => f.file(),
        }
    }
    pub fn package(&self) -> Package<'a> {
        match self {
            OneofField::Scalar(f) => f.package(),
            OneofField::Enum(f) => f.package(),
            OneofField::Embed(f) => f.package(),
        }
    }

    pub fn has_import(&self) -> bool {
        match self {
            OneofField::Scalar(_) => false,
            OneofField::Enum(f) => f.has_import(),
            OneofField::Embed(f) => f.has_import(),
        }
    }
    pub fn imports(&self) -> FileRefs<'a> {
        match self {
            OneofField::Scalar(_) => FileRefs::empty(),
            OneofField::Enum(f) => f.imports(),
            OneofField::Embed(f) => f.imports(),
        }
    }

    pub fn build_target(&self) -> bool {
        match self {
            OneofField::Scalar(f) => f.build_target(),
            OneofField::Enum(f) => f.build_target(),
            OneofField::Embed(f) => f.build_target(),
        }
    }
    pub fn r#enum(&self) -> Option<Enum<'a>> {
        match self {
            OneofField::Enum(f) => Some(f.r#enum()),
            _ => None,
        }
    }
    pub fn enumeration(&self) -> Option<Enum<'a>> {
        self.r#enum()
    }
    pub fn embed(&self) -> Option<Message<'a>> {
        match self {
            OneofField::Embed(f) => Some(f.embed()),
            _ => None,
        }
    }
    pub fn is_scalar(&self) -> bool {
        matches!(self, OneofField::Scalar(_))
    }
    pub fn scalar(&self) -> Option<Scalar> {
        match self {
            OneofField::Scalar(f) => Some(f.scalar()),
            _ => None,
        }
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        match self {
            OneofField::Scalar(f) => f.set_comments(comments),
            OneofField::Enum(f) => f.set_comments(comments),
            OneofField::Embed(f) => f.set_comments(comments),
        }
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        match self {
            OneofField::Scalar(f) => f.descriptor(),
            OneofField::Enum(f) => f.descriptor(),
            OneofField::Embed(f) => f.descriptor(),
        }
    }

    pub fn syntax(&self) -> Syntax {
        match self {
            OneofField::Scalar(f) => f.syntax(),
            OneofField::Enum(f) => f.syntax(),
            OneofField::Embed(f) => f.syntax(),
        }
    }

    pub fn is_marked_required(&self) -> bool {
        match self {
            OneofField::Scalar(f) => f.is_marked_required(),
            OneofField::Enum(f) => f.is_marked_required(),
            OneofField::Embed(f) => f.is_marked_required(),
        }
    }

    pub fn is_embed(&self) -> bool {
        matches!(self, OneofField::Embed(_))
    }

    pub fn is_in_real_oneof(&self) -> bool {
        match self {
            OneofField::Scalar(f) => f.is_in_real_oneof(),
            OneofField::Enum(f) => f.is_in_real_oneof(),
            OneofField::Embed(f) => f.is_in_real_oneof(),
        }
    }

    pub fn is_in_synthetic_oneof(&self) -> bool {
        match self {
            OneofField::Scalar(f) => f.is_in_synthetic_oneof(),
            OneofField::Enum(f) => f.is_in_synthetic_oneof(),
            OneofField::Embed(f) => f.is_in_synthetic_oneof(),
        }
    }

    pub fn is_well_known_type(&self) -> bool {
        match self {
            OneofField::Enum(f) => f.is_well_known_type(),
            OneofField::Embed(f) => f.is_well_known_type(),
            OneofField::Scalar(_) => false,
        }
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        match self {
            OneofField::Enum(f) => f.well_known_type(),
            OneofField::Embed(f) => f.well_known_type(),
            OneofField::Scalar(_) => None,
        }
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, OneofField::Enum(_))
    }

    pub fn has_presence(&self) -> bool {
        match self {
            OneofField::Scalar(f) => f.has_presence(),
            OneofField::Enum(f) => f.has_presence(),
            OneofField::Embed(f) => f.has_presence(),
        }
    }

    pub(crate) fn set_value(&self, value: crate::Node<'a>) -> Result<(), anyhow::Error> {
        match self {
            OneofField::Enum(f) => f.set_value(value),
            OneofField::Embed(f) => f.set_value(value),
            _ => panic!("set_value called on non-enum field"),
        }
    }

    pub fn value_type(&self) -> Type<'a> {
        self.descriptor().proto_type()
    }

    pub(crate) fn new(
        detail: FieldDetail<'a>,
        oneof: Oneof<'a>,
    ) -> Result<crate::Field<'a>, anyhow::Error> {
        match detail.value_type() {
            Type::Scalar(scalar) => Ok(Field::Oneof(OneofField::Scalar(OneofScalarField(
                Rc::new(OneofScalarFieldDetail {
                    scalar,
                    detail: OneofFieldDetail {
                        detail,
                        oneof: oneof.into(),
                    },
                }),
            )))),
            Type::Enum(_) => Ok(Field::Oneof(OneofField::Enum(OneofEnumField(Rc::new(
                OneofEnumFieldDetail {
                    detail: OneofFieldDetail {
                        detail,
                        oneof: oneof.into(),
                    },
                    enumeration: RefCell::new(WeakEnum::empty()),
                },
            ))))),
            Type::Message(_) => Ok(Field::Oneof(OneofField::Embed(OneofEmbedField(Rc::new(
                OneofEmbedFieldDetail {
                    detail: OneofFieldDetail {
                        detail,
                        oneof: oneof.into(),
                    },
                    embed: RefCell::new(WeakMessage::new()),
                },
            ))))),
            Type::Group => bail!("Group is not supported. Use an embedded Message instead."),
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
            OneofField::Scalar(f) => f.jstype(),
            OneofField::Enum(f) => f.jstype(),
            OneofField::Embed(f) => f.jstype(),
        }
    }
    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub fn is_packed(&self) -> bool {
        match self {
            OneofField::Scalar(f) => f.is_packed(),
            OneofField::Enum(f) => f.is_packed(),
            OneofField::Embed(f) => f.is_packed(),
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
            OneofField::Scalar(f) => f.is_lazy(),
            OneofField::Enum(f) => f.is_lazy(),
            OneofField::Embed(f) => f.is_lazy(),
        }
    }

    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub fn is_deprecated(&self) -> bool {
        match self {
            OneofField::Scalar(f) => f.is_deprecated(),
            OneofField::Enum(f) => f.is_deprecated(),
            OneofField::Embed(f) => f.is_deprecated(),
        }
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        self.descriptor().options().uninterpreted_options()
    }

    pub fn number(&self) -> i32 {
        match self {
            OneofField::Scalar(f) => f.number(),
            OneofField::Enum(f) => f.number(),
            OneofField::Embed(f) => f.number(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OneofEnumFieldDetail<'a> {
    detail: OneofFieldDetail<'a>,
    enumeration: RefCell<WeakEnum<'a>>,
}
#[derive(Debug, Clone)]
pub struct OneofEnumField<'a>(Rc<OneofEnumFieldDetail<'a>>);

impl<'a> OneofEnumField<'a> {
    pub fn name(&self) -> &Name {
        self.0.detail.name()
    }

    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message<'a> {
        self.0.detail.message()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }
    pub fn r#enum(&self) -> Enum<'a> {
        self.0.enumeration.borrow().clone().into()
    }
    pub fn enumeration(&self) -> Enum<'a> {
        self.r#enum()
    }
    pub fn file(&self) -> File<'a> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a> {
        self.0.detail.package()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }
    pub fn imports(&self) -> FileRefs<'a> {
        if self.has_import() {
            FileRefs::from(self.0.enumeration.borrow().weak_file())
        } else {
            FileRefs::empty()
        }
    }

    pub fn has_import(&self) -> bool {
        self.enumeration().file() != self.file()
    }

    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }

    pub fn is_in_real_oneof(&self) -> bool {
        self.0.detail.is_in_real_oneof()
    }

    pub fn is_in_synthetic_oneof(&self) -> bool {
        self.0.detail.is_in_synthetic_oneof()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.enumeration().is_well_known_type()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.r#enum().well_known_type()
    }

    pub fn has_presence(&self) -> bool {
        true
    }

    fn set_value(&self, value: crate::Node<'a>) -> Result<(), anyhow::Error> {
        match value {
            Node::Enum(v) => {
                self.0.enumeration.replace(v.into());
                Ok(())
            }
            _ => bail!("expected Enum, received {}", value),
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

    pub fn number(&self) -> i32 {
        self.descriptor().number()
    }
}

#[derive(Debug, Clone)]
pub struct OneofScalarFieldDetail<'a> {
    scalar: Scalar,
    detail: OneofFieldDetail<'a>,
}

#[derive(Debug, Clone)]
pub struct OneofScalarField<'a>(Rc<OneofScalarFieldDetail<'a>>);

impl<'a> OneofScalarField<'a> {
    pub fn name(&self) -> &Name {
        self.0.detail.name()
    }
    pub fn oneof(&self) -> Oneof<'a> {
        self.0.detail.oneof()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }

    pub fn message(&self) -> Message<'a> {
        self.0.detail.message()
    }

    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }

    pub fn file(&self) -> File<'a> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a> {
        self.0.detail.package()
    }
    pub fn scalar(&self) -> Scalar {
        self.0.scalar
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }

    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }

    pub fn is_in_real_oneof(&self) -> bool {
        self.0.detail.is_in_real_oneof()
    }

    pub fn is_in_synthetic_oneof(&self) -> bool {
        self.0.detail.is_in_synthetic_oneof()
    }

    pub fn has_presence(&self) -> bool {
        true
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

    pub fn number(&self) -> i32 {
        self.descriptor().number()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OneofEmbedFieldDetail<'a> {
    detail: OneofFieldDetail<'a>,
    embed: RefCell<WeakMessage<'a>>,
}

#[derive(Debug, Clone)]
pub struct OneofEmbedField<'a>(Rc<OneofEmbedFieldDetail<'a>>);
impl<'a> OneofEmbedField<'a> {
    pub fn name(&self) -> &Name {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message<'a> {
        self.0.detail.message()
    }

    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }
    pub fn file(&self) -> File<'a> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a> {
        self.0.detail.package()
    }

    pub fn embed(&self) -> Message<'a> {
        self.0.embed.borrow().clone().into()
    }
    pub fn has_import(&self) -> bool {
        self.0.embed.borrow().file() != self.file()
    }
    pub fn imports(&self) -> FileRefs<'a> {
        if self.has_import() {
            FileRefs::from(self.0.embed.borrow().weak_file())
        } else {
            FileRefs::empty()
        }
    }
    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }

    fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }

    pub fn is_in_real_oneof(&self) -> bool {
        self.0.detail.is_in_real_oneof()
    }
    pub fn is_in_synthetic_oneof(&self) -> bool {
        self.0.detail.is_in_synthetic_oneof()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.embed().is_well_known_type()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.embed().well_known_type()
    }

    pub fn has_presence(&self) -> bool {
        true
    }

    fn set_value(&self, value: Node<'a>) -> Result<(), anyhow::Error> {
        match value {
            Node::Message(v) => {
                self.0.embed.replace(v.into());
                Ok(())
            }
            _ => bail!("expected Message, received {}", value),
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

    pub fn number(&self) -> i32 {
        self.descriptor().number()
    }
}
