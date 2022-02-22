use std::rc::Rc;

use crate::{Enum, EnumValue, Field, File, Message, Method, Name, Oneof, Service};

#[derive(Debug, Clone)]
pub enum Node<'a, U> {
    File(Rc<File<'a, U>>),
    Message(Rc<Message<'a, U>>),
    Oneof(Rc<Oneof<'a, U>>),
    Enum(Rc<Enum<'a, U>>),
    EnumValue(Rc<EnumValue<'a, U>>),
    Service(Rc<Service<'a, U>>),
    Method(Rc<Method<'a, U>>),
    Field(Rc<Field<'a, U>>),
}
impl<'a, U> Node<'a, U> {
    pub fn name(&self) -> &Name<U> {
        match self {
            Node::File(f) => &f.name,
            Node::Message(m) => &m.name,
            Node::Field(f) => &f.name,
            Node::Oneof(o) => &o.name,
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
            Node::Oneof(o) => &o.fully_qualified_name,
            Node::Enum(e) => &e.fully_qualified_name,
            Node::EnumValue(ev) => &ev.fully_qualified_name,
            Node::Service(s) => &s.fully_qualified_name,
            Node::Method(m) => &m.fully_qualified_name,
            Node::Field(f) => &f.fully_qualified_name,
        }
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        match self {
            Node::File(f) => f.node_at_path(path),
            Node::Message(m) => m.node_at_path(path),
            Node::Oneof(o) => o.node_at_path(path),
            Node::Enum(e) => e.node_at_path(path),
            Node::EnumValue(ev) => ev.node_at_path(path),
            Node::Service(s) => s.node_at_path(path),
            Node::Method(m) => m.node_at_path(path),
            Node::Field(f) => f.node_at_path(path),
        }
    }
}

impl<'a, U> From<File<'a, U>> for Node<'a, U> {
    fn from(file: File<'a, U>) -> Self {
        Node::File(Rc::new(file))
    }
}
impl<'a, U> From<Message<'a, U>> for Node<'a, U> {
    fn from(message: Message<'a, U>) -> Self {
        Node::Message(Rc::new(message))
    }
}
impl<'a, U> From<Oneof<'a, U>> for Node<'a, U> {
    fn from(oneof: Oneof<'a, U>) -> Self {
        Node::Oneof(Rc::new(oneof))
    }
}
impl<'a, U> From<Enum<'a, U>> for Node<'a, U> {
    fn from(enum_: Enum<'a, U>) -> Self {
        Node::Enum(Rc::new(enum_))
    }
}

impl<'a, U> From<Service<'a, U>> for Node<'a, U> {
    fn from(service: Service<'a, U>) -> Self {
        Node::Service(Rc::new(service))
    }
}
impl<'a, U> From<Method<'a, U>> for Node<'a, U> {
    fn from(method: Method<'a, U>) -> Self {
        Node::Method(Rc::new(method))
    }
}
impl<'a, U> From<Field<'a, U>> for Node<'a, U> {
    fn from(field: Field<'a, U>) -> Self {
        Node::Field(Rc::new(field))
    }
}

impl<'a, U> From<EnumValue<'a, U>> for Node<'a, U> {
    fn from(enum_value: EnumValue<'a, U>) -> Self {
        Node::EnumValue(Rc::new(enum_value))
    }
}

pub trait NodeAtPath<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>>;
}

pub(crate) fn format_fqn<'a, U>(container: &Node<'a, U>, name: &str) -> String {
    format!("{}.{}", container.fully_qualified_name(), name)
}
