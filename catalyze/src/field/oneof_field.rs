#![allow(clippy::new_ret_no_self)]

use core::panic;
use std::{cell::RefCell, rc::Rc};

use crate::{
    comments::Comments,
    enum_::{Enum, WeakEnum},
    error::Error,
    field::{Field, JsType, Scalar, Type},
    file::{File, FileRefs, Syntax},
    message::Message,
    message::WeakMessage,
    node::{Kind, Node},
    oneof::{Oneof, WeakOneof},
    package::Package,
    uninterpreted_option::UninterpretedOption,
    well_known::WellKnownType,
};

use super::FieldDetail;
#[derive(Debug, Clone)]
pub(crate) struct OneofFieldDetail {
    pub detail: FieldDetail,
    pub oneof: WeakOneof,
}
impl OneofFieldDetail {
    pub fn name(&self) -> &str {
        self.detail.name()
    }

    pub fn fully_qualified_name(&self) -> &str {
        self.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message {
        self.detail.message()
    }
    pub fn comments(&self) -> Comments {
        self.detail.comments()
    }
    pub fn syntax(&self) -> Syntax {
        self.detail.syntax()
    }
    pub fn file(&self) -> File {
        self.detail.file()
    }
    pub fn package(&self) -> Package {
        self.detail.package()
    }
    pub fn oneof(&self) -> Oneof {
        self.oneof.clone().into()
    }
    pub(crate) fn set_comments(&self, comments: Comments) {
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
pub enum OneofField {
    Scalar(OneofScalarField),
    Enum(OneofEnumField),
    Embed(OneofEmbedField),
}

impl OneofField {
    pub fn name(&self) -> &str {
        match self {
            OneofField::Scalar(f) => f.name(),
            OneofField::Enum(f) => f.name(),
            OneofField::Embed(f) => f.name(),
        }
    }
    pub fn fully_qualified_name(&self) -> &str {
        match self {
            OneofField::Scalar(f) => f.fully_qualified_name(),
            OneofField::Enum(f) => f.fully_qualified_name(),
            OneofField::Embed(f) => f.fully_qualified_name(),
        }
    }
    pub fn message(&self) -> Message {
        match self {
            OneofField::Scalar(f) => f.message(),
            OneofField::Enum(f) => f.message(),
            OneofField::Embed(f) => f.message(),
        }
    }
    pub fn comments(&self) -> Comments {
        match self {
            OneofField::Scalar(f) => f.comments(),
            OneofField::Enum(f) => f.comments(),
            OneofField::Embed(f) => f.comments(),
        }
    }

    pub fn file(&self) -> File {
        match self {
            OneofField::Scalar(f) => f.file(),
            OneofField::Enum(f) => f.file(),
            OneofField::Embed(f) => f.file(),
        }
    }
    pub fn package(&self) -> Package {
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
    pub fn imports(&self) -> FileRefs {
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
    pub fn enum_(&self) -> Option<Enum> {
        match self {
            OneofField::Enum(f) => Some(f.enum_()),
            _ => None,
        }
    }
    pub fn embed(&self) -> Option<Message> {
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
    pub(crate) fn set_comments(&self, comments: Comments) {
        match self {
            OneofField::Scalar(f) => f.set_comments(comments),
            OneofField::Enum(f) => f.set_comments(comments),
            OneofField::Embed(f) => f.set_comments(comments),
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

    pub(crate) fn set_value(&self, value: Node) -> Result<(), Error> {
        match self {
            OneofField::Enum(f) => f.set_value(value),
            OneofField::Embed(f) => f.set_value(value),
            _ => panic!("set_value called on non-enum field"),
        }
    }

    pub fn value_type(&self) -> Type {
        self.descriptor().type_()
    }

    pub(crate) fn new(detail: FieldDetail, oneof: Oneof) -> Result<Field, Error> {
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
                    enum_: RefCell::new(WeakEnum::empty()),
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
            Type::Group => Err(Error::group_not_supported(oneof.fully_qualified_name())),
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
    pub fn uninterpreted_options(&self) -> &[UninterpretedOption] {
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
pub(crate) struct OneofEnumFieldDetail {
    detail: OneofFieldDetail,
    enum_: RefCell<WeakEnum>,
}
#[derive(Debug, Clone)]
pub struct OneofEnumField(Rc<OneofEnumFieldDetail>);

impl OneofEnumField {
    pub fn name(&self) -> &str {
        self.0.detail.name()
    }

    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message {
        self.0.detail.message()
    }
    pub fn comments(&self) -> Comments {
        self.0.detail.comments()
    }
    pub fn enum_(&self) -> Enum {
        self.0.enum_.borrow().clone().into()
    }
    pub fn file(&self) -> File {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package {
        self.0.detail.package()
    }

    pub(crate) fn set_comments(&self, comments: Comments) {
        self.0.detail.set_comments(comments);
    }
    pub fn imports(&self) -> FileRefs {
        if self.has_import() {
            FileRefs::from(self.0.enum_.borrow().weak_file())
        } else {
            FileRefs::empty()
        }
    }

    pub fn has_import(&self) -> bool {
        self.enum_().file() != self.file()
    }

    pub fn build_target(&self) -> bool {
        self.file().build_target()
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
        self.enum_().is_well_known_type()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.enum_().well_known_type()
    }

    pub fn has_presence(&self) -> bool {
        true
    }

    fn set_value(&self, node: Node) -> Result<(), Error> {
        match node {
            Node::Enum(v) => {
                self.0.enum_.replace(v.into());
                Ok(())
            }
            _ => Err(Error::InvalidNode {
                expected: Kind::Enum,
                node,
            }),
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
pub struct OneofScalarFieldDetail {
    scalar: Scalar,
    detail: OneofFieldDetail,
}

#[derive(Debug, Clone)]
pub struct OneofScalarField(Rc<OneofScalarFieldDetail>);

impl OneofScalarField {
    pub fn name(&self) -> &str {
        self.0.detail.name()
    }
    pub fn oneof(&self) -> Oneof {
        self.0.detail.oneof()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }

    pub fn message(&self) -> Message {
        self.0.detail.message()
    }

    pub fn comments(&self) -> Comments {
        self.0.detail.comments()
    }

    pub fn file(&self) -> File {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package {
        self.0.detail.package()
    }
    pub fn scalar(&self) -> Scalar {
        self.0.scalar
    }
    pub(crate) fn set_comments(&self, comments: Comments) {
        self.0.detail.set_comments(comments);
    }

    pub fn build_target(&self) -> bool {
        self.file().build_target()
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
pub(crate) struct OneofEmbedFieldDetail {
    detail: OneofFieldDetail,
    embed: RefCell<WeakMessage>,
}

#[derive(Debug, Clone)]
pub struct OneofEmbedField(Rc<OneofEmbedFieldDetail>);
impl OneofEmbedField {
    pub fn name(&self) -> &str {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message {
        self.0.detail.message()
    }

    pub fn comments(&self) -> Comments {
        self.0.detail.comments()
    }
    pub fn file(&self) -> File {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package {
        self.0.detail.package()
    }

    pub fn embed(&self) -> Message {
        self.0.embed.borrow().clone().into()
    }
    pub fn has_import(&self) -> bool {
        self.0.embed.borrow().file() != self.file()
    }
    pub fn imports(&self) -> FileRefs {
        if self.has_import() {
            FileRefs::from(self.0.embed.borrow().weak_file())
        } else {
            FileRefs::empty()
        }
    }
    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }

    pub(crate) fn set_comments(&self, comments: Comments) {
        self.0.detail.set_comments(comments);
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

    fn set_value(&self, node: Node) -> Result<(), Error> {
        match node {
            Node::Message(v) => {
                self.0.embed.replace(v.into());
                Ok(())
            }
            _ => Err(Error::InvalidNode {
                expected: Kind::Message,
                node,
            }),
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
