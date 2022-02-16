use std::rc::Rc;

use crate::{Enum, EnumValue, Field, File, Message, Method, Name, Oneof, Service};

#[derive(Debug, Clone)]
pub enum Node<L> {
    File(Rc<File<L>>),
    Message(Rc<Message<L>>),
    OneOf(Rc<Oneof<L>>),
    Enum(Rc<Enum<L>>),
    EnumValue(Rc<EnumValue<L>>),
    Service(Rc<Service<L>>),
    Method(Rc<Method<L>>),
    Field(Rc<Field<L>>),
}
impl<L> Node<L> {
    pub fn name(&self) -> &Name<L> {
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
impl<L> From<File<L>> for Node<L> {
    fn from(file: File<L>) -> Self {
        Node::File(Rc::new(file))
    }
}
impl<L> From<Message<L>> for Node<L> {
    fn from(message: Message<L>) -> Self {
        Node::Message(Rc::new(message))
    }
}
impl<L> From<Oneof<L>> for Node<L> {
    fn from(oneof: Oneof<L>) -> Self {
        Node::OneOf(Rc::new(oneof))
    }
}
impl<L> From<Enum<L>> for Node<L> {
    fn from(enum_: Enum<L>) -> Self {
        Node::Enum(Rc::new(enum_))
    }
}

impl<L> From<Service<L>> for Node<L> {
    fn from(service: Service<L>) -> Self {
        Node::Service(Rc::new(service))
    }
}
impl<L> From<Method<L>> for Node<L> {
    fn from(method: Method<L>) -> Self {
        Node::Method(Rc::new(method))
    }
}
impl<L> From<Field<L>> for Node<L> {
    fn from(field: Field<L>) -> Self {
        Node::Field(Rc::new(field))
    }
}

impl<L> From<EnumValue<L>> for Node<L> {
    fn from(enum_value: EnumValue<L>) -> Self {
        Node::EnumValue(Rc::new(enum_value))
    }
}
