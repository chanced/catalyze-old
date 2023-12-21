use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use protobuf::reflect::MessageDescriptor;

use crate::comments::Comments;
use crate::enum_::{AllEnums, Enum};
use crate::error::Error;
use crate::extension::{Extension, WeakExtension};

use crate::field::Field;
use crate::file::{File, Syntax, WeakFile};
use crate::iter::Iter;
use crate::node::{Container, Node, Nodes, WeakContainer};
use crate::oneof::Oneof;
use crate::package::Package;
use crate::uninterpreted_option::UninterpretedOption;
use crate::well_known::{WellKnownMessage, WellKnownType};
use crate::DescriptorPath;

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub(crate) struct Detail {
    descriptor: MessageDescriptor,
    fqn: String,
    messages: RefCell<Rc<[Message]>>,
    enums: RefCell<Rc<[Enum]>>,
    fields: RefCell<Rc<[Field]>>,
    oneofs: RefCell<Rc<[Oneof]>>,
    real_oneofs: RefCell<Rc<[Oneof]>>,
    synthetic_oneofs: RefCell<Rc<[Oneof]>>,
    dependents: RefCell<Rc<[WeakMessage]>>,
    imports: RefCell<Rc<[WeakFile]>>,
    import_set: RefCell<HashSet<String>>,
    container: RefCell<WeakContainer>,
    maps: RefCell<HashMap<String, Message>>,
    /// `Extension`s defined by this message.
    defined_extensions: RefCell<Rc<[Extension]>>,
    /// `Extension`s applied to this `Message`
    applied_extensions: Rc<RefCell<Rc<[WeakExtension]>>>,
    comments: RefCell<Comments>,
    wkt: Option<WellKnownMessage>,
}

impl From<Option<&protobuf::descriptor::MessageOptions>> for Options {
    fn from(opts: Option<&protobuf::descriptor::MessageOptions>) -> Self {
        // MessageOptions { opts }
        todo!()
    }
}

impl Detail {
    fn new(desc: MessageDescriptor, container: Container) -> Rc<Self> {
        let fqn = format!("{}.{}", container.fully_qualified_name(), desc.name());

        let wkt = if container.package().is_well_known_type() {
            WellKnownMessage::from_str(desc.name()).ok()
        } else {
            None
        };
        Rc::new(Self {
            container: RefCell::new(container.into()),
            fqn,
            descriptor: desc,
            wkt,
            enums: Rc::new(RefCell::new(Vec::with_capacity(desc.enums().len()))),
            fields: Rc::new(RefCell::new(Vec::with_capacity(desc.fields().len()))),
            oneofs: Rc::new(RefCell::new(Vec::with_capacity(desc.oneofs().len()))),
            real_oneofs: Rc::new(RefCell::new(Vec::new())),
            synthetic_oneofs: Rc::new(RefCell::new(Vec::new())),
            messages: Rc::new(RefCell::new(Vec::new())),
            maps: Rc::new(RefCell::new(HashMap::new())),
            dependents: Rc::new(RefCell::new(Vec::new())),
            applied_extensions: Rc::new(RefCell::new(Vec::new())),
            defined_extensions: Rc::new(RefCell::new(Vec::with_capacity(desc.extensions().len()))),
            comments: RefCell::new(Comments::default()),
            imports: Rc::new(RefCell::new(Vec::new())),
            import_set: Rc::new(RefCell::new(HashSet::new())),
        })
    }
}

#[derive(Debug, Clone)]
/// Message describes a proto message. Messages can be contained in either
/// another Message or a File, and may house further Messages and/or Enums. While
/// all Fields technically live on the Message, some may be contained within
/// Oneof blocks.
///
/// Fields within Oneof blocks fields will be accessible on both the Message and the Oneof.
pub struct Message(Rc<Detail>);

impl Message {
    pub fn new(desc: MessageDescriptor, container: Container) -> Result<Self, Error> {
        let msg = Self(Detail::new(desc, container))
            .hydrate_nested_msgs()?
            .hydrate_enums()
            .hydrate_exts()
            .hydrate_oneofs()
            .hydrate_fields()?;
        Ok(msg)
    }

    pub fn as_container(&self) -> Container {
        self.into()
    }
    pub fn descriptor(&self) -> MessageDescriptor {
        self.0.descriptor
    }
    pub fn name(&self) -> &str {
        self.0.descriptor.name()
    }

    pub fn dependents(&self) -> Dependents {
        self.0.dependents.clone().into()
    }
    pub fn build_target(&self) -> bool {
        self.0.container.borrow().build_target()
    }

