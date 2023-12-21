use crate::comments::Comments;
use crate::file::File;
use crate::message::{Message, WeakMessage};
use crate::node::Node;
use crate::package::Package;
use crate::service::{Service, WeakService};
use crate::uninterpreted_option::UninterpretedOption;
use protobuf::descriptor::MethodDescriptorProto as MethodDescriptor;
use protobuf::SpecialFields;
use std::fmt;
use std::{cell::RefCell, rc::Rc};

pub struct Io<'a> {
    pub input: &'a str,
    pub output: &'a str,
}

/// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
/// or neither? HTTP based RPC implementation may choose GET verb for safe
/// methods, and PUT verb for idempotent methods instead of the default POST.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum IdempotencyLevel {
    IdempotencyUnknown = 0,
    /// implies idempotent
    NoSideEffects = 1,
    /// idempotent, but may have side effects
    Idempotent = 2,

    Unknown(i32),
}
impl From<protobuf::descriptor::method_options::IdempotencyLevel> for IdempotencyLevel {
    fn from(value: protobuf::descriptor::method_options::IdempotencyLevel) -> Self {
        match value {
            protobuf::descriptor::method_options::IdempotencyLevel::IDEMPOTENT => {
                IdempotencyLevel::Idempotent
            }
            protobuf::descriptor::method_options::IdempotencyLevel::NO_SIDE_EFFECTS => {
                IdempotencyLevel::NoSideEffects
            }
            protobuf::descriptor::method_options::IdempotencyLevel::IDEMPOTENCY_UNKNOWN => {
                IdempotencyLevel::IdempotencyUnknown
            }
        }
    }
}

impl From<i32> for IdempotencyLevel {
    fn from(value: i32) -> Self {
        match value {
            0 => IdempotencyLevel::IdempotencyUnknown,
            1 => IdempotencyLevel::NoSideEffects,
            2 => IdempotencyLevel::Idempotent,
            _ => IdempotencyLevel::Unknown(value),
        }
    }
}

#[derive(Debug, Clone)]
struct Detail {
    descriptor: MethodDescriptor,
    fqn: String,
    comments: RefCell<Comments>,
    service: WeakService,
    input: RefCell<WeakMessage>,
    output: RefCell<WeakMessage>,
}

#[derive(Debug, Clone)]
pub struct Method(Rc<Detail>);

impl Method {
    pub(crate) fn new(descriptor: MethodDescriptor, svc: Service) -> Self {
        let input = Rc::new(RefCell::new(WeakMessage::new()));
        let output = Rc::new(RefCell::new(WeakMessage::new()));
        let fqn = format!("{}.{}", svc.fully_qualified_name(), descriptor.name());
        Method(Rc::new(Detail {
            descriptor,
            fqn,
            comments: RefCell::new(Comments::default()),
            service: svc.clone().into(),
            input,
            output,
        }))
    }

    pub(crate) fn set_input(&self, msg: Message) {
        self.0.input.replace(msg.clone().into());
    }
    pub(crate) fn set_output(&self, msg: Message) {
        self.0.output.replace(msg.clone().into());
    }

    pub fn name(&self) -> &str {
        &self.0.descriptor.name()
    }
    pub fn descriptor(&self) -> MethodDescriptor {
        self.0.descriptor
    }
    pub fn comments(&self) -> Comments {
        Comments::default()
    }
    pub fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
    }
    pub fn service(&self) -> Service {
        self.0.service.clone().into()
    }
    pub fn file(&self) -> File {
        self.service().file()
    }
    pub fn package(&self) -> Package {
        self.file().package()
    }

    /// Indicates if this method allows clients to stream inputs.
    pub fn is_client_streaming(&self) -> bool {
        self.descriptor().client_streaming()
    }
    /// Indicates if this method allows servers to stream outputs.
    pub fn is_server_streaming(&self) -> bool {
        self.descriptor().server_streaming()
    }
    /// Indicates if this method allows for bidirectional streaming.
    pub fn is_bidirectional_streaming(&self) -> bool {
        self.is_client_streaming() && self.is_server_streaming()
    }
    pub fn input(&self) -> Message {
        self.0.input.borrow().clone().into()
    }
    pub fn output(&self) -> Message {
        self.0.output.borrow().clone().into()
    }
    /// alias for `input`
    pub fn request(&self) -> Message {
        self.input()
    }
    /// alias for `output`
    pub fn response(&self) -> Message {
        self.output()
    }

    pub(crate) fn input_type(&self) -> &str {
        self.0.descriptor.input_type()
    }
    pub(crate) fn output_type(&self) -> &str {
        self.0.descriptor.output_type()
    }
    pub(crate) fn set_comments(&self, comments: Comments) {
        self.0.comments.replace(comments);
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node> {
        if path.is_empty() {
            Some(self.into())
        } else {
            None
        }
    }

    pub(crate) fn io(&self) -> Io {
        Io {
            input: self.input_type(),
            output: self.output_type(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum MethodIo {
    Input(String),
    Output(String),
}

impl MethodIo {
    pub(crate) fn is_empty(&self) -> bool {
        match self {
            Self::Input(s) => s.is_empty(),
            Self::Output(s) => s.is_empty(),
        }
    }
    pub(crate) fn node_name(&self) -> &str {
        match self {
            Self::Input(s) => s,
            Self::Output(s) => s,
        }
    }
}

impl fmt::Display for MethodIo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            MethodIo::Input(_) => write!(f, "input ({})", self.node_name()),
            MethodIo::Output(_) => write!(f, "output ({})", self.node_name()),
        }
    }
}

/// Options for a Method.
///
/// Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
/// framework.
pub struct MethodOptions {
    ///  Is this method deprecated?
    ///  Depending on the target platform, this can emit Deprecated annotations
    ///  for the method, or it will be completely ignored; in the very least,
    ///  this is a formalization for deprecating methods.
    pub deprecated: Option<bool>,
    pub idempotency_level: Option<IdempotencyLevel>,
    ///  The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub special_fields: SpecialFields,
}
impl MethodOptions {
    // Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
    //   framework.  We apologize for hoarding these numbers to ourselves, but
    //   we were already using them long before we decided to release Protocol
    //   Buffers.

    /// Is this method deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the method, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating methods.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// The parser stores options it doesn't recognize here. See above.
    pub fn uninterpreted_options(&self) -> &[UninterpretedOption] {
        (&self.opts().uninterpreted_option).into()
    }

    /// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
    /// or neither? HTTP based RPC implementation may choose GET verb for safe
    /// methods, and PUT verb for idempotent methods instead of the default POST.
    pub fn idempotency_level(&self) -> IdempotencyLevel {
        self.opts().idempotency_level().into()
    }
}
