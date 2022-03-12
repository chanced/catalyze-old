use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use anyhow::bail;

use crate::extension::WeakExtension;

use crate::iter::Iter;
use crate::Syntax;
use crate::{container::Container, container::WeakContainer, Name};
use crate::{
    AllEnums, Comments, Enum, Extension, Field, File, FullyQualified, Node, Nodes, Oneof, WeakFile,
    WellKnownMessage,
};
use crate::{DescriptorPath, MessageDescriptor};
use crate::{Package, WellKnownType};

use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
/// Message describes a proto message. Messages can be contained in either
/// another Message or a File, and may house further Messages and/or Enums. While
/// all Fields technically live on the Message, some may be contained within
/// Oneof blocks.
///
/// Fields within Oneof blocks fields will be accessible on both the Message and the Oneof.
pub struct Message<'a, U>(Rc<MessageDetail<'a, U>>);
impl<'a, U> Clone for Message<'a, U> {
    fn clone(&self) -> Self {
        Message(self.0.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MessageDetail<'a, U> {
    name: Name<U>,
    descriptor: MessageDescriptor<'a>,
    fqn: String,
    util: Rc<U>,
    messages: Rc<RefCell<Vec<Message<'a, U>>>>,
    enums: Rc<RefCell<Vec<Enum<'a, U>>>>,
    fields: Rc<RefCell<Vec<Field<'a, U>>>>,
    oneofs: Rc<RefCell<Vec<Oneof<'a, U>>>>,
    real_oneofs: Rc<RefCell<Vec<Oneof<'a, U>>>>,
    synthetic_oneofs: Rc<RefCell<Vec<Oneof<'a, U>>>>,
    dependents: Rc<RefCell<Vec<WeakMessage<'a, U>>>>,
    imports: Rc<RefCell<Vec<WeakFile<'a, U>>>>,
    import_set: Rc<RefCell<HashSet<String>>>,
    container: RefCell<WeakContainer<'a, U>>,
    maps: Rc<RefCell<Vec<Message<'a, U>>>>,
    /// `Extension`s defined by this message.
    defined_extensions: Rc<RefCell<Vec<Extension<'a, U>>>>,
    /// `Extension`s applied to this `Message`
    applied_extensions: Rc<RefCell<Vec<WeakExtension<'a, U>>>>,
    comments: RefCell<Comments<'a>>,
    wkt: Option<WellKnownMessage>,
}

impl<'a, U> Message<'a, U> {
    pub fn new(
        desc: MessageDescriptor<'a>,
        container: Container<'a, U>,
    ) -> Result<Self, anyhow::Error> {
        let util = container.util();
        let fqn = format!("{}.{}", container.fully_qualified_name(), desc.name());

        let wkt = if container.package().is_well_known_type() {
            WellKnownMessage::from_str(desc.name()).ok()
        } else {
            None
        };

        let msg = Message(Rc::new(MessageDetail {
            name: Name::new(desc.name(), util.clone()),
            container: RefCell::new(container.into()),
            fqn,
            descriptor: desc,
            util,
            wkt,
            enums: Rc::new(RefCell::new(Vec::with_capacity(desc.enums().len()))),
            fields: Rc::new(RefCell::new(Vec::with_capacity(desc.fields().len()))),
            oneofs: Rc::new(RefCell::new(Vec::with_capacity(desc.oneofs().len()))),
            real_oneofs: Rc::new(RefCell::new(Vec::new())),
            synthetic_oneofs: Rc::new(RefCell::new(Vec::new())),
            messages: Rc::new(RefCell::new(Vec::new())),
            maps: Rc::new(RefCell::new(Vec::new())),
            dependents: Rc::new(RefCell::new(Vec::new())),
            applied_extensions: Rc::new(RefCell::new(Vec::new())),
            defined_extensions: Rc::new(RefCell::new(Vec::with_capacity(desc.extensions().len()))),
            comments: RefCell::new(Comments::default()),
            imports: Rc::new(RefCell::new(Vec::new())),
            import_set: Rc::new(RefCell::new(HashSet::new())),
        }));

        {
            let container = Container::Message(msg.clone());
            let mut msgs = msg.0.messages.borrow_mut();
            let mut maps = msg.0.maps.borrow_mut();
            for md in desc.nested_messages() {
                let m = Message::new(md, container.clone())?;
                if m.is_map_entry() {
                    maps.push(m);
                } else {
                    msgs.push(m);
                }
            }
            let mut enums = msg.0.enums.borrow_mut();
            for ed in desc.enums() {
                let e = Enum::new(ed, container.clone());
                enums.push(e);
            }
            let mut oneofs = msg.0.oneofs.borrow_mut();
            let mut real_oneofs = msg.0.real_oneofs.borrow_mut();
            let mut synthetic_oneofs = msg.0.synthetic_oneofs.borrow_mut();
            for od in desc.oneofs() {
                let o = Oneof::new(od, msg.clone());
                oneofs.push(o.clone());
                if o.is_real() {
                    real_oneofs.push(o);
                } else {
                    synthetic_oneofs.push(o);
                }
            }
            let mut def_exts = msg.0.defined_extensions.borrow_mut();
            for xd in desc.extensions() {
                let ext = Extension::new(xd, container.clone());
                def_exts.push(ext);
            }

            let mut fields = msg.0.fields.borrow_mut();
            for fd in desc.fields() {
                let oneof = fd
                    .oneof_index()
                    .map(|i| oneofs.get(i as usize).expect("Oneof index out of bounds"))
                    .cloned();
                let f = Field::new(fd, msg.clone(), oneof.clone())?;
                if let Some(oneof) = oneof {
                    oneof.add_field(f.clone());
                }

                fields.push(f);
            }
        }
        Ok(msg)
    }
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.0.util.clone()
    }
    pub fn map_entries(&self) -> Iter<Message<'a, U>> {
        Iter::from(&self.0.maps)
    }
    pub fn dependents(&self) -> Dependents<'a, U> {
        self.0.dependents.clone().into()
    }
    pub fn build_target(&self) -> bool {
        self.0.container.borrow().build_target()
    }

    pub fn package(&self) -> Package<'a, U> {
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
    pub fn container(&self) -> Container<'a, U> {
        self.0.container.borrow().clone().into()
    }
    pub fn syntax(&self) -> Syntax {
        self.container().syntax()
    }
    pub fn file(&self) -> File<'a, U> {
        self.0.container.borrow().file()
    }

    pub fn fields(&self) -> Iter<Field<'a, U>> {
        Iter::from(&self.0.fields)
    }

    pub fn messages(&self) -> Iter<Self> {
        Iter::from(&self.0.messages)
    }
    pub fn oneofs(&self) -> Iter<Oneof<'a, U>> {
        Iter::from(&self.0.oneofs)
    }
    pub fn real_oneofs(&self) -> Iter<Oneof<'a, U>> {
        Iter::from(&self.0.real_oneofs)
    }
    pub fn synthetic_oneofs(&self) -> Iter<Oneof<'a, U>> {
        Iter::from(&self.0.synthetic_oneofs)
    }
    pub fn enums(&self) -> Iter<Enum<'a, U>> {
        Iter::from(&self.0.enums)
    }
    pub fn all_messages(&self) -> AllMessages<'a, U> {
        AllMessages::new(self.0.messages.clone())
    }

    pub fn all_enums(&self) -> AllEnums<'a, U> {
        AllEnums::new(self.0.enums.clone(), self.0.messages.clone())
    }

    // pub fn dependents(&self) -> UpgradeIter<Message<'a, U>, Into<Message<'a, U>>> {
    //     UpgradeIter::new(self.0.dependents.clone().borrow().into_iter())
    // }

    pub fn defined_extensions(&self) -> Iter<Extension<'a, U>> {
        Iter::from(&self.0.defined_extensions)
    }

    pub fn comments(&self) -> Comments<'a> {
        *self.0.comments.borrow()
    }

    pub fn nodes(&self) -> Nodes<'a, U> {
        Nodes::new(vec![
            self.enums().into(),
            self.messages().into(),
            self.fields().into(),
            self.oneofs().into(),
            self.defined_extensions().into(),
        ])
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.comments.replace(comments);
    }

    pub(crate) fn add_dependent(&self, dependent: Message<'a, U>) {
        self.0.dependents.borrow_mut().push(dependent.into());
    }

    pub(crate) fn add_applied_extension(&self, extension: Extension<'a, U>) {
        self.0
            .applied_extensions
            .borrow_mut()
            .push(extension.into());
    }

    pub(crate) fn register_import(&self, file: File<'a, U>) {
        let mut set = self.0.import_set.borrow_mut();
        if set.contains(&file.name().to_string()) {
            return;
        }
        self.container().register_import(file.clone());
        set.insert(file.name().to_string());
        self.0.imports.borrow_mut().push(file.into());
    }

    pub(crate) fn weak_file(&self) -> WeakFile<'a, U> {
        self.0.container.borrow().weak_file()
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        let msg = self.clone();
        if path.is_empty() {
            return Some(Node::Message(msg));
        }
        if path.len() % 2 == 0 {
            return None;
        }

        let next = path[1] as usize;
        DescriptorPath::try_from(path[0]).ok().and_then(|p| {
            let node: Option<Node<'a, U>> = match p {
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
                DescriptorPath::Extension => todo!(),
            };
            node.and_then(|n| n.node_at_path(&path[2..]))
        })
    }

    #[cfg(test)]
    pub fn add_node(&self, n: Node<'a, U>) {
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

impl<'a, U> FullyQualified for Message<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}

impl<'a, U> From<&WeakMessage<'a, U>> for Message<'a, U> {
    fn from(weak: &WeakMessage<'a, U>) -> Self {
        Message(weak.0.upgrade().unwrap())
    }
}

