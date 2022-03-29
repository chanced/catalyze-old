use std::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::proto::MethodDescriptor;
use crate::{Comments, File, Message, Name, Node, Package, Service, WeakMessage, WeakService};

#[derive(Debug, Clone)]
struct MethodDetail<'a> {
    name: Name,
    desc: MethodDescriptor<'a>,
    fqn: String,
    comments: RefCell<Comments<'a>>,
    service: WeakService<'a>,

    input: Rc<RefCell<WeakMessage<'a>>>,
    output: Rc<RefCell<WeakMessage<'a>>>,
}

#[derive(Debug, Clone)]
pub struct Method<'a>(Rc<MethodDetail<'a>>);

impl<'a> Method<'a> {
    pub(crate) fn new(desc: MethodDescriptor<'a>, svc: Service<'a>) -> Self {
        let input = Rc::new(RefCell::new(WeakMessage::new()));
        let output = Rc::new(RefCell::new(WeakMessage::new()));
        let fqn = format!("{}.{}", svc.fully_qualified_name(), desc.name());
        Method(Rc::new(MethodDetail {
            name: desc.name().into(),
            desc,
            fqn,
            comments: RefCell::new(Comments::default()),
            service: svc.clone().into(),
            input,
            output,
        }))
    }

    pub(crate) fn set_input(&self, msg: Message<'a>) {
        self.0.input.replace(msg.clone().into());
    }
    pub(crate) fn set_output(&self, msg: Message<'a>) {
        self.0.output.replace(msg.clone().into());
    }

    pub fn name(&self) -> &Name {
        &self.0.name
    }
    pub fn descriptor(&self) -> MethodDescriptor<'a> {
        self.0.desc
    }
    pub fn comments(&self) -> Comments<'a> {
        Comments::default()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
    pub fn service(&self) -> Service<'a> {
        self.0.service.clone().into()
    }
    pub fn file(&self) -> File<'a> {
        self.service().file()
    }
    pub fn package(&self) -> Package<'a> {
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
    pub fn input(&self) -> Message<'a> {
        self.0.input.borrow().clone().into()
    }
    pub fn output(&self) -> Message<'a> {
        self.0.output.borrow().clone().into()
    }
    /// alias for `input`
    pub fn request(&self) -> Message<'a> {
        self.input()
    }
    /// alias for `output`
    pub fn response(&self) -> Message<'a> {
        self.output()
    }

    pub(crate) fn input_type(&self) -> &'a str {
        self.0.desc.input_type()
    }
    pub(crate) fn output_type(&self) -> &'a str {
        self.0.desc.output_type()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.comments.replace(comments);
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a>> {
        if path.is_empty() {
            Some(self.into())
        } else {
            None
        }
    }

    pub(crate) fn io(&self) -> (MethodIO<'_>, MethodIO<'_>) {
        (
            MethodIO::Input(self.input_type()),
            MethodIO::Output(self.output_type()),
        )
    }
}

pub(crate) enum MethodIO<'a> {
    Input(&'a str),
    Output(&'a str),
}

impl MethodIO<'_> {
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

impl fmt::Display for MethodIO<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            MethodIO::Input(_) => write!(f, "input"),
            MethodIO::Output(_) => write!(f, "output"),
        }
    }
}
