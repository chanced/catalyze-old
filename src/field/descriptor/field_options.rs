use crate::proto::descriptor::UninterpretedOption;

use std::{cell::RefCell, marker::PhantomData, rc::Rc, slice};

use super::{CType, JsType};

#[derive(Debug)]
pub struct FieldOptions<'a, U> {
    opts: &'a prost_types::FieldOptions,
    uninterpreted_option: Vec<UninterpretedOption<'a, U>>,
    u: PhantomData<U>,
}

impl<'a, U> FieldOptions<'a, U> {
    pub fn new(opts: &'a prost_types::FieldOptions) -> Self {
        Self {
            opts,
            u: PhantomData,
            uninterpreted_option: opts
                .uninterpreted_option
                .iter()
                .map(UninterpretedOption::from)
                .collect(),
        }
    }
    /// The ctype option instructs the C++ code generator to use a different
    /// representation of the field than it normally would.  See the specific
    /// options below.  This option is not yet implemented in the open source
    /// release -- sorry, we'll try to include it in a future version!
    pub fn c_type(&self) -> CType {
        CType::from(self.opts.ctype())
    }
    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub fn packed(&self) -> bool {
        self.opts.packed()
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
    pub fn js_type(&self) -> JsType {
        JsType::from(self.opts.jstype())
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
        self.opts.lazy()
    }
    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub fn is_deprecated(&self) -> bool {
        self.opts.deprecated()
    }
    /// For Google-internal migration only. Do not use.
    pub fn is_weak(&self) -> bool {
        self.opts.weak()
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> slice::Iter<UninterpretedOption<'a, U>> {
        self.uninterpreted_option.iter()
    }
}

impl<'a, U> Clone for FieldOptions<'a, U> {
    fn clone(&self) -> Self {
        Self {
            opts: self.opts,
            u: PhantomData,
            uninterpreted_option: self.uninterpreted_option.clone(),
        }
    }
}
