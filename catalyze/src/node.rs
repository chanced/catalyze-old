use crate::{
    iter::Iter, Comments, Enum, EnumValue, Extension, Field, File, Message, Method, Oneof, Package,
    Service,
};
use crate::{Ast, MapField, RepeatedField};
use std::collections::VecDeque;
use std::convert::From;
use std::fmt::{self, Display};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Package,
    File,
    Message,
    Oneof,
    Enum,
    EnumValue,
    Service,
    Method,
    Field,
    Extension,
}

impl Display for Kind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Package => write!(fmt, "Package"),
            Kind::File => write!(fmt, "File"),
            Kind::Message => write!(fmt, "Message"),
            Kind::Oneof => write!(fmt, "Oneof"),
            Kind::Enum => write!(fmt, "Enum"),
            Kind::EnumValue => write!(fmt, "EnumValue"),
            Kind::Service => write!(fmt, "Service"),
            Kind::Method => write!(fmt, "Method"),
            Kind::Field => write!(fmt, "Field"),
            Kind::Extension => write!(fmt, "Extension"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Package(Package),
    File(File),
    Message(Message),
    Oneof(Oneof),
    Enum(Enum),
    EnumValue(EnumValue),
    Service(Service),
    Method(Method),
    Field(Field),
    Extension(Extension),
}