impl<'a, U> From<WeakMessage<'a, U>> for Message<'a, U> {
    fn from(weak: WeakMessage<'a, U>) -> Self {
        Message(weak.0.upgrade().unwrap())
    }
}

impl<'a, U> TryFrom<Container<'a, U>> for Message<'a, U> {
    type Error = anyhow::Error;
    fn try_from(container: Container<'a, U>) -> Result<Self, Self::Error> {
        match container {
            Container::Message(m) => Ok(m),
            _ => bail!("container is not a Message"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct WeakMessage<'a, U>(Weak<MessageDetail<'a, U>>);

impl<'a, U> WeakMessage<'a, U> {
    pub(crate) fn empty() -> WeakMessage<'a, U> {
        WeakMessage(Weak::new())
    }
    pub fn build_target(&self) -> bool {
        self.upgrade().build_target()
    }
    // pub fn container(&self) -> Container<'a, U> {
    //     self.upgrade().container()
    // }
    // pub fn name(&self) -> Name<U> {
    //     self.upgrade().name()
    // }
    pub fn package(&self) -> Package<'a, U> {
        self.upgrade().package()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.upgrade().fully_qualified_name()
    }
    fn upgrade(&self) -> Message<'a, U> {
        self.into()
    }
    pub fn file(&self) -> File<'a, U> {
        self.upgrade().file()
    }
    pub(crate) fn weak_file(&self) -> WeakFile<'a, U> {
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

