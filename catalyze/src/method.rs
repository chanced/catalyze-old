use crate::{Comments, File, Message, Node, Package, Service, WeakMessage, WeakService};
use protobuf::descriptor::MethodDescriptorProto as MethodDescriptor;
use std::fmt;
use std::{cell::RefCell, rc::Rc};
pub struct Io<'a> {
    pub input: &'a str,
    pub output: &'a str,
}

#[derive(Debug, Clone)]
struct MethodDetail {
    descriptor: MethodDescriptor,
    fqn: String,
    comments: RefCell<Comments>,
    service: WeakService,
    input: RefCell<WeakMessage>,
    output: RefCell<WeakMessage>,
}

#[derive(Debug, Clone)]
pub struct Method(Rc<MethodDetail>);

impl Method {
    pub(crate) fn new(descriptor: MethodDescriptor, svc: Service) -> Self {
        let input = Rc::new(RefCell::new(WeakMessage::new()));
        let output = Rc::new(RefCell::new(WeakMessage::new()));
        let fqn = format!("{}.{}", svc.fully_qualified_name(), descriptor.name());
        Method(Rc::new(MethodDetail {
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
