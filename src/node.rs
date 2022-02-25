use std::rc::Rc;

use crate::{Enum, EnumValue, Extension, Field, File, Message, Method, Name, Oneof, Service};

pub trait NodeAtPath<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>>;
}

pub trait FullyQualified {
    fn fully_qualified_name(&self) -> String;
}

#[derive(Debug)]
pub enum Node<'a, U> {
    File(File<'a, U>),
    Message(Message<'a, U>),
    Oneof(Rc<Oneof<'a, U>>),
    Enum(Rc<Enum<'a, U>>),
    EnumValue(Rc<EnumValue<'a, U>>),
    Service(Rc<Service<'a, U>>),
    Method(Rc<Method<'a, U>>),
    Field(Field<'a, U>),
    Extension(Rc<Extension<'a, U>>),
}

impl<'a, U> Clone for Node<'a, U> {
    fn clone(&self) -> Self {
        match self {
            Self::File(n) => Self::File(n.clone()),
            Self::Message(n) => Self::Message(n.clone()),
            Self::Oneof(n) => Self::Oneof(n.clone()),
            Self::Enum(n) => Self::Enum(n.clone()),
            Self::EnumValue(n) => Self::EnumValue(n.clone()),
            Self::Service(n) => Self::Service(n.clone()),
            Self::Method(n) => Self::Method(n.clone()),
            Self::Field(n) => Self::Field(n.clone()),
            Self::Extension(n) => Self::Extension(n.clone()),
        }
    }
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

pub trait IntoNode<'a, U> {
    fn into_node(self) -> Node<'a, U>;
}

macro_rules! from_and_into_node {
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
            impl<'a, U> IntoNode<'a, U> for Rc<$t<'a, U>> {
                fn into_node(self) -> Node<'a, U> {
                    Node::from(self)
                }
            }
        )*
    };
}

from_and_into_node!(File, Message, Oneof, Enum, EnumValue, Service, Method, Extension);

impl<'a, U> From<Field<'a, U>> for Node<'a, U> {
    fn from(field: Field<'a, U>) -> Self {
        Node::Field(field)
    }
}
impl<'a, U> IntoNode<'a, U> for Field<'a, U> {
    fn into_node(self) -> Node<'a, U> {
        Node::from(self)
    }
}

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
