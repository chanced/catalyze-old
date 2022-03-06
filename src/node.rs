use crate::{
    Comments, Enum, EnumValue, Extension, Field, File, Message, Method, Name, Oneof, Package,
    Service,
};

pub(crate) trait NodeAtPath<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>>;
}

pub trait FullyQualified {
    fn fully_qualified_name(&self) -> String;
}

#[derive(Debug)]
pub enum Node<'a, U> {
    Package(Package<'a, U>),
    File(File<'a, U>),
    Message(Message<'a, U>),
    Oneof(Oneof<'a, U>),
    Enum(Enum<'a, U>),
    EnumValue(EnumValue<'a, U>),
    Service(Service<'a, U>),
    Method(Method<'a, U>),
    Field(Field<'a, U>),
    Extension(Extension<'a, U>),
}

impl<'a, U> Node<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            Node::Package(p) => p.name(),
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
    pub fn fully_qualified_name(&self) -> String {
        match self {
            Node::Package(p) => p.fully_qualified_name(),
            Node::File(f) => f.fully_qualified_name(),
            Node::Message(m) => m.fully_qualified_name(),
            Node::Field(f) => f.fully_qualified_name(),
            Node::Oneof(o) => o.fully_qualified_name(),
            Node::Enum(e) => e.fully_qualified_name(),
            Node::EnumValue(ev) => ev.fully_qualified_name(),
            Node::Service(s) => s.fully_qualified_name(),
            Node::Method(m) => m.fully_qualified_name(),
            Node::Extension(e) => e.fully_qualified_name(),
        }
    }

    pub(crate) fn set_comments(&self, c: Comments<'a, U>) {
        match self {
            Node::Message(m) => m.set_comments(c),
            Node::Field(f) => f.set_comments(c),
            Node::Oneof(o) => o.set_comments(c),
            Node::Enum(e) => e.set_comments(c),
            Node::EnumValue(ev) => ev.set_comments(c),
            Node::Service(s) => s.set_comments(c),
            Node::Method(m) => m.set_comments(c),
            Node::Extension(e) => e.set_comments(c),

            Node::Package(_) | Node::File(_) => unreachable!(),
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
            Node::Extension(_) => panic!("Can not path to an extension"), // TODO: confirm this
            Node::Package(_) => panic!("Can not path to a package"),
        }
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
            Node::Package(p) => p.fully_qualified_name(),
        }
    }
}
impl<'a, U> Clone for Node<'a, U> {
    fn clone(&self) -> Self {
        match self {
            Self::Package(p) => Self::Package(p.clone()),
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

pub(crate) fn format_fqn<'a, N: FullyQualified>(n: &N, name: &str) -> String {
    format!("{}.{}", n.fully_qualified_name(), name)
}
impl<'a, U> From<File<'a, U>> for Node<'a, U> {
    fn from(file: File<'a, U>) -> Self {
        Node::File(file)
    }
}
impl<'a, U> From<&File<'a, U>> for Node<'a, U> {
    fn from(file: &File<'a, U>) -> Self {
        Node::File(file.clone())
    }
}

impl<'a, U> From<Package<'a, U>> for Node<'a, U> {
    fn from(p: Package<'a, U>) -> Self {
        Node::Package(p)
    }
}

impl<'a, U> From<&Package<'a, U>> for Node<'a, U> {
    fn from(p: &Package<'a, U>) -> Self {
        Node::Package(p.clone())
    }
}

impl<'a, U> From<Message<'a, U>> for Node<'a, U> {
    fn from(m: Message<'a, U>) -> Self {
        Node::Message(m)
    }
}

impl<'a, U> From<&Message<'a, U>> for Node<'a, U> {
    fn from(m: &Message<'a, U>) -> Self {
        Node::Message(m.clone())
    }
}

impl<'a, U> From<Oneof<'a, U>> for Node<'a, U> {
    fn from(oneof: Oneof<'a, U>) -> Self {
        Node::Oneof(oneof)
    }
}
impl<'a, U> From<&Oneof<'a, U>> for Node<'a, U> {
    fn from(oneof: &Oneof<'a, U>) -> Self {
        Node::Oneof(oneof.clone())
    }
}
impl<'a, U> From<Field<'a, U>> for Node<'a, U> {
    fn from(field: Field<'a, U>) -> Self {
        Node::Field(field)
    }
}
impl<'a, U> From<&Field<'a, U>> for Node<'a, U> {
    fn from(field: &Field<'a, U>) -> Self {
        Node::Field(field.clone())
    }
}

impl<'a, U> From<Enum<'a, U>> for Node<'a, U> {
    fn from(e: Enum<'a, U>) -> Self {
        Node::Enum(e)
    }
}

impl<'a, U> From<&Enum<'a, U>> for Node<'a, U> {
    fn from(e: &Enum<'a, U>) -> Self {
        Node::Enum(e.clone())
    }
}

impl<'a, U> From<EnumValue<'a, U>> for Node<'a, U> {
    fn from(ev: EnumValue<'a, U>) -> Self {
        Node::EnumValue(ev)
    }
}

impl<'a, U> From<&EnumValue<'a, U>> for Node<'a, U> {
    fn from(ev: &EnumValue<'a, U>) -> Self {
        Node::EnumValue(ev.clone())
    }
}

impl<'a, U> From<Service<'a, U>> for Node<'a, U> {
    fn from(s: Service<'a, U>) -> Self {
        Node::Service(s)
    }
}

impl<'a, U> From<&Service<'a, U>> for Node<'a, U> {
    fn from(s: &Service<'a, U>) -> Self {
        Node::Service(s.clone())
    }
}

impl<'a, U> From<Method<'a, U>> for Node<'a, U> {
    fn from(m: Method<'a, U>) -> Self {
        Node::Method(m)
    }
}

impl<'a, U> From<&Method<'a, U>> for Node<'a, U> {
    fn from(m: &Method<'a, U>) -> Self {
        Node::Method(m.clone())
    }
}

impl<'a, U> From<Extension<'a, U>> for Node<'a, U> {
    fn from(e: Extension<'a, U>) -> Self {
        Node::Extension(e)
    }
}

impl<'a, U> From<&Extension<'a, U>> for Node<'a, U> {
    fn from(e: &Extension<'a, U>) -> Self {
        Node::Extension(e.clone())
    }
}

// File(File<'a, U>),
//     Message(Message<'a, U>),
//     Oneof(Oneof<'a, U>),
//     Enum(Enum<'a, U>),
//     EnumValue(EnumValue<'a, U>),
//     Service(Service<'a, U>),
//     Method(Method<'a, U>),
//     Field(Field<'a, U>),
//     Extension(Extension<'a, U>),
