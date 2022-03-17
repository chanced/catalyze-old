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
    AllEnums, Comments, Enum, Extension, Field, File, Node, Nodes, Oneof, WeakFile,
    WellKnownMessage,
};
use crate::{DescriptorPath, MessageDescriptor};
use crate::{Package, WellKnownType};

use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Message describes a proto message. Messages can be contained in either
/// another Message or a File, and may house further Messages and/or Enums. While
/// all Fields technically live on the Message, some may be contained within
/// Oneof blocks.
///
/// Fields within Oneof blocks fields will be accessible on both the Message and the Oneof.
pub struct Message<'a>(Rc<MessageDetail<'a>>);

#[derive(Debug, Clone)]
pub(crate) struct MessageDetail<'a> {
    name: Name,
    descriptor: MessageDescriptor<'a>,
    fqn: String,

    messages: Rc<RefCell<Vec<Message<'a>>>>,
    enums: Rc<RefCell<Vec<Enum<'a>>>>,
    fields: Rc<RefCell<Vec<Field<'a>>>>,
    oneofs: Rc<RefCell<Vec<Oneof<'a>>>>,
    real_oneofs: Rc<RefCell<Vec<Oneof<'a>>>>,
    synthetic_oneofs: Rc<RefCell<Vec<Oneof<'a>>>>,
    dependents: Rc<RefCell<Vec<WeakMessage<'a>>>>,
    imports: Rc<RefCell<Vec<WeakFile<'a>>>>,
    import_set: Rc<RefCell<HashSet<String>>>,
    container: RefCell<WeakContainer<'a>>,
    maps: Rc<RefCell<Vec<Message<'a>>>>,
    /// `Extension`s defined by this message.
    defined_extensions: Rc<RefCell<Vec<Extension<'a>>>>,
    /// `Extension`s applied to this `Message`
    applied_extensions: Rc<RefCell<Vec<WeakExtension<'a>>>>,
    comments: RefCell<Comments<'a>>,
    wkt: Option<WellKnownMessage>,
}

impl<'a> Message<'a> {
    pub fn new(
        desc: MessageDescriptor<'a>,
        container: Container<'a>,
    ) -> Result<Self, anyhow::Error> {
        let fqn = format!("{}.{}", container.fully_qualified_name(), desc.name());

        let wkt = if container.package().is_well_known_type() {
            WellKnownMessage::from_str(desc.name()).ok()
        } else {
            None
        };

        let msg = Message(Rc::new(MessageDetail {
            name: desc.name().into(),
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
    pub fn name(&self) -> &Name {
        &self.0.name
    }
    pub fn map_entries(&self) -> Iter<Message<'a>> {
        Iter::from(&self.0.maps)
    }
    pub fn dependents(&self) -> Dependents<'a> {
        self.0.dependents.clone().into()
    }
    pub fn build_target(&self) -> bool {
        self.0.container.borrow().build_target()
    }

    pub fn package(&self) -> Package<'a> {
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
    pub fn container(&self) -> Container<'a> {
        self.0.container.borrow().clone().into()
    }
    pub fn syntax(&self) -> Syntax {
        self.container().syntax()
    }
    pub fn file(&self) -> File<'a> {
        self.0.container.borrow().file()
    }

