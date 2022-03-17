#![allow(clippy::new_ret_no_self)]

use std::{cell::RefCell, rc::Rc};

use anyhow::bail;

use crate::{
    proto::FieldDescriptor, proto::Syntax, Comments, Field, File, FileRefs, JsType, Message, Name,
    Node, Package, Type, UninterpretedOptions, WeakMessage, WellKnownMessage, WellKnownType,
};

use super::FieldDetail;

#[derive(Debug)]
pub(crate) struct EmbedFieldDetail<'a> {
    pub detail: FieldDetail<'a>,
    pub embed: RefCell<WeakMessage<'a>>,
}
impl<'a> EmbedFieldDetail<'a> {
    pub fn name(&self) -> Name {
        self.detail.name()
    }
    pub fn embed(&self) -> Message<'a> {
        self.embed.borrow().clone().into()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn is_repeated(&self) -> bool {
        self.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.detail.is_map()
    }
    pub fn message(&self) -> Message<'a> {
        self.detail.message()
    }
    pub fn well_known_message(&self) -> Option<WellKnownMessage> {
        self.embed().well_known_message()
    }
    pub fn syntax(&self) -> Syntax {
        self.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.detail.descriptor()
    }

    pub fn imports(&self) -> FileRefs<'a> {
        if self.embed.borrow().file() != self.detail.file() {
            FileRefs::from(self.embed.borrow().weak_file())
        } else {
            FileRefs::empty()
        }
    }
    pub fn build_target(&self) -> bool {
        self.detail.build_target()
    }
    pub fn is_well_known_type(&self) -> bool {
        self.embed.borrow().is_well_known_type()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.embed().well_known_type()
    }

    pub fn has_import(&self) -> bool {
        self.embed.borrow().file() != self.detail.file()
    }
    pub(crate) fn set_value(&self, value: Node<'a>) -> Result<(), anyhow::Error> {
        match value {
            Node::Message(m) => {
                self.embed.replace(m.into());
                Ok(())
            }
            _ => bail!("expected Message, received {}", value),
        }
    }
}

impl<'a> Clone for EmbedFieldDetail<'a> {
    fn clone(&self) -> Self {
        Self {
            detail: self.detail.clone(),
            embed: self.embed.clone(),
        }
    }
}

#[derive(Debug)]
pub struct EmbedField<'a>(Rc<EmbedFieldDetail<'a>>);

impl<'a> EmbedField<'a> {
    pub fn name(&self) -> Name {
        self.0.name()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }

    pub fn embed(&self) -> Message<'a> {
        self.0.embed()
    }
    pub fn build_target(&self) -> bool {
        self.0.build_target()
    }
    pub fn file(&self) -> File<'a> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a> {
        self.0.detail.package()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fqn.clone()
    }

    /// Indicates whether or not the field is labeled as a required field. This
    /// will only be `true` if the syntax is proto2.
    pub fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }
    pub fn is_marked_optional(&self) -> bool {
        self.0.detail.is_marked_optional()
    }

    pub fn has_presence(&self) -> bool {
        true
    }

    pub fn has_import(&self) -> bool {
        self.0.has_import()
    }

    pub fn imports(&self) -> FileRefs<'a> {
        self.0.imports()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.fully_qualified_name()
    }
    pub fn is_repeated(&self) -> bool {
        self.0.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.0.is_map()
    }
    pub fn message(&self) -> Message<'a> {
        self.0.message()
    }
    pub fn well_known_message(&self) -> Option<WellKnownMessage> {
        self.0.well_known_message()
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.descriptor()
    }

    pub fn syntax(&self) -> Syntax {
        self.0.syntax()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.0.is_well_known_type()
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.0.well_known_type()
    }

    pub(crate) fn set_value(&self, value: Node<'a>) -> Result<(), anyhow::Error> {
        self.0.set_value(value)
    }

    pub fn value_type(&self) -> Type<'a> {
        self.0.descriptor().proto_type()
    }

    pub(crate) fn new(detail: FieldDetail<'a>) -> Result<Field<'a>, anyhow::Error> {
        let detail = Rc::new(EmbedFieldDetail {
            detail,
            embed: RefCell::new(WeakMessage::empty()),
        });
        Ok(Field::Embed(EmbedField(detail)))
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
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        self.descriptor().options().uninterpreted_options()
    }
}
impl<'a> Clone for EmbedField<'a> {
    fn clone(&self) -> Self {
        EmbedField(self.0.clone())
    }
}
