use std::rc::Rc;

use crate::{Enum, Field, File, Lang, Message, Method, Name, Oneof, Service};

#[derive(Debug, Clone)]
pub enum Node<L: Lang> {
    File(Rc<File<L>>),
    Message(Rc<Message<L>>),
    OneOf(Rc<Oneof<L>>),
    Enum(Rc<Enum<L>>),
    Service(Rc<Service<L>>),
    Method(Rc<Method<L>>),
    Field(Rc<Field<L>>),
}
impl<L: Lang> Node<L> {
    pub fn name(&self) -> &Name<L> {
        match self {
            Node::File(f) => &f.name,
            Node::Message(m) => &m.name,
            Node::OneOf(o) => &o.name,
            Node::Enum(e) => &e.name,
            Node::Service(s) => &s.name,
            Node::Method(m) => &m.name,
            Node::Field(f) => &f.name,
        }
    }
}
impl<L: Lang> From<File<L>> for Node<L> {
    fn from(file: File<L>) -> Self {
        Node::File(Rc::new(file))
    }
}
impl<L: Lang> From<Message<L>> for Node<L> {
    fn from(message: Message<L>) -> Self {
        Node::Message(Rc::new(message))
    }
}
impl<L: Lang> From<Oneof<L>> for Node<L> {
    fn from(oneof: Oneof<L>) -> Self {
        Node::OneOf(Rc::new(oneof))
    }
}
impl<L: Lang> From<Enum<L>> for Node<L> {
    fn from(enum_: Enum<L>) -> Self {
        Node::Enum(Rc::new(enum_))
    }
}
impl<L: Lang> From<Service<L>> for Node<L> {
    fn from(service: Service<L>) -> Self {
        Node::Service(Rc::new(service))
    }
}
impl<L: Lang> From<Method<L>> for Node<L> {
    fn from(method: Method<L>) -> Self {
        Node::Method(Rc::new(method))
    }
}
impl<L: Lang> From<Field<L>> for Node<L> {
    fn from(field: Field<L>) -> Self {
        Node::Field(Rc::new(field))
    }
}