    pub fn fields(&self) -> Iter<Field<'a>> {
        Iter::from(&self.0.fields)
    }

    pub fn messages(&self) -> Iter<Self> {
        Iter::from(&self.0.messages)
    }
    pub fn oneofs(&self) -> Iter<Oneof<'a>> {
        Iter::from(&self.0.oneofs)
    }
    pub fn real_oneofs(&self) -> Iter<Oneof<'a>> {
        Iter::from(&self.0.real_oneofs)
    }
    pub fn synthetic_oneofs(&self) -> Iter<Oneof<'a>> {
        Iter::from(&self.0.synthetic_oneofs)
    }
    pub fn enums(&self) -> Iter<Enum<'a>> {
        Iter::from(&self.0.enums)
    }
    pub fn all_messages(&self) -> AllMessages<'a> {
        AllMessages::new(self.0.messages.clone())
    }

    pub fn all_enums(&self) -> AllEnums<'a> {
        AllEnums::new(self.0.enums.clone(), self.0.messages.clone())
    }

    // pub fn dependents(&self) -> UpgradeIter<Message<'a>, Into<Message<'a>>> {
    //     UpgradeIter::new(self.0.dependents.clone().borrow().into_iter())
    // }

    pub fn defined_extensions(&self) -> Iter<Extension<'a>> {
        Iter::from(&self.0.defined_extensions)
    }

    pub fn comments(&self) -> Comments<'a> {
        *self.0.comments.borrow()
    }

    pub fn nodes(&self) -> Nodes<'a> {
        Nodes::new(vec![
            self.enums().into(),
            self.messages().into(),
            self.fields().into(),
            self.oneofs().into(),
            self.defined_extensions().into(),
        ])
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.comments.replace(comments);
    }

    pub(crate) fn add_dependent(&self, dependent: Message<'a>) {
        self.0.dependents.borrow_mut().push(dependent.into());
    }

    pub(crate) fn add_applied_extension(&self, extension: Extension<'a>) {
        self.0
            .applied_extensions
            .borrow_mut()
            .push(extension.into());
    }

    pub(crate) fn register_import(&self, file: File<'a>) {
        let mut set = self.0.import_set.borrow_mut();
        if set.contains(&file.name().to_string()) {
            return;
        }
        self.container().register_import(file.clone());
        set.insert(file.name().to_string());
        self.0.imports.borrow_mut().push(file.into());
    }

    pub(crate) fn weak_file(&self) -> WeakFile<'a> {
        self.0.container.borrow().weak_file()
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a>> {
        let msg = self.clone();
        if path.is_empty() {
            return Some(Node::Message(msg));
        }
        if path.len() % 2 == 0 {
            return None;
        }

        let next = path[1] as usize;
        DescriptorPath::try_from(path[0]).ok().and_then(|p| {
            let node: Option<Node<'a>> = match p {
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
    pub fn add_node(&self, n: Node<'a>) {
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

impl<'a> From<&WeakMessage<'a>> for Message<'a> {
    fn from(weak: &WeakMessage<'a>) -> Self {
        Message(weak.0.upgrade().unwrap())
    }
}

impl<'a> From<WeakMessage<'a>> for Message<'a> {
    fn from(weak: WeakMessage<'a>) -> Self {
        Message(weak.0.upgrade().unwrap())
    }
}

impl<'a> TryFrom<Container<'a>> for Message<'a> {
    type Error = anyhow::Error;
    fn try_from(container: Container<'a>) -> Result<Self, Self::Error> {
        match container {
            Container::Message(m) => Ok(m),
            _ => bail!("container is not a Message"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WeakMessage<'a>(Weak<MessageDetail<'a>>);

impl<'a> WeakMessage<'a> {
    pub(crate) fn empty() -> WeakMessage<'a> {
        WeakMessage(Weak::new())
    }
    pub fn build_target(&self) -> bool {
        self.upgrade().build_target()
    }
    // pub fn container(&self) -> Container<'a> {
    //     self.upgrade().container()
    // }
    // pub fn name(&self) -> &Name {
    //     self.upgrade().name()
    // }
    pub fn package(&self) -> Package<'a> {
        self.upgrade().package()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.upgrade().fully_qualified_name()
    }
    fn upgrade(&self) -> Message<'a> {
        self.into()
    }
    pub fn file(&self) -> File<'a> {
        self.upgrade().file()
    }
    pub(crate) fn weak_file(&self) -> WeakFile<'a> {
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

impl<'a> From<&Message<'a>> for WeakMessage<'a> {
    fn from(m: &Message<'a>) -> Self {
        WeakMessage(Rc::downgrade(&m.0))
    }
}

impl<'a> From<Message<'a>> for WeakMessage<'a> {
    fn from(m: Message<'a>) -> Self {
        WeakMessage(Rc::downgrade(&m.0))
    }
}

#[derive(Debug, Clone)]
pub struct AllMessages<'a> {
    q: VecDeque<Message<'a>>,
}
impl<'a> AllMessages<'a> {
    pub(crate) fn new(msgs: Rc<RefCell<Vec<Message<'a>>>>) -> Self {
        Self {
            q: VecDeque::from_iter(msgs.borrow().iter().cloned()),
        }
    }
}
impl<'a> Iterator for AllMessages<'a> {
    type Item = Message<'a>;
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
pub struct Dependents<'a, T = Message<'a>> {
    vec: Rc<RefCell<Vec<WeakMessage<'a>>>>,
    idx: usize,
    _marker: PhantomData<T>,
}

impl<'a> Iterator for Dependents<'a> {
    type Item = Message<'a>;
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

impl<'a> From<Rc<RefCell<Vec<WeakMessage<'a>>>>> for Dependents<'a> {
    fn from(vec: Rc<RefCell<Vec<WeakMessage<'a>>>>) -> Self {
        Self {
            vec,
            idx: 0,
            _marker: PhantomData,
        }
    }
}
