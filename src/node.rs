use std::{ops::Deref, rc::Rc};

use crate::{Enum, EnumValue, Extension, Field, File, Message, Method, Name, Oneof, Service};

pub trait NodeAtPath<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>>;
}

pub trait FullyQualified {
    fn fully_qualified_name(&self) -> String;
}

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
    Extension(Rc<Extension<'a, U>>),
}

impl<'a, U> Node<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            Node::File(f) => f.name(),
            Node::Message(m) => m.name(),
            Node::Field(f) => f.name(),
            Node::Oneof(o) => o.name(),
            Node::Enum(e) => e.name(),
            Node::EnumValue(ev) => ev.name(),
            Node::Service(s) => s.name(),
            Node::Method(m) => m.name(),
            Node::Extension(e) => e.name(),
        }
    }
}

impl<'a, U> NodeAtPath<'a, U> for Node<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        match self {
            Node::File(f) => f.node_at_path(path),
            Node::Message(m) => m.node_at_path(path),
            Node::Oneof(o) => o.node_at_path(path),
            Node::Enum(e) => e.node_at_path(path),
            Node::EnumValue(ev) => ev.node_at_path(path),
            Node::Service(s) => s.node_at_path(path),
            Node::Method(m) => m.node_at_path(path),
            Node::Field(f) => f.node_at_path(path),
            Node::Extension(_) => todo!(),
        }
    }
}

pub trait AsNode<'a, U> {
    fn as_node(self) -> Node<'a, U>;
}

macro_rules! from_and_as_node {
    ($($t:ident),*) => {
        $(
            impl<'a, U> From<Rc<$t<'a, U>>> for Node<'a, U> {
                fn from(node: Rc<$t<'a, U>>) -> Self {
                    Node::$t(node)
                }
            }
            impl<'a, U> From<&Rc<$t<'a, U>>> for Node<'a, U> {
                fn from(node: &Rc<$t<'a, U>>) -> Self {
                    Node::$t(node.clone())
                }
            }
            impl<'a, U> AsNode<'a, U> for Rc<$t<'a, U>> {
                fn as_node(self) -> Node<'a, U> {
                    Node::from(self)
                }
            }
        )*
    };
}

from_and_as_node!(File, Message, Oneof, Enum, EnumValue, Service, Method, Field, Extension);

impl<'a, U> FullyQualified for Node<'a, Node<'a, U>> {
    fn fully_qualified_name(&self) -> String {
        match self {
            Node::File(f) => f.fully_qualified_name(),
            Node::Message(m) => m.fully_qualified_name(),
            Node::Oneof(o) => o.fully_qualified_name(),
            Node::Enum(e) => e.fully_qualified_name(),
            Node::EnumValue(ev) => ev.fully_qualified_name(),
            Node::Service(s) => s.fully_qualified_name(),
            Node::Method(m) => m.fully_qualified_name(),
            Node::Field(f) => f.fully_qualified_name(),
            Node::Extension(e) => e.fully_qualified_name(),
        }
    }
}

pub(crate) fn format_fqn<'a, N: FullyQualified>(n: &N, name: &str) -> String {
    format!("{}.{}", n.fully_qualified_name(), name)
}
