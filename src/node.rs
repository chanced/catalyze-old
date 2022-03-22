use anyhow::bail;

use crate::{
    iter::Iter, Comments, Enum, EnumValue, Extension, Field, File, Message, Method, Name, Oneof,
    Package, Service,
};
use crate::{Ast, MapField, RepeatedField};
use std::collections::VecDeque;
use std::convert::From;
use std::fmt::{self, Display};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub enum Node<'a> {
    Package(Package<'a>),
    File(File<'a>),
    Message(Message<'a>),
    Oneof(Oneof<'a>),
    Enum(Enum<'a>),
    EnumValue(EnumValue<'a>),
    Service(Service<'a>),
    Method(Method<'a>),
    Field(Field<'a>),
    Extension(Extension<'a>),
}

impl<'a> Node<'a> {
    pub fn name(&self) -> &Name {
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

    pub fn nodes(&self) -> Nodes<'a> {
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
    pub fn as_package(self) -> anyhow::Result<Package<'a>> {
        match self {
            Node::Package(p) => Ok(p),
            _ => bail!("err: {} is not a Package", self),
        }
    }
    pub fn as_file(self) -> anyhow::Result<File<'a>> {
        match self {
            Node::File(f) => Ok(f),
            _ => bail!("err: {} is not a File", self),
        }
    }
    pub fn as_message(self) -> anyhow::Result<Message<'a>> {
        match self {
            Node::Message(m) => Ok(m),
            _ => bail!("err: {} is not a Message", self),
        }
    }
    pub fn as_oneof(self) -> anyhow::Result<Oneof<'a>> {
        match self {
            Node::Oneof(o) => Ok(o),
            _ => bail!("err: {} is not a Oneof", self),
        }
    }
    pub fn as_enum(self) -> anyhow::Result<Enum<'a>> {
        match self {
            Node::Enum(e) => Ok(e),
            _ => bail!("err: {} is not a Enum", self),
        }
    }
    pub fn as_enum_value(self) -> anyhow::Result<EnumValue<'a>> {
        match self {
            Node::EnumValue(ev) => Ok(ev),
            _ => bail!("err: {} is not a EnumValue", self),
        }
    }
    pub fn as_service(self) -> anyhow::Result<Service<'a>> {
        match self {
            Node::Service(s) => Ok(s),
            _ => bail!("err: {} is not a Service", self),
        }
    }
    pub fn as_method(self) -> anyhow::Result<Method<'a>> {
        match self {
            Node::Method(m) => Ok(m),
            _ => bail!("err: {} is not a Method", self),
        }
    }
    pub fn as_field(self) -> anyhow::Result<Field<'a>> {
        match self {
            Node::Field(f) => Ok(f),
            _ => bail!("err: {} is not a Field", self),
        }
    }
    pub fn as_extension(self) -> anyhow::Result<Extension<'a>> {
        match self {
            Node::Extension(e) => Ok(e),
            _ => bail!("err: {} is not a Extension", self),
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

    pub fn package(&self) -> Package<'a> {
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

    pub(crate) fn add_dependent(&self, dep: Message<'a>) {
        match self {
            Node::Message(m) => m.add_dependent(dep),
            Node::Enum(e) => e.add_dependent(dep),
            _ => unreachable!(),
        }
    }
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a>> {
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
impl<'a> Display for Node<'a> {
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

impl<'a> From<File<'a>> for Node<'a> {
    fn from(file: File<'a>) -> Self {
        Node::File(file)
    }
}
impl<'a> From<&File<'a>> for Node<'a> {
    fn from(file: &File<'a>) -> Self {
        Node::File(file.clone())
    }
}

impl<'a> From<Package<'a>> for Node<'a> {
    fn from(p: Package<'a>) -> Self {
        Node::Package(p)
    }
}

impl<'a> From<&Package<'a>> for Node<'a> {
    fn from(p: &Package<'a>) -> Self {
        Node::Package(p.clone())
    }
}

impl<'a> From<Message<'a>> for Node<'a> {
    fn from(m: Message<'a>) -> Self {
        Node::Message(m)
    }
}

