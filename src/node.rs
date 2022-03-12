use crate::{
    iter::Iter, Comments, Enum, EnumValue, Extension, Field, File, Message, Method, Name, Oneof,
    Package, Service,
};
use crate::{AllMessages, Ast, MapField, RepeatedField};
use std::collections::VecDeque;
use std::convert::From;
use std::fmt::{self, Display};
use std::marker::PhantomData;
use std::rc::Rc;
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
impl<'a, U> Display for Node<'a, U> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Package(p) => write!(fmt, "Package({})", p.name()),
            Node::File(f) => write!(fmt, "File({})", f.name()),
            Node::Message(m) => write!(fmt, "Message({})", m.fully_qualified_name()),
            Node::Oneof(o) => write!(fmt, "Oneof({})", o.fully_qualified_name()),
            Node::Enum(e) => write!(fmt, "Enum({})", e.fully_qualified_name()),
            Node::EnumValue(e) => write!(fmt, "EnumValue({})", e.fully_qualified_name()),
            Node::Service(s) => write!(fmt, "Service({})", s.fully_qualified_name()),
            Node::Method(m) => write!(fmt, "Mehtod({})", m.fully_qualified_name()),
            Node::Field(f) => match f {
                Field::Embed(f) => write!(fmt, "EmbedField({})", f.fully_qualified_name()),
                Field::Enum(f) => write!(fmt, "EnumField({})", f.fully_qualified_name()),
                Field::Map(f) => match f {
                    MapField::Scalar(f) => {
                        write!(fmt, "MappedScalarField({})", f.fully_qualified_name())
                    }
                    MapField::Enum(f) => {
                        write!(fmt, "MappedEnumField({})", f.fully_qualified_name())
                    }
                    MapField::Embed(f) => {
                        write!(fmt, "MappedEmbedField({})", f.fully_qualified_name())
                    }
                },
                Field::Oneof(f) => match f {
                    crate::OneofField::Scalar(f) => {
                        write!(fmt, "OneofScalarField({})", f.fully_qualified_name())
                    }
                    crate::OneofField::Enum(f) => {
                        write!(fmt, "OneofEnumField({})", f.fully_qualified_name())
                    }
                    crate::OneofField::Embed(f) => {
                        write!(fmt, "OneofEmbedField({})", f.fully_qualified_name())
                    }
                },
                Field::Repeated(f) => match f {
                    RepeatedField::Scalar(f) => {
                        write!(fmt, "RepeatedScalarField({})", f.fully_qualified_name())
                    }
                    RepeatedField::Enum(f) => {
                        write!(fmt, "RepeatedEnumField({})", f.fully_qualified_name())
                    }
                    RepeatedField::Embed(f) => {
                        write!(fmt, "RepeatedEmbedField({})", f.fully_qualified_name())
                    }
                },
                Field::Scalar(f) => write!(fmt, "ScalarField({})", f.fully_qualified_name()),
            },
            Node::Extension(e) => write!(fmt, "Extension({})", e.fully_qualified_name()),
        }
    }
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

    pub fn nodes(&self) -> Nodes<'a, U> {
        match self {
            Node::Package(p) => p.nodes(),
            Node::File(f) => f.nodes(),
            Node::Message(m) => m.nodes(),
            Node::Enum(e) => e.nodes(),
            Node::Service(s) => s.nodes(),
            _ => Nodes::empty(),
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

    pub(crate) fn set_comments(&self, c: Comments<'a>) {
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
    pub fn util(&self) -> Rc<U> {
        match self {
            Node::Package(n) => n.util(),
            Node::File(n) => n.util(),
            Node::Message(n) => n.util(),
            Node::Oneof(n) => n.util(),
            Node::Enum(n) => n.util(),
            Node::EnumValue(n) => n.util(),
            Node::Service(n) => n.util(),
            Node::Method(n) => n.util(),
            Node::Field(n) => n.util(),
            Node::Extension(n) => n.util(),
        }
    }

    pub fn package(&self) -> Package<'a, U> {
        match self {
            Node::Package(p) => p.clone(),
            Node::File(f) => f.package(),
            Node::Message(m) => m.package(),
            Node::Field(f) => f.package(),
            Node::Oneof(o) => o.package(),
            Node::Enum(e) => e.package(),
            Node::EnumValue(ev) => ev.package(),
            Node::Service(s) => s.package(),
            Node::Method(m) => m.package(),
            Node::Extension(e) => e.package(),
        }
    }

    pub(crate) fn add_dependent(&self, dep: Message<'a, U>) {
        match self {
            Node::Message(m) => m.add_dependent(dep),
            Node::Enum(e) => e.add_dependent(dep),
            _ => unreachable!(),
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
            Node::Extension(e) => e.node_at_path(path), // TODO: confirm this
            Node::Package(_) => unreachable!(),
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

pub(crate) fn format_fqn<N: FullyQualified>(n: &N, name: &str) -> String {
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

#[derive(Debug, Clone)]
pub enum NodeIter<'a, U, T = Node<'a, U>> {
    Nodes(Nodes<'a, U>),
    Packages(Iter<Package<'a, U>>),
    Files(Iter<File<'a, U>>),
    Messages(Iter<Message<'a, U>>),
    Oneofs(Iter<Oneof<'a, U>>),
    Enums(Iter<Enum<'a, U>>),
    EnumValues(Iter<EnumValue<'a, U>>),
    Services(Iter<Service<'a, U>>),
    Methods(Iter<Method<'a, U>>),
    Fields(Iter<Field<'a, U>>),
    Extensions(Iter<Extension<'a, U>>),
    _Phantom(PhantomData<T>),
}
impl<'a, U, T> NodeIter<'a, U, T> {
    pub fn len(&self) -> usize {
        match self {
            NodeIter::Nodes(nodes) => nodes.len(),
            NodeIter::Files(i) => i.len(),
            NodeIter::Messages(i) => i.len(),
            NodeIter::Oneofs(i) => i.len(),
            NodeIter::Enums(i) => i.len(),
            NodeIter::EnumValues(i) => i.len(),
            NodeIter::Services(i) => i.len(),
            NodeIter::Methods(i) => i.len(),
            NodeIter::Fields(i) => i.len(),
            NodeIter::Extensions(i) => i.len(),
            NodeIter::Packages(i) => i.len(),
            NodeIter::_Phantom(_) => unreachable!(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a, U> Iterator for NodeIter<'a, U, Node<'a, U>> {
    type Item = Node<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            NodeIter::Nodes(nodes) => nodes.next(),
            NodeIter::Packages(i) => i.next().map(Into::into),
            NodeIter::Files(i) => i.next().map(Into::into),
            NodeIter::Messages(i) => i.next().map(Into::into),
            NodeIter::Oneofs(i) => i.next().map(Into::into),
            NodeIter::Enums(i) => i.next().map(Into::into),
            NodeIter::EnumValues(i) => i.next().map(Into::into),
            NodeIter::Services(i) => i.next().map(Into::into),
            NodeIter::Methods(i) => i.next().map(Into::into),
            NodeIter::Fields(i) => i.next().map(Into::into),
            NodeIter::Extensions(i) => i.next().map(Into::into),
            NodeIter::_Phantom(_) => unreachable!(),
        }
    }
}
impl<'a, U> From<Nodes<'a, U>> for NodeIter<'a, U> {
    fn from(i: Nodes<'a, U>) -> Self {
        NodeIter::Nodes(i)
    }
}

impl<'a, U> From<Iter<Message<'a, U>>> for NodeIter<'a, U> {
    fn from(i: Iter<Message<'a, U>>) -> Self {
        NodeIter::Messages(i)
    }
}
impl<'a, U> From<Iter<Oneof<'a, U>>> for NodeIter<'a, U> {
    fn from(i: Iter<Oneof<'a, U>>) -> Self {
        NodeIter::Oneofs(i)
    }
}
impl<'a, U> From<Iter<Enum<'a, U>>> for NodeIter<'a, U> {
    fn from(i: Iter<Enum<'a, U>>) -> Self {
        NodeIter::Enums(i)
    }
}
impl<'a, U> From<Iter<EnumValue<'a, U>>> for NodeIter<'a, U> {
    fn from(i: Iter<EnumValue<'a, U>>) -> Self {
        NodeIter::EnumValues(i)
    }
}
impl<'a, U> From<Iter<Service<'a, U>>> for NodeIter<'a, U> {
    fn from(i: Iter<Service<'a, U>>) -> Self {
        NodeIter::Services(i)
    }
}
impl<'a, U> From<Iter<Method<'a, U>>> for NodeIter<'a, U> {
    fn from(i: Iter<Method<'a, U>>) -> Self {
        NodeIter::Methods(i)
    }
}
impl<'a, U> From<Iter<Field<'a, U>>> for NodeIter<'a, U> {
    fn from(i: Iter<Field<'a, U>>) -> Self {
        NodeIter::Fields(i)
    }
}
impl<'a, U> From<Iter<Extension<'a, U>>> for NodeIter<'a, U> {
    fn from(i: Iter<Extension<'a, U>>) -> Self {
        NodeIter::Extensions(i)
    }
}
impl<'a, U> From<Iter<File<'a, U>>> for NodeIter<'a, U> {
    fn from(i: Iter<File<'a, U>>) -> Self {
        NodeIter::Files(i)
    }
}

#[derive(Debug, Clone)]
pub struct Nodes<'a, U, T = Node<'a, U>> {
    _marker: PhantomData<T>,
    iters: VecDeque<NodeIter<'a, U>>,
}

impl<'a, U> Nodes<'a, U> {
    pub fn new(iters: Vec<NodeIter<'a, U>>) -> Nodes<'a, U> {
        Nodes {
            _marker: PhantomData,
            iters: iters.into(),
        }
    }
    pub fn empty() -> Nodes<'a, U> {
        Nodes {
            _marker: PhantomData,
            iters: VecDeque::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.iters.iter().map(NodeIter::len).sum()
    }
    pub fn is_empty(&self) -> bool {
        self.iters.iter().all(NodeIter::is_empty)
    }
    pub fn push_back<I: Into<NodeIter<'a, U>>>(&mut self, i: I) {
        self.iters.push_back(i.into());
    }
    pub fn push_front<I: Into<NodeIter<'a, U>>>(&mut self, i: I) {
        self.iters.push_front(i.into());
    }
}

impl<'a, U> Iterator for Nodes<'a, U, Node<'a, U>> {
    type Item = Node<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(i) = self.iters.front_mut() {
                if let Some(n) = i.next() {
                    return Some(n);
                } else {
                    self.iters.pop_front();
                }
            } else {
                return None;
            }
        }
    }
}

pub struct AllNodes<'a, U, T = Node<'a, U>> {
    iter: Nodes<'a, U>,
    _marker: PhantomData<T>,
}

impl<'a, U> AllNodes<'a, U, Node<'a, U>> {
    pub fn new(node: Node<'a, U>) -> AllNodes<'a, U, Node<'a, U>> {
        AllNodes {
            iter: node.nodes(),
            _marker: PhantomData,
        }
    }
    pub fn push_back(&mut self, nodes: Nodes<'a, U>) {
        self.iter.push_back(nodes);
    }
}

impl<'a, U> Iterator for AllNodes<'a, U> {
    type Item = Node<'a, U>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.iter.next() {
            let i = n.nodes();
            if !i.is_empty() {
                self.iter.push_front(i);
            }
            Some(n)
        } else {
            None
        }
    }
}
impl<'a, U> From<Nodes<'a, U>> for AllNodes<'a, U> {
    fn from(i: Nodes<'a, U>) -> Self {
        AllNodes {
            iter: i,
            _marker: PhantomData,
        }
    }
}