impl Node {
    pub fn name(&self) -> &str {
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
    pub fn kind(&self) -> Kind {
        match self {
            Node::Package(_) => Kind::Package,
            Node::File(_) => Kind::File,
            Node::Message(_) => Kind::Message,
            Node::Field(_) => Kind::Field,
            Node::Oneof(_) => Kind::Oneof,
            Node::Enum(_) => Kind::Enum,
            Node::EnumValue(_) => Kind::EnumValue,
            Node::Service(_) => Kind::Service,
            Node::Method(_) => Kind::Method,
            Node::Extension(_) => Kind::Extension,
        }
    }
    pub fn nodes(&self) -> Nodes {
        match self {
            Node::Package(p) => p.nodes(),
            Node::File(f) => f.nodes(),
            Node::Message(m) => m.nodes(),
            Node::Enum(e) => e.nodes(),
            Node::Service(s) => s.nodes(),
            _ => Nodes::empty(),
        }
    }
    pub fn fully_qualified_name(&self) -> &str {
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
    // pub fn into_package(self) -> Result<Package, Node> {
    //     let Self::Package(p) = self else {
    //         return Err(self);
    //     };
    //     Ok(p)
    // }

    // pub fn into_file(self) -> Result<File, Node> {
    //     let Self::File(f) = self else {
    //         return Err(self);
    //     };
    //     Ok(f)
    // }

    pub(crate) fn set_comments(&self, c: Comments) {
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

    pub fn package(&self) -> Package {
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

    pub(crate) fn add_dependent(&self, dep: Message) {
        match self {
            Node::Message(m) => m.add_dependent(dep),
            Node::Enum(e) => e.add_dependent(dep),
            _ => unreachable!(),
        }
    }
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node> {
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

    /// Returns `true` if the node is [`Package`].
    ///
    /// [`Package`]: Node::Package
    #[must_use]
    pub fn is_package(&self) -> bool {
        matches!(self, Self::Package(..))
    }

    #[must_use]
    pub fn as_package(&self) -> Option<&Package> {
        if let Self::Package(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_package(self) -> Result<Package, Self> {
        if let Self::Package(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the node is [`File`].
    ///
    /// [`File`]: Node::File
    #[must_use]
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(..))
    }

    #[must_use]
    pub fn as_file(&self) -> Option<&File> {
        if let Self::File(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_file(self) -> Result<File, Self> {
        if let Self::File(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the node is [`Message`].
    ///
    /// [`Message`]: Node::Message
    #[must_use]
    pub fn is_message(&self) -> bool {
        matches!(self, Self::Message(..))
    }

    /// Returns `true` if the node is [`Oneof`].
    ///
    /// [`Oneof`]: Node::Oneof
    #[must_use]
    pub fn is_oneof(&self) -> bool {
        matches!(self, Self::Oneof(..))
    }

    #[must_use]
    pub fn as_oneof(&self) -> Option<&Oneof> {
        if let Self::Oneof(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_oneof(self) -> Result<Oneof, Self> {
        if let Self::Oneof(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the node is [`Enum`].
    ///
    /// [`Enum`]: Node::Enum
    #[must_use]
    pub fn is_enum(&self) -> bool {
        matches!(self, Self::Enum(..))
    }

    #[must_use]
    pub fn as_enum(&self) -> Option<&Enum> {
        if let Self::Enum(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_enum(self) -> Result<Enum, Self> {
        if let Self::Enum(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    #[must_use]
    pub fn as_message(&self) -> Option<&Message> {
        if let Self::Message(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_message(self) -> Result<Message, Self> {
        if let Self::Message(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the node is [`EnumValue`].
    ///
    /// [`EnumValue`]: Node::EnumValue
    #[must_use]
    pub fn is_enum_value(&self) -> bool {
        matches!(self, Self::EnumValue(..))
    }

    #[must_use]
    pub fn as_enum_value(&self) -> Option<&EnumValue> {
        if let Self::EnumValue(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_enum_value(self) -> Result<EnumValue, Self> {
        if let Self::EnumValue(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the node is [`Service`].
    ///
    /// [`Service`]: Node::Service
    #[must_use]
    pub fn is_service(&self) -> bool {
        matches!(self, Self::Service(..))
    }

    #[must_use]
    pub fn as_service(&self) -> Option<&Service> {
        if let Self::Service(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_service(self) -> Result<Service, Self> {
        if let Self::Service(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the node is [`Method`].
    ///
    /// [`Method`]: Node::Method
    #[must_use]
    pub fn is_method(&self) -> bool {
        matches!(self, Self::Method(..))
    }

    #[must_use]
    pub fn as_method(&self) -> Option<&Method> {
        if let Self::Method(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_method(self) -> Result<Method, Self> {
        if let Self::Method(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the node is [`Field`].
    ///
    /// [`Field`]: Node::Field
    #[must_use]
    pub fn is_field(&self) -> bool {
        matches!(self, Self::Field(..))
    }

    #[must_use]
    pub fn as_field(&self) -> Option<&Field> {
        if let Self::Field(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_field(self) -> Result<Field, Self> {
        if let Self::Field(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the node is [`Extension`].
    ///
    /// [`Extension`]: Node::Extension
    #[must_use]
    pub fn is_extension(&self) -> bool {
        matches!(self, Self::Extension(..))
    }

    #[must_use]
    pub fn as_extension(&self) -> Option<&Extension> {
        if let Self::Extension(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_extension(self) -> Result<Extension, Self> {
        if let Self::Extension(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}
impl Display for Node {
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

pub trait IntoNode {
    fn into_node(self) -> Node;
}

impl From<File> for Node {
    fn from(file: File) -> Self {
        Node::File(file)
    }
}
impl From<&File> for Node {
    fn from(file: &File) -> Self {
        Node::File(file.clone())
    }
}

impl From<Package> for Node {
    fn from(p: Package) -> Self {
        Node::Package(p)
    }
}

impl From<&Package> for Node {
    fn from(p: &Package) -> Self {
        Node::Package(p.clone())
    }
}

impl From<Message> for Node {
    fn from(m: Message) -> Self {
        Node::Message(m)
    }
}

impl From<&Message> for Node {
    fn from(m: &Message) -> Self {
        Node::Message(m.clone())
    }
}

impl From<Oneof> for Node {
    fn from(oneof: Oneof) -> Self {
        Node::Oneof(oneof)
    }
}
impl From<&Oneof> for Node {
    fn from(oneof: &Oneof) -> Self {
        Node::Oneof(oneof.clone())
    }
}
impl From<Field> for Node {
    fn from(field: Field) -> Self {
        Node::Field(field)
    }
}
impl From<&Field> for Node {
    fn from(field: &Field) -> Self {
        Node::Field(field.clone())
    }
}

impl From<Enum> for Node {
    fn from(e: Enum) -> Self {
        Node::Enum(e)
    }
}

impl From<&Enum> for Node {
    fn from(e: &Enum) -> Self {
        Node::Enum(e.clone())
    }
}

impl From<EnumValue> for Node {
    fn from(ev: EnumValue) -> Self {
        Node::EnumValue(ev)
    }
}

impl From<&EnumValue> for Node {
    fn from(ev: &EnumValue) -> Self {
        Node::EnumValue(ev.clone())
    }
}

impl From<Service> for Node {
    fn from(s: Service) -> Self {
        Node::Service(s)
    }
}

impl From<&Service> for Node {
    fn from(s: &Service) -> Self {
        Node::Service(s.clone())
    }
}

impl From<Method> for Node {
    fn from(m: Method) -> Self {
        Node::Method(m)
    }
}

impl From<&Method> for Node {
    fn from(m: &Method) -> Self {
        Node::Method(m.clone())
    }
}

impl From<Extension> for Node {
    fn from(e: Extension) -> Self {
        Node::Extension(e)
    }
}

impl From<&Extension> for Node {
    fn from(e: &Extension) -> Self {
        Node::Extension(e.clone())
    }
}

// File(File),
//     Message(Message),
//     Oneof(Oneof),
//     Enum(Enum),
//     EnumValue(EnumValue),
//     Service(Service),
//     Method(Method),
//     Field(Field),
//     Extension(Extension),

#[derive(Debug, Clone)]
pub enum NodeIter<T = Node> {
    Nodes(Nodes),
    Packages(Iter<Package>),
    Files(Iter<File>),
    Messages(Iter<Message>),
    Oneofs(Iter<Oneof>),
    Enums(Iter<Enum>),
    EnumValues(Iter<EnumValue>),
    Services(Iter<Service>),
    Methods(Iter<Method>),
    Fields(Iter<Field>),
    Extensions(Iter<Extension>),
    _Phantom(PhantomData<T>),
}
impl<'a, T> NodeIter<T> {
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
impl Iterator for NodeIter<Node> {
    type Item = Node;
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
impl From<Nodes> for NodeIter {
    fn from(i: Nodes) -> Self {
        NodeIter::Nodes(i)
    }
}

impl From<Iter<Message>> for NodeIter {
    fn from(i: Iter<Message>) -> Self {
        NodeIter::Messages(i)
    }
}
impl From<Iter<Oneof>> for NodeIter {
    fn from(i: Iter<Oneof>) -> Self {
        NodeIter::Oneofs(i)
    }
}
impl From<Iter<Enum>> for NodeIter {
    fn from(i: Iter<Enum>) -> Self {
        NodeIter::Enums(i)
    }
}
impl From<Iter<EnumValue>> for NodeIter {
    fn from(i: Iter<EnumValue>) -> Self {
        NodeIter::EnumValues(i)
    }
}
impl From<Iter<Service>> for NodeIter {
    fn from(i: Iter<Service>) -> Self {
        NodeIter::Services(i)
    }
}
impl From<Iter<Method>> for NodeIter {
    fn from(i: Iter<Method>) -> Self {
        NodeIter::Methods(i)
    }
}
impl From<Iter<Field>> for NodeIter {
    fn from(i: Iter<Field>) -> Self {
        NodeIter::Fields(i)
    }
}
impl From<Iter<Extension>> for NodeIter {
    fn from(i: Iter<Extension>) -> Self {
        NodeIter::Extensions(i)
    }
}
impl From<Iter<File>> for NodeIter {
    fn from(i: Iter<File>) -> Self {
        NodeIter::Files(i)
    }
}

#[derive(Debug, Clone)]
pub struct Nodes<T = Node> {
    _marker: PhantomData<T>,
    iters: VecDeque<NodeIter>,
}

impl Nodes {
    pub fn new(iters: Vec<NodeIter>) -> Nodes {
        Nodes {
            _marker: PhantomData,
            iters: iters.into(),
        }
    }
    pub fn empty() -> Nodes {
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
    pub fn push_back<I: Into<NodeIter>>(&mut self, i: I) {
        self.iters.push_back(i.into());
    }
    pub fn push_front<I: Into<NodeIter>>(&mut self, i: I) {
        self.iters.push_front(i.into());
    }
}

impl Iterator for Nodes<Node> {
    type Item = Node;
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

pub struct AllNodes<T = Node> {
    iter: Nodes,
    _marker: PhantomData<T>,
}

impl AllNodes<Node> {
    pub fn new(node: Node) -> AllNodes<Node> {
        AllNodes {
            iter: node.nodes(),
            _marker: PhantomData,
        }
    }
    pub fn push_back(&mut self, nodes: Nodes) {
        self.iter.push_back(nodes);
    }
}

impl Iterator for AllNodes {
    type Item = Node;

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
impl From<Nodes> for AllNodes {
    fn from(i: Nodes) -> Self {
        AllNodes {
            iter: i,
            _marker: PhantomData,
        }
    }
}

impl From<&Ast> for AllNodes {
    fn from(ast: &Ast) -> Self {
        AllNodes {
            iter: Nodes::new(vec![NodeIter::Packages(ast.packages())]),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use protobuf::reflect::FileDescriptor;

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