    pub fn package(&self) -> Package {
        self.0.container.borrow().package()
    }
    pub fn is_map_entry(&self) -> bool {
        self.0.descriptor.options().is_map_entry()
    }
    pub fn is_well_known_type(&self) -> bool {
        self.0.wkt.is_some()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.0.wkt.map(Into::into)
    }
    pub fn well_known_message(&self) -> Option<WellKnownMessage> {
        self.0.wkt
    }
    pub fn container(&self) -> Container {
        self.0.container.borrow().clone().into()
    }
    pub fn syntax(&self) -> Syntax {
        self.container().syntax()
    }
    pub fn file(&self) -> File {
        self.0.container.borrow().file()
    }
    pub(crate) fn maps(&self) -> HashMap<&str, Message> {
        self.0
            .maps
            .borrow()
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    }
    pub fn fields(&self) -> Iter<Field> {
        Iter::from(&self.0.fields)
    }

    pub fn messages(&self) -> Iter<Self> {
        Iter::from(&self.0.messages)
    }
    pub fn message(&self, name: &str) -> Option<Message> {
        self.embed(name)
    }
    pub fn embed(&self, name: &str) -> Option<Message> {
        self.messages().find(|msg| msg.name() == name)
    }
    pub fn enum_(&self, name: &str) -> Option<Enum> {
        self.enums().find(|enum_| enum_.name() == name)
    }
    pub fn oneofs(&self) -> Iter<Oneof> {
        Iter::from(&self.0.oneofs)
    }
    pub fn real_oneofs(&self) -> Iter<Oneof> {
        Iter::from(&self.0.real_oneofs)
    }
    pub fn synthetic_oneofs(&self) -> Iter<Oneof> {
        Iter::from(&self.0.synthetic_oneofs)
    }
    pub fn enums(&self) -> Iter<Enum> {
        Iter::from(&self.0.enums)
    }
    pub fn all_messages(&self) -> AllMessages {
        AllMessages::new(self.0.messages.clone())
    }

    pub fn all_enums(&self) -> AllEnums {
        AllEnums::new(self.0.enums.clone(), self.0.messages.clone())
    }

    // pub fn dependents(&self) -> UpgradeIter<Message, Into<Message>> {
    //     UpgradeIter::new(self.0.dependents.clone().borrow().into_iter())
    // }

    pub fn defined_extensions(&self) -> Iter<Extension> {
        Iter::from(&self.0.defined_extensions)
    }

