use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use prost_types::DescriptorProto;

use crate::container::BuildTarget;
use crate::field::FieldList;
use crate::iter::{AllEnums, AllMessages, Iter, UpgradeIter};
use crate::name::Named;
use crate::path::DescriptorPath;
use crate::traits::{Downgrade, Upgrade};
use crate::{container::Container, container::WeakContainer, Name};
use crate::{
    format_fqn, Enum, EnumList, Extension, Field, FullyQualified, Node, NodeAtPath, Oneof,
    OneofList, WeakExtension,
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
    descriptor: &'a DescriptorProto,
    well_known_type: Option<WellKnownType>,
    fqn: String,
    util: Rc<RefCell<U>>,
    messages: MessageList<'a, U>,
    enums: EnumList<'a, U>,
    fields: FieldList<'a, U>,
    oneofs: OneofList<'a, U>,
    dependents: Rc<RefCell<Vec<Weak<Message<'a, U>>>>>,
    container: WeakContainer<'a, U>,
    maps: MessageList<'a, U>,
    preserved_messages: MessageList<'a, U>,
    /// `Extension`s defined by this message.
    defined_extensions: Rc<RefCell<Vec<Extension<'a, U>>>>,
    /// `Extension`s applied to this `Message`
    applied_extensions: Rc<RefCell<Vec<WeakExtension<'a, U>>>>,
}

impl<'a, U> Message<'a, U> {
    pub(crate) fn new(
        descriptor: &'a DescriptorProto,
        container: Container<'a, U>,
        util: Rc<RefCell<U>>,
    ) -> Self {
        let fqn = format_fqn(&container, descriptor.name());
        let well_known_type = if container.package().map_or(false, |pkg| pkg.is_well_known()) {
            match WellKnownType::from_str(&fqn) {
                Ok(wkt) => Some(wkt),
                Err(_) => None,
            }
        } else {
            None
        };
        let msg = Message(Rc::new(MessageDetail {
            name: Name::new(descriptor.name(), util.clone()),
            container: container.downgrade(),
            fqn,
            well_known_type,
            descriptor,
            util: util.clone(),
            is_map_entry: descriptor.options.as_ref().map_or(false, |o| o.map_entry()),
            enums: Rc::new(RefCell::new(Vec::with_capacity(descriptor.enum_type.len()))),
            fields: Rc::new(RefCell::new(Vec::with_capacity(descriptor.field.len()))),
            oneofs: Rc::new(RefCell::new(Vec::with_capacity(
                descriptor.oneof_decl.len(),
            ))),
            preserved_messages: Rc::new(RefCell::new(Vec::with_capacity(
                descriptor.nested_type.len(),
            ))),
            messages: Rc::new(RefCell::new(Vec::new())),
            maps: Rc::new(RefCell::new(Vec::new())),
            dependents: Rc::new(RefCell::new(Vec::new())),
            applied_extensions: Rc::new(RefCell::new(Vec::new())),
            defined_extensions: Rc::new(RefCell::new(Vec::with_capacity(
                descriptor.extension.len(),
            ))),
        }));

        let container = Container::Message(msg.clone());
        let mut msgs = msg.0.messages.borrow_mut();
        for md in msg.0.descriptor.nested_type.iter() {
            let msg = Message::new(md, container.clone(), util.clone());
            msgs.push(msg);
        }
        let mut enums = msg.0.enums.borrow_mut();
        for ed in descriptor.enum_type.iter() {
            let e = Enum::new(ed, container.clone(), util.clone());
            enums.push(e);
        }
        let mut oneofs = msg.0.oneofs.borrow_mut();
        for od in msg.0.descriptor.oneof_decl.iter() {
            let o = Oneof::new(od, container.clone(), util.clone());
            oneofs.push(o);
        }
        let mut def_exts = msg.0.defined_extensions.borrow_mut();
        for xd in descriptor.extension.iter() {
            let ext = Extension::new(xd, container.clone(), util.clone());
            def_exts.push(ext);
        }

        msg
    }
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
    pub fn util(&self) -> Rc<RefCell<U>> {
        self.0.util.clone()
    }
    pub fn build_target(&self) -> bool {
        self.0.container.build_target()
    }

    pub fn package(&self) -> Option<Package<'a, U>> {
        self.0.container.package()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.0.well_known_type.is_some()
    }
    pub fn container(&self) -> Container<'a, U> {
        self.0.container.upgrade()
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

    pub fn dependents(&self) -> UpgradeIter<Message<'a, U>> {
        UpgradeIter::new(self.0.dependents.clone())
    }

    pub fn defined_extensions(&self) -> Iter<Extension<'a, U>> {
        Iter::from(&self.0.defined_extensions)
    }

    pub(crate) fn add_field(&self, field: Field<'a, U>) {
        self.0.fields.borrow_mut().push(field);
    }

    pub(crate) fn add_dependent(&self, dependent: Weak<Message<'a, U>>) {
        self.0.dependents.borrow_mut().push(dependent);
    }
}

impl<'a, U> Downgrade for Message<'a, U> {
    type Target = WeakMessage<'a, U>;

    fn downgrade(self) -> Self::Target {
        todo!()
    }
}

impl<'a, U> FullyQualified for Message<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}

impl<'a, U> Named<U> for Message<'a, U> {
    fn name(&self) -> Name<U> {
        self.0.name.clone()
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
            match p {
                DescriptorPath::EnumType => msg.0.enums.borrow().get(next).cloned().map(Node::Enum),
                DescriptorPath::Field => msg.0.fields.borrow().get(next).cloned().map(Node::Field),
                DescriptorPath::OneofDecl => {
                    msg.0.oneofs.borrow().get(next).cloned().map(Node::Oneof)
                }
                DescriptorPath::NestedType => msg
                    .0
                    .messages
                    .borrow()
                    .get(next)
                    .cloned()
                    .map(Node::Message),
            }
            .and_then(|n| n.node_at_path(&path[2..]))
        })
    }
}

impl<'a, U> Into<Container<'a, U>> for Message<'a, U> {
    fn into(self) -> Container<'a, U> {
        Container::Message(self)
    }
}

impl<'a, U> Into<Node<'a, U>> for Message<'a, U> {
    fn into(self) -> Node<'a, U> {
        Node::Message(self)
    }
}
impl<'a, U> Deref for Message<'a, U> {
    type Target = Node<'a, U>;
    fn deref(&self) -> &Self::Target {
        &Node::Message(self.clone())
    }
}
#[derive(Debug)]
pub(crate) struct WeakMessage<'a, U>(Weak<MessageDetail<'a, U>>);
impl<'a, U> Upgrade for WeakMessage<'a, U> {
    type Target = Message<'a, U>;

    fn upgrade(self) -> Self::Target {
        Message(self.0.upgrade().expect("Message was dropped"))
    }
}

impl<'a, U> Clone for WeakMessage<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
