use std::rc::Rc;

use crate::{Enum, EnumValue, Field, File, Message, Method, Name, Oneof, Service};

#[derive(Debug, Clone)]
pub enum Node<U> {
    File(Rc<File<U>>),
    Message(Rc<Message<U>>),
    OneOf(Rc<Oneof<U>>),
    Enum(Rc<Enum<U>>),
    EnumValue(Rc<EnumValue<U>>),
    Service(Rc<Service<U>>),
    Method(Rc<Method<U>>),
    Field(Rc<Field<U>>),
}
impl<U> Node<U> {
    pub fn name(&self) -> &Name<U> {
        match self {
            Node::File(f) => &f.name,
            Node::Message(m) => &m.name,
            Node::Field(f) => &f.name,
            Node::OneOf(o) => &o.name,
            Node::Enum(e) => &e.name,
            Node::EnumValue(ev) => &ev.name,
            Node::Service(s) => &s.name,
            Node::Method(m) => &m.name,
        }
    }
    pub fn fully_qualified_name(&self) -> &str {
        match self {
            Node::File(f) => &f.fully_qualified_name,
            Node::Message(m) => &m.fully_qualified_name,
            Node::OneOf(o) => &o.fully_qualified_name,
            Node::Enum(e) => &e.fully_qualified_name,
            Node::EnumValue(ev) => &ev.fully_qualified_name,
            Node::Service(s) => &s.fully_qualified_name,
            Node::Method(m) => &m.fully_qualified_name,
            Node::Field(f) => &f.fully_qualified_name,
        }
    }

    pub(crate) fn child_at_path(&self, path: &[i32]) -> Option<Node<U>> {
        match self {
            Node::File(f) => f.node_at_path(path),
            Node::Message(m) => m.node_at_path(path),
            Node::OneOf(o) => o.node_at_path(path),
            Node::Enum(e) => e.node_at_path(path),
            Node::EnumValue(ev) => ev.node_at_path(path),
            Node::Service(s) => s.node_at_path(path),
            Node::Method(m) => m.node_at_path(path),
            Node::Field(f) => f.node_at_path(path),
        }
    }
}

impl<U> From<File<U>> for Node<U> {
    fn from(file: File<U>) -> Self {
        Node::File(Rc::new(file))
    }
}
impl<U> From<Message<U>> for Node<U> {
    fn from(message: Message<U>) -> Self {
        Node::Message(Rc::new(message))
    }
}
impl<U> From<Oneof<U>> for Node<U> {
    fn from(oneof: Oneof<U>) -> Self {
        Node::OneOf(Rc::new(oneof))
    }
}
impl<U> From<Enum<U>> for Node<U> {
    fn from(enum_: Enum<U>) -> Self {
        Node::Enum(Rc::new(enum_))
    }
}

impl<U> From<Service<U>> for Node<U> {
    fn from(service: Service<U>) -> Self {
        Node::Service(Rc::new(service))
    }
}
impl<U> From<Method<U>> for Node<U> {
    fn from(method: Method<U>) -> Self {
        Node::Method(Rc::new(method))
    }
}
impl<U> From<Field<U>> for Node<U> {
    fn from(field: Field<U>) -> Self {
        Node::Field(Rc::new(field))
    }
}

impl<U> From<EnumValue<U>> for Node<U> {
    fn from(enum_value: EnumValue<U>) -> Self {
        Node::EnumValue(Rc::new(enum_value))
    }
}