    pub fn comments(&self) -> Comments {
        *self.0.comments.borrow()
    }
    pub fn field(&self, name: &str) -> Option<Field> {
        self.0
            .fields
            .borrow()
            .iter()
            .find(|f| f.name() == name)
            .cloned()
    }
    pub fn nodes(&self) -> Nodes {
        Nodes::new(vec![
            self.enums().into(),
            self.messages().into(),
            self.fields().into(),
            self.oneofs().into(),
            self.defined_extensions().into(),
        ])
    }
    pub fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
    }
    pub(crate) fn set_comments(&self, comments: Comments) {
        self.0.comments.replace(comments);
    }

    pub(crate) fn add_dependent(&self, dependent: Message) {
        self.0.dependents.borrow_mut().push(dependent.into());
    }

    pub(crate) fn add_applied_extension(&self, extension: Extension) {
        self.0
            .applied_extensions
            .borrow_mut()
            .push(extension.into());
    }

    pub(crate) fn register_import(&self, file: File) {
        let mut set = self.0.import_set.borrow_mut();
        if set.contains(&*file.name()) {
            return;
        }
        self.container().register_import(file.clone());
        set.insert(file.name().to_string());
        self.0.imports.borrow_mut().push(file.into());
    }

    pub(crate) fn weak_file(&self) -> WeakFile {
        self.0.container.borrow().weak_file()
    }

    pub(crate) fn replace_field(&self, idx: usize, field: Field) {
        self.0.fields.borrow_mut()[idx] = field;
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node> {
        let msg = self.clone();
        if path.is_empty() {
            return Some(Node::Message(msg));
        }
        if path.len() % 2 == 0 {
            return None;
        }

        let next = path[1] as usize;
        DescriptorPath::try_from(path[0]).ok().and_then(|p| {
            let node: Option<Node> = match p {
                DescriptorPath::EnumType => msg
                    .0
                    .enums
                    .borrow()
                    .get(next)
                    .map(|e| Node::Enum(e.clone())),
                DescriptorPath::Field => msg
                    .0
                    .fields
                    .borrow()
                    .get(next)
                    .map(|f| Node::Field(f.clone())),
                DescriptorPath::OneofDecl => msg
                    .0
                    .oneofs
                    .borrow()
                    .get(next)
                    .map(|o| Node::Oneof(o.clone())),
                DescriptorPath::NestedType => msg
                    .0
                    .messages
                    .borrow()
                    .get(next)
                    .map(|m| Node::Message(m.clone())),
                DescriptorPath::Extension => msg
                    .0
                    .applied_extensions
                    .borrow()
                    .get(next)
                    .map(|m| Node::Extension(m.upgrade())),
            };
            node.and_then(|n| n.node_at_path(&path[2..]))
        })
    }

    fn hydrate_fields(self) -> Result<Message, Error> {
        {
            let oneofs = self.0.oneofs.borrow_mut();
            let mut fields = self.0.fields.borrow_mut();
            for fd in self.descriptor().fields() {
                let oneof = fd
                    .oneof_index()
                    .map(|i| oneofs.get(i as usize).expect("Oneof index out of bounds"))
                    .cloned();
                let f = Field::new(fd, self.clone(), oneof.clone())?;
                if let Some(oneof) = oneof {
                    oneof.add_field(f.clone());
                }

                fields.push(f);
            }
        }
        Ok(self)
    }

    fn hydrate_exts(self) -> Self {
        {
            let container = self.as_container();
            let mut def_exts = self.0.defined_extensions.borrow_mut();
            for xd in self.descriptor().extensions() {
                let ext = Extension::new(xd, container.clone());
                def_exts.push(ext);
            }
        }
        self
    }

    fn hydrate_oneofs(self) -> Self {
        {
            let mut oneofs = self.0.oneofs.borrow_mut();
            let mut real_oneofs = self.0.real_oneofs.borrow_mut();
            let mut synthetic_oneofs = self.0.synthetic_oneofs.borrow_mut();
            for od in self.descriptor().oneofs() {
                let o = Oneof::new(od, self.clone());
                oneofs.push(o.clone());
                if o.is_real() {
                    real_oneofs.push(o);
                } else {
                    synthetic_oneofs.push(o);
                }
            }
        }
        self
    }
    fn hydrate_enums(self) -> Self {
        {
            let container = self.as_container();
            let mut enums = self.0.enums.borrow_mut();
            for ed in self.descriptor().enums() {
                let e = Enum::new(ed, container.clone());
                enums.push(e);
            }
        }
        self
    }

    fn hydrate_nested_msgs(self) -> Result<Self, Error> {
        {
            let container = self.as_container();
            let mut msgs = self.0.messages.borrow_mut();
            let mut maps = self.0.maps.borrow_mut();
            for md in self.descriptor().nested_messages() {
                let m = Message::new(md, container.clone())?;
                if m.is_map_entry() {
                    maps.insert(m.fully_qualified_name().to_string(), m);
                } else {
                    msgs.push(m);
                }
            }
        }
        Ok(self)
    }

    #[cfg(test)]
    pub fn add_node(&self, n: Node) {
        match n {
            Node::Message(m) => self.0.messages.borrow_mut().push(m),
            Node::Enum(e) => self.0.enums.borrow_mut().push(e),
            Node::Oneof(o) => self.0.oneofs.borrow_mut().push(o),
            Node::Field(f) => self.0.fields.borrow_mut().push(f),
            Node::Extension(e) => self.0.defined_extensions.borrow_mut().push(e),
            _ => panic!("unexpected node type"),
        }
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.fully_qualified_name() == other.fully_qualified_name()
    }
}

impl From<&WeakMessage> for Message {
    fn from(weak: &WeakMessage) -> Self {
        Message(weak.0.upgrade().expect("message was dropped"))
    }
}

impl From<WeakMessage> for Message {
    fn from(weak: WeakMessage) -> Self {
        Message(weak.0.upgrade().expect("Message was dropped"))
    }
}

