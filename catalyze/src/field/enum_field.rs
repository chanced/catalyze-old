#![allow(clippy::new_ret_no_self)]

use std::{cell::RefCell, rc::Rc};

use crate::{
    comments::Comments,
    enum_::{Enum, WeakEnum},
    error::Error,
    field::{Field, FieldDetail, Type},
    file::{File, FileRefs, Syntax},
    message::Message,
    node::{Kind, Node},
    package::Package,
    uninterpreted_option::UninterpretedOption,
    well_known::{WellKnownEnum, WellKnownType},
};

use super::JsType;

#[derive(Debug, Clone)]
pub(crate) struct Detail {
    pub detail: FieldDetail,
    pub enum_: RefCell<WeakEnum>,
}

impl Detail {
    pub fn is_repeated(&self) -> bool {
        self.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.detail.is_map()
    }
    pub fn message(&self) -> Message {
        self.detail.message()
    }
    pub fn comments(&self) -> Comments {
        self.detail.comments()
    }

    pub fn set_comments(&self, comments: Comments) {
        self.detail.set_comments(comments)
    }
    pub fn package(&self) -> Package {
        self.detail.package()
    }
    pub fn well_known_enum(&self) -> Option<WellKnownEnum> {
        self.enum_().well_known_enum()
    }
    pub fn is_marked_required(&self) -> bool {
        self.detail.is_marked_required()
    }
    pub fn is_marked_optional(&self) -> bool {
        self.detail.is_marked_optional()
    }

    pub fn syntax(&self) -> Syntax {
        self.detail.syntax()
    }
    pub fn enum_(&self) -> Enum {
        self.enum_.borrow().clone().into()
    }
    pub fn build_target(&self) -> bool {
        self.file().build_target()
    }

    pub fn file(&self) -> File {
        self.detail.file()
    }

    pub fn imports(&self) -> FileRefs {
        let e = self.enum_();
        if self.file() != e.file() {
            FileRefs::from(e.weak_file())
        } else {
            FileRefs::empty()
        }
    }

    pub fn has_import(&self) -> bool {
        self.enum_().file() != self.detail.file()
    }
    pub fn is_well_known_type(&self) -> bool {
        self.enum_().is_well_known_type()
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.enum_().well_known_type()
    }

    pub(crate) fn set_value(&self, node: Node) -> Result<(), Error> {
        match node {
            Node::Enum(v) => {
                self.enum_.replace(v.into());
                Ok(())
            }
            _ => Err(Error::invalid_node(Kind::Enum, node)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnumField(Rc<Detail>);

impl EnumField {
    pub(crate) fn new(detail: FieldDetail) -> Field {
        if !matches!(detail.value_type(), Type::Enum(_)) {
            panic!(
                "EnumField::new called with non-enum type: {:?}",
                detail.value_type()
            );
        }
        let e = Detail {
            detail,
            enum_: RefCell::new(WeakEnum::empty()),
        };
        let f = EnumField(Rc::new(e));
        Field::Enum(f)
    }

    pub fn name(&self) -> &str {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
    pub fn well_known_enum(&self) -> Option<WellKnownEnum> {
        self.0.well_known_enum()
    }
    /// Returns the `Enum` of this `EnumField`.
    pub fn enum_(&self) -> Enum {
        self.0.enum_.borrow().clone().into()
    }

    pub fn build_target(&self) -> bool {
        self.0.build_target()
    }
    pub fn is_repeated(&self) -> bool {
        self.0.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.0.is_map()
    }
    pub fn message(&self) -> Message {
        self.0.message()
    }
    pub fn comments(&self) -> Comments {
        self.0.comments()
    }

    pub fn set_comments(&self, comments: Comments) {
        self.0.set_comments(comments)
    }
    pub fn package(&self) -> Package {
        self.0.package()
    }

    pub fn has_presence(&self) -> bool {
        self.syntax() == Syntax::Proto2 || self.descriptor().is_marked_optional(self.syntax())
    }
    pub fn syntax(&self) -> Syntax {
        self.0.syntax()
    }
    // pub fn descriptor(&self) -> FieldDescriptor {
    //     self.0.descriptor()
    // }

    pub fn file(&self) -> File {
        self.0.detail.file()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.0.is_well_known_type()
    }

    pub fn has_import(&self) -> bool {
        self.0.has_import()
    }
    pub fn imports(&self) -> FileRefs {
        self.0.imports()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.is_marked_required()
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.0.well_known_type()
    }

    pub fn is_marked_optional(&self) -> bool {
        self.0.is_marked_optional()
    }

    pub(crate) fn set_value(&self, node: Node) -> Result<(), Error> {
        self.0.set_value(node)
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
        self.descriptor().options().is_deprecated()
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> &[UninterpretedOption] {
        self.descriptor().options().uninterpreted_options()
    }

    pub fn number(&self) -> i32 {
        self.descriptor().number()
    }
}
