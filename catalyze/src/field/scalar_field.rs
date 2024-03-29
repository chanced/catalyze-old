#![allow(clippy::new_ret_no_self)]
use std::rc::Rc;

use super::FieldDetail;
use crate::{
    comments::Comments,
    field::{Field, JsType, Scalar, Syntax, Type},
    file::File,
    message::Message,
    package::Package,
    uninterpreted_option::UninterpretedOption,
};

#[derive(Debug, Clone)]
pub(crate) struct Detail {
    detail: FieldDetail,
    scalar: Scalar,
}

impl Detail {
    pub(crate) fn new(detail: FieldDetail, scalar: Scalar) -> Self {
        Self { detail, scalar }
    }

    pub fn name(&self) -> &str {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.detail.fully_qualified_name()
    }
    pub fn build_target(&self) -> bool {
        self.detail.build_target()
    }
    pub fn scalar(&self) -> Scalar {
        self.scalar
    }
    pub fn is_repeated(&self) -> bool {
        self.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.detail.is_map()
    }
    pub fn message(&self) -> Message {
        self.detail.message()
    }
    /// Returns `Rc<U>`

    pub fn syntax(&self) -> Syntax {
        self.detail.syntax()
    }

    pub fn comments(&self) -> Comments {
        self.detail.comments()
    }
    pub fn package(&self) -> Package {
        self.detail.package()
    }
    pub fn file(&self) -> File {
        self.detail.file()
    }

    pub(crate) fn set_comments(&self, comments: Comments) {
        self.detail.set_comments(comments);
    }

    pub fn is_marked_required(&self) -> bool {
        self.detail.is_marked_required()
    }
    pub fn is_marked_optional(&self) -> bool {
        self.detail.is_marked_optional()
    }
}
#[derive(Debug, Clone)]
pub struct ScalarField(Rc<Detail>);

impl ScalarField {
    /// # Panics
    /// Panics if the field's value type is not a scalar.
    pub(crate) fn new(detail: FieldDetail) -> Field {
        match detail.value_type() {
            Type::Scalar(s) => Field::Scalar(Self(Rc::new(Detail { detail, scalar: s }))),
            invalid_type => panic!(
                "ScalarField::new called with non-scalar type: {:?}",
                invalid_type
            ),
        }
    }

    pub fn scalar(&self) -> Scalar {
        self.0.scalar
    }
    pub fn name(&self) -> &str {
        self.0.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.0.fully_qualified_name()
    }
    pub fn comments(&self) -> Comments {
        self.0.comments()
    }
    pub fn file(&self) -> File {
        self.0.file()
    }
    pub fn package(&self) -> Package {
        self.0.package()
    }
    pub fn syntax(&self) -> Syntax {
        self.0.syntax()
    }
    pub(crate) fn set_comments(&self, comments: Comments) {
        self.0.set_comments(comments);
    }

    pub fn build_target(&self) -> bool {
        self.0.build_target()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.is_marked_required()
    }

    pub fn is_marked_optional(&self) -> bool {
        self.0.is_marked_optional()
    }

    pub fn has_presence(&self) -> bool {
        self.syntax().is_proto2() || self.is_marked_optional()
    }

    pub fn value_type(&self) -> Type {
        self.descriptor().type_()
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
        self.descriptor().options().deprecated()
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> &[UninterpretedOption] {
        self.descriptor().options().uninterpreted_options()
    }

    pub fn message(&self) -> Message {
        self.0.message()
    }

    pub fn number(&self) -> i32 {
        self.descriptor().number()
    }
}

// #[derive(Debug, Clone)]
// pub(crate) struct WeakScalarField(Weak<MappedScalarFieldDetail>);