impl<'a, U> From<&Ast<'a, U>> for AllNodes<'a, U> {
    fn from(ast: &Ast<'a, U>) -> Self {
        AllNodes {
            iter: Nodes::new(vec![NodeIter::Packages(ast.packages())]),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{proto::FileDescriptor, util::Generic, *};

    #[test]
    fn test_nodes() {
        let u = Rc::new(Generic {});
        let pkg = Package::new("pkg", u);

        let f = File::new(true, FileDescriptor::default(), pkg.clone()).unwrap();
        let m1 = Message::new(Default::default(), f.clone().into()).unwrap();
        let m2 = Message::new(Default::default(), f.clone().into()).unwrap();
        f.add_node(m1.into());
        f.add_node(m2.into());
        let mut count = 0;
        for n in f.nodes() {
            count += 1;
            println!("{:?}", n)
        }
        assert_eq!(count, 2)
    }
    #[test]
    fn test_all_nodes() {
        let u = Rc::new(Generic {});
        let pkg = Package::new("pkg", u);

        let f = File::new(true, FileDescriptor::default(), pkg.clone()).unwrap();
        let m1 = Message::new(Default::default(), f.clone().into()).unwrap();
        let m1e1 = Enum::new(Default::default(), m1.clone().into());
        m1.add_node(m1e1.into());
        let m2 = Message::new(Default::default(), f.clone().into()).unwrap();
        f.add_node(m1.into());
        f.add_node(m2.into());
        let mut count = 0;
        for n in f.all_nodes() {
            count += 1;
            println!("{:?}", n)
        }
        assert_eq!(count, 3)
    }
}