impl<'a> From<&Message<'a>> for Node<'a> {
    fn from(m: &Message<'a>) -> Self {
        Node::Message(m.clone())
    }
}

impl<'a> From<Oneof<'a>> for Node<'a> {
    fn from(oneof: Oneof<'a>) -> Self {
        Node::Oneof(oneof)
    }
}
impl<'a> From<&Oneof<'a>> for Node<'a> {
    fn from(oneof: &Oneof<'a>) -> Self {
        Node::Oneof(oneof.clone())
    }
}
impl<'a> From<Field<'a>> for Node<'a> {
    fn from(field: Field<'a>) -> Self {
        Node::Field(field)
    }
}
impl<'a> From<&Field<'a>> for Node<'a> {
    fn from(field: &Field<'a>) -> Self {
        Node::Field(field.clone())
    }
}

impl<'a> From<Enum<'a>> for Node<'a> {
    fn from(e: Enum<'a>) -> Self {
        Node::Enum(e)
    }
}

impl<'a> From<&Enum<'a>> for Node<'a> {
    fn from(e: &Enum<'a>) -> Self {
        Node::Enum(e.clone())
    }
}

impl<'a> From<EnumValue<'a>> for Node<'a> {
    fn from(ev: EnumValue<'a>) -> Self {
        Node::EnumValue(ev)
    }
}

impl<'a> From<&EnumValue<'a>> for Node<'a> {
    fn from(ev: &EnumValue<'a>) -> Self {
        Node::EnumValue(ev.clone())
    }
}

impl<'a> From<Service<'a>> for Node<'a> {
    fn from(s: Service<'a>) -> Self {
        Node::Service(s)
    }
}

impl<'a> From<&Service<'a>> for Node<'a> {
    fn from(s: &Service<'a>) -> Self {
        Node::Service(s.clone())
    }
}

impl<'a> From<Method<'a>> for Node<'a> {
    fn from(m: Method<'a>) -> Self {
        Node::Method(m)
    }
}

impl<'a> From<&Method<'a>> for Node<'a> {
    fn from(m: &Method<'a>) -> Self {
        Node::Method(m.clone())
    }
}

impl<'a> From<Extension<'a>> for Node<'a> {
    fn from(e: Extension<'a>) -> Self {
        Node::Extension(e)
    }
}

impl<'a> From<&Extension<'a>> for Node<'a> {
    fn from(e: &Extension<'a>) -> Self {
        Node::Extension(e.clone())
    }
}

// File(File<'a>),
//     Message(Message<'a>),
//     Oneof(Oneof<'a>),
//     Enum(Enum<'a>),
//     EnumValue(EnumValue<'a>),
//     Service(Service<'a>),
//     Method(Method<'a>),
//     Field(Field<'a>),
//     Extension(Extension<'a>),

