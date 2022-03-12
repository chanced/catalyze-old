use std::{cell::RefCell, rc::Rc};

use crate::proto::MethodDescriptor;
use crate::{
    Comments, File, FullyQualified, Message, Name, Node, Package, Service, WeakMessage, WeakService,
};

#[derive(Debug, Clone)]
struct MethodDetail<'a, U> {
    name: Name<U>,
    desc: MethodDescriptor<'a>,
    fqn: String,
    comments: RefCell<Comments<'a>>,
    service: WeakService<'a, U>,
    util: Rc<U>,
    input: Rc<RefCell<WeakMessage<'a, U>>>,
    output: Rc<RefCell<WeakMessage<'a, U>>>,
}

#[derive(Debug)]
pub struct Method<'a, U>(Rc<MethodDetail<'a, U>>);

impl<'a, U> Method<'a, U> {
    pub(crate) fn new(desc: MethodDescriptor<'a>, svc: Service<'a, U>) -> Self {
        let input = Rc::new(RefCell::new(WeakMessage::empty()));
        let output = Rc::new(RefCell::new(WeakMessage::empty()));
        let fqn = format!("{}.{}", svc.fully_qualified_name(), desc.name());
        Method(Rc::new(MethodDetail {
            name: Name::new(desc.name(), svc.util()),
            desc,
            fqn,
            comments: RefCell::new(Comments::default()),
            service: svc.clone().into(),
            util: svc.util(),
            input,
            output,
        }))
    }

    pub(crate) fn set_input(&self, msg: Message<'a, U>) {
        self.0.input.replace(msg.clone().into());
    }
    pub(crate) fn set_output(&self, msg: Message<'a, U>) {
        self.0.output.replace(msg.clone().into());
    }

    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
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
    pub fn service(&self) -> Service<'a, U> {
        self.0.service.clone().into()
    }
    pub fn file(&self) -> File<'a, U> {
        self.service().file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.file().package()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.comments.replace(comments);
    }

    pub fn util(&self) -> Rc<U> {
        self.0.util.clone()
    }
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            Some(self.into())
        } else {
            None
        }
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

    pub(crate) fn input_type(&self) -> &'a str {
        self.0.desc.input_type()
    }
    pub(crate) fn output_type(&self) -> &'a str {
        self.0.desc.output_type()
    }
}

impl<'a, U> Clone for Method<'a, U> {
    fn clone(&self) -> Self {
        Method(self.0.clone())
    }
}

impl<'a, U> FullyQualified for Method<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}