impl TryFrom<Container> for Message {
    type Error = Container;
    fn try_from(container: Container) -> Result<Self, Self::Error> {
        match container {
            Container::Message(m) => Ok(m),
            _ => Err(container),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WeakMessage(Weak<Detail>);

impl WeakMessage {
    pub(crate) fn new() -> WeakMessage {
        WeakMessage(Weak::new())
    }
    pub fn build_target(&self) -> bool {
        self.upgrade().build_target()
    }
    // pub fn container(&self) -> Container {
    //     self.upgrade().container()
    // }
    // pub fn name(&self) -> &Name {
    //     self.upgrade().name()
    // }
    pub fn package(&self) -> Package {
        self.upgrade().package()
    }
    fn upgrade(&self) -> Message {
        self.into()
    }
    pub fn file(&self) -> File {
        self.upgrade().file()
    }
    pub(crate) fn weak_file(&self) -> WeakFile {
        self.upgrade().weak_file()
    }
    pub fn is_well_known_type(&self) -> bool {
        self.upgrade().is_well_known_type()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.upgrade().well_known_type()
    }
    pub fn well_known_message(&self) -> Option<WellKnownMessage> {
        self.upgrade().well_known_message()
    }
}

impl From<&Message> for WeakMessage {
    fn from(m: &Message) -> Self {
        WeakMessage(Rc::downgrade(&m.0))
    }
}

impl From<Message> for WeakMessage {
    fn from(m: Message) -> Self {
        WeakMessage(Rc::downgrade(&m.0))
    }
}

#[derive(Debug, Clone)]
pub struct AllMessages {
    q: VecDeque<Message>,
}
impl AllMessages {
    pub(crate) fn new(msgs: Rc<RefCell<Vec<Message>>>) -> Self {
        Self {
            q: VecDeque::from_iter(msgs.borrow().iter().cloned()),
        }
    }
}
impl Iterator for AllMessages {
    type Item = Message;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(msg) = self.q.pop_front() {
            for v in msg.messages() {
                self.q.push_back(v);
            }
            Some(msg)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dependents<T = Message> {
    vec: RefCell<Vec<WeakMessage>>,
    idx: usize,
    _marker: PhantomData<T>,
}

impl Iterator for Dependents {
    type Item = Message;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.vec.borrow().len() {
            let msg = self.vec.borrow()[self.idx].upgrade();
            self.idx += 1;
            Some(msg)
        } else {
            None
        }
    }
}

impl From<Rc<RefCell<Vec<WeakMessage>>>> for Dependents {
    fn from(vec: Rc<RefCell<Vec<WeakMessage>>>) -> Self {
        Self {
            vec,
            idx: 0,
            _marker: PhantomData,
        }
    }
}
impl From<WeakMessage> for Option<Message> {
    fn from(msg: WeakMessage) -> Self {
        msg.0.upgrade().map(Message)
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    // message fields
    pub message_set_wire_format: Option<bool>,
    ///  Disables the generation of the standard "descriptor()" accessor, which can
    ///  conflict with a field of the same name.  This is meant to make migration
    ///  from proto1 easier; new code should avoid fields named "descriptor".
    pub no_standard_descriptor_accessor: Option<bool>,
    ///  Is this message deprecated?
    ///  Depending on the target platform, this can emit Deprecated annotations
    ///  for the message, or it will be completely ignored; in the very least,
    ///  this is a formalization for deprecating messages.
    pub deprecated: Option<bool>,
    pub map_entry: Option<bool>,
    ///  The parser stores options it doesn't recognize here.
    pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
    pub special_fields: protobuf::SpecialFields,
}
impl Options {
    /// Set true to use the old proto1 MessageSet wire format for extensions.
    /// This is provided for backwards-compatibility with the MessageSet wire
    /// format.  You should not use this for any other reason:  It's less
    /// efficient, has fewer features, and is more complicated.
    ///
    /// The message must be defined exactly as follows:
    ///   message Foo {
    ///     option message_set_wire_format = true;
    ///     extensions 4 to max;
    ///   }
    /// Note that the message cannot have any defined fields; MessageSets only
    /// have extensions.
    ///
    /// All extensions of your type must be singular messages; e.g. they cannot
    /// be int32s, enums, or repeated messages.
    ///
    /// Because this is an option, the above two restrictions are not enforced by
    /// the protocol compiler.
    pub fn message_set_wire_format(&self) -> bool {
        self.opts().message_set_wire_format()
    }
    /// Whether the message is an automatically generated map entry type for the
    /// maps field.
    ///
    /// For maps fields:
    ///     map<KeyType, ValueType> map_field = 1;
    /// The parsed descriptor looks like:
    ///     message MapFieldEntry {
    ///         option map_entry = true;
    ///         optional KeyType key = 1;
    ///         optional ValueType value = 2;
    ///     }
    ///     repeated MapFieldEntry map_field = 1;
    ///
    /// Implementations may choose not to generate the map_entry=true message, but
    /// use a native map in the target language to hold the keys and values.
    /// The reflection APIs in such implementations still need to work as
    /// if the field is a repeated message field.
    ///
    /// NOTE: Do not set the option in .proto files. Always use the maps syntax
    /// instead. The option should only be implicitly set by the proto compiler
    /// parser.
    pub fn map_entry(&self) -> bool {
        self.opts().map_entry()
    }

    pub fn is_map_entry(&self) -> bool {
        self.map_entry()
    }

    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn no_standard_descriptor_accessor(&self) -> bool {
        self.opts().no_standard_descriptor_accessor()
    }
    /// The parser stores options it doesn't recognize here. See above.
    pub fn uninterpreted_option(&self) -> &[UninterpretedOption] {
        (&self.opts().uninterpreted_option).into()
    }
}