impl<'a, U> From<&Message<'a, U>> for WeakMessage<'a, U> {
    fn from(m: &Message<'a, U>) -> Self {
        WeakMessage(Rc::downgrade(&m.0))
    }
}

impl<'a, U> From<Message<'a, U>> for WeakMessage<'a, U> {
    fn from(m: Message<'a, U>) -> Self {
        WeakMessage(Rc::downgrade(&m.0))
    }
}

impl<'a, U> Clone for WeakMessage<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Debug, Clone)]
pub struct AllMessages<'a, U> {
    q: VecDeque<Message<'a, U>>,
}
impl<'a, U> AllMessages<'a, U> {
    pub(crate) fn new(msgs: Rc<RefCell<Vec<Message<'a, U>>>>) -> Self {
        Self {
            q: VecDeque::from_iter(msgs.borrow().iter().cloned()),
        }
    }
}
impl<'a, U> Iterator for AllMessages<'a, U> {
    type Item = Message<'a, U>;
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
pub struct Dependents<'a, U, T = Message<'a, U>> {
    vec: Rc<RefCell<Vec<WeakMessage<'a, U>>>>,
    idx: usize,
    _marker: PhantomData<T>,
}

impl<'a, U> Iterator for Dependents<'a, U> {
    type Item = Message<'a, U>;
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

impl<'a, U> From<Rc<RefCell<Vec<WeakMessage<'a, U>>>>> for Dependents<'a, U> {
    fn from(vec: Rc<RefCell<Vec<WeakMessage<'a, U>>>>) -> Self {
        Self {
            vec,
            idx: 0,
            _marker: PhantomData,
        }
    }
}