#[derive(Debug, Clone)]
pub enum NodeIter<'a, T = Node<'a>> {
    Nodes(Nodes<'a>),
    Packages(Iter<Package<'a>>),
    Files(Iter<File<'a>>),
    Messages(Iter<Message<'a>>),
    Oneofs(Iter<Oneof<'a>>),
    Enums(Iter<Enum<'a>>),
    EnumValues(Iter<EnumValue<'a>>),
    Services(Iter<Service<'a>>),
    Methods(Iter<Method<'a>>),
    Fields(Iter<Field<'a>>),
    Extensions(Iter<Extension<'a>>),
    _Phantom(PhantomData<T>),
}
impl<'a, T> NodeIter<'a, T> {
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
impl<'a> Iterator for NodeIter<'a, Node<'a>> {
    type Item = Node<'a>;
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
impl<'a> From<Nodes<'a>> for NodeIter<'a> {
    fn from(i: Nodes<'a>) -> Self {
        NodeIter::Nodes(i)
    }
}

impl<'a> From<Iter<Message<'a>>> for NodeIter<'a> {
    fn from(i: Iter<Message<'a>>) -> Self {
        NodeIter::Messages(i)
    }
}
impl<'a> From<Iter<Oneof<'a>>> for NodeIter<'a> {
    fn from(i: Iter<Oneof<'a>>) -> Self {
        NodeIter::Oneofs(i)
    }
}
impl<'a> From<Iter<Enum<'a>>> for NodeIter<'a> {
    fn from(i: Iter<Enum<'a>>) -> Self {
        NodeIter::Enums(i)
    }
}
impl<'a> From<Iter<EnumValue<'a>>> for NodeIter<'a> {
    fn from(i: Iter<EnumValue<'a>>) -> Self {
        NodeIter::EnumValues(i)
    }
}
impl<'a> From<Iter<Service<'a>>> for NodeIter<'a> {
    fn from(i: Iter<Service<'a>>) -> Self {
        NodeIter::Services(i)
    }
}
impl<'a> From<Iter<Method<'a>>> for NodeIter<'a> {
    fn from(i: Iter<Method<'a>>) -> Self {
        NodeIter::Methods(i)
    }
}
impl<'a> From<Iter<Field<'a>>> for NodeIter<'a> {
    fn from(i: Iter<Field<'a>>) -> Self {
        NodeIter::Fields(i)
    }
}
impl<'a> From<Iter<Extension<'a>>> for NodeIter<'a> {
    fn from(i: Iter<Extension<'a>>) -> Self {
        NodeIter::Extensions(i)
    }
}
impl<'a> From<Iter<File<'a>>> for NodeIter<'a> {
    fn from(i: Iter<File<'a>>) -> Self {
        NodeIter::Files(i)
    }
}

#[derive(Debug, Clone)]
pub struct Nodes<'a, T = Node<'a>> {
    _marker: PhantomData<T>,
    iters: VecDeque<NodeIter<'a>>,
}

impl<'a> Nodes<'a> {
    pub fn new(iters: Vec<NodeIter<'a>>) -> Nodes<'a> {
        Nodes {
            _marker: PhantomData,
            iters: iters.into(),
        }
    }
    pub fn empty() -> Nodes<'a> {
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
    pub fn push_back<I: Into<NodeIter<'a>>>(&mut self, i: I) {
        self.iters.push_back(i.into());
    }
    pub fn push_front<I: Into<NodeIter<'a>>>(&mut self, i: I) {
        self.iters.push_front(i.into());
    }
}

impl<'a> Iterator for Nodes<'a, Node<'a>> {
    type Item = Node<'a>;
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

pub struct AllNodes<'a, T = Node<'a>> {
    iter: Nodes<'a>,
    _marker: PhantomData<T>,
}

impl<'a> AllNodes<'a, Node<'a>> {
    pub fn new(node: Node<'a>) -> AllNodes<'a, Node<'a>> {
        AllNodes {
            iter: node.nodes(),
            _marker: PhantomData,
        }
    }
    pub fn push_back(&mut self, nodes: Nodes<'a>) {
        self.iter.push_back(nodes);
    }
}

impl<'a> Iterator for AllNodes<'a> {
    type Item = Node<'a>;

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
impl<'a> From<Nodes<'a>> for AllNodes<'a> {
    fn from(i: Nodes<'a>) -> Self {
        AllNodes {
            iter: i,
            _marker: PhantomData,
        }
    }
}

impl<'a> From<&Ast<'a>> for AllNodes<'a> {
    fn from(ast: &Ast<'a>) -> Self {
        AllNodes {
            iter: Nodes::new(vec![NodeIter::Packages(ast.packages())]),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_nodes() {
        let pkg = Package::new("pkg");

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
        let pkg = Package::new("pkg");

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
