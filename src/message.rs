use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::extension::WeakExtension;
use crate::field::FieldList;
use crate::iter::{AllEnums, AllMessages, Iter};
use crate::proto::{DescriptorPath, MessageDescriptor};
use crate::{container::Container, container::WeakContainer, Name};
use crate::{
    format_fqn, Enum, EnumList, Extension, Field, FullyQualified, Node, NodeAtPath, Oneof,
    OneofList,
};
use crate::{Package, WellKnownType};

pub(crate) type MessageList<'a, U> = Rc<RefCell<Vec<Message<'a, U>>>>;

#[derive(Debug)]
/// Message describes a proto message. Messages can be contained in either
/// another Message or a File, and may house further Messages and/or Enums. While
/// all Fields technically live on the Message, some may be contained within
/// OneOf blocks.
pub struct Message<'a, U>(Rc<MessageDetail<'a, U>>);
impl<'a, U> Clone for Message<'a, U> {
    fn clone(&self) -> Self {
        Message(self.0.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MessageDetail<'a, U> {
    name: Name<U>,
    is_map_entry: bool,
    descriptor: MessageDescriptor<'a, U>,
    well_known_type: Option<WellKnownType>,
    fqn: String,
    util: RefCell<Rc<U>>,
    messages: MessageList<'a, U>,
    enums: EnumList<'a, U>,
    fields: FieldList<'a, U>,
    oneofs: OneofList<'a, U>,
    dependents: Rc<RefCell<Vec<WeakMessage<'a, U>>>>,
    container: WeakContainer<'a, U>,
    maps: MessageList<'a, U>,
    preserved_messages: MessageList<'a, U>,
    /// `Extension`s defined by this message.
    defined_extensions: Rc<RefCell<Vec<Extension<'a, U>>>>,
    /// `Extension`s applied to this `Message`
    applied_extensions: Rc<RefCell<Vec<WeakExtension<'a, U>>>>,
}

impl<'a, U> Message<'a, U> {
    pub(crate) fn new(desc: MessageDescriptor<'a, U>, container: Container<'a, U>) -> Self {
        let util = container.util();
        let fqn = format_fqn(&container, desc.name());
        // let well_known_type = if container.package().is_well_known() {
        //     match WellKnownType::from_str(&fqn) {
        //         Ok(wkt) => Some(wkt),
        //         Err(_) => None,
        //     }
        // } else {
        //     None
        // };

        // TODO: Fix this
        let well_known_type = None;

        let msg = Message(Rc::new(MessageDetail {
            name: Name::new(desc.name(), util.clone()),
            container: container.into(),
            fqn,
            well_known_type,
            descriptor: desc,
            util: RefCell::new(util.clone()),

            is_map_entry: desc.options().map_entry(),
            enums: Rc::new(RefCell::new(Vec::with_capacity(desc.enums().len()))),
            fields: Rc::new(RefCell::new(Vec::with_capacity(desc.fields().len()))),
            oneofs: Rc::new(RefCell::new(Vec::with_capacity(desc.oneofs().len()))),
            preserved_messages: Rc::new(RefCell::new(Vec::with_capacity(
                desc.nested_messages().len(),
            ))),
            messages: Rc::new(RefCell::new(Vec::new())),
            maps: Rc::new(RefCell::new(Vec::new())),
            dependents: Rc::new(RefCell::new(Vec::new())),
            applied_extensions: Rc::new(RefCell::new(Vec::new())),
            defined_extensions: Rc::new(RefCell::new(Vec::with_capacity(desc.extensions().len()))),
        }));

        let container = Container::Message(msg.clone());
        {
            let mut msgs = msg.0.messages.borrow_mut();
            for md in desc.nested_messages() {
                let msg = Message::new(md, container.clone());
                msgs.push(msg);
            }
        }
        {
            let mut enums = msg.0.enums.borrow_mut();
            for ed in desc.enums() {
                let e = Enum::new(ed, container.clone());
                enums.push(e);
            }
        }
        {
            let mut oneofs = msg.0.oneofs.borrow_mut();
            for od in desc.oneofs() {
                let o = Oneof::new(od, container.clone());
                oneofs.push(o);
            }
        }
        {
            let mut def_exts = msg.0.defined_extensions.borrow_mut();
            for xd in desc.extensions() {
                let ext = Extension::new(xd, container.clone());
                def_exts.push(ext);
            }
        }
        msg
    }
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.0.util.borrow().clone()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.0.util.replace(util);
    }
    pub fn build_target(&self) -> bool {
        self.0.container.build_target()
    }

    pub fn package(&self) -> Package<'a, U> {
        self.0.container.package()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.0.well_known_type.is_some()
    }
    pub fn container(&self) -> Container<'a, U> {
        todo!()
        // self.0.container.upgrade()
    }

    pub fn fields(&self) -> Iter<Field<'a, U>> {
        todo!()
        // Iter::from(&self.0.fields)
    }

    pub fn messages(&self) -> Iter<Self> {
        todo!();
        // Iter::from(&self.0.messages)
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

    // pub fn dependents(&self) -> UpgradeIter<Message<'a, U>, dyn Into<Message<'a, U>>> {
    //     UpgradeIter::new(self.0.dependents.clone().borrow().into_iter())
    // }

    pub fn defined_extensions(&self) -> Iter<Extension<'a, U>> {
        Iter::from(&self.0.defined_extensions)
    }

    pub(crate) fn add_field(&self, field: Field<'a, U>) {
        self.0.fields.borrow_mut().push(field);
    }

    pub(crate) fn add_dependent(&self, dependent: Message<'a, U>) {
        self.0.dependents.borrow_mut().push(dependent.into());
    }
}

impl<'a, U> FullyQualified for Message<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}

impl<'a, U> NodeAtPath<'a, U> for Message<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
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
            };
            node.and_then(|n| n.node_at_path(&path[2..]))
        })
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

#[derive(Debug)]
pub(crate) struct WeakMessage<'a, U>(Weak<MessageDetail<'a, U>>);

impl<'a, U> WeakMessage<'a, U> {
    pub fn build_target(&self) -> bool {
        self.upgrade().build_target()
    }
    pub fn container(&self) -> Container<'a, U> {
        self.upgrade().container()
    }
    pub fn name(&self) -> Name<U> {
        self.upgrade().name()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.upgrade().package()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.upgrade().fully_qualified_name()
    }
    fn upgrade(&self) -> Message<'a, U> {
        self.into()
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
