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
            Node::OneOf(o) => &o.name,
            Node::Enum(e) => &e.name,
            Node::Service(s) => &s.name,
            Node::Method(m) => &m.name,
            Node::Field(f) => &f.name,
            Node::EnumValue(ev) => &ev.name,
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
