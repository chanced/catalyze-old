use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use prost_types::DescriptorProto;

use crate::container::BuildTarget;
use crate::iter::{Iter, UpgradeIter};
use crate::name::Named;
use crate::path::DescriptorPath;
use crate::{container::Container, container::WeakContainer, Name};
use crate::{
    format_fqn, AllEnums, Enum, EnumList, Extension, Field, FieldList, FullyQualified, Node,
    NodeAtPath, Oneof, OneofList,
};
use crate::{Package, WellKnownType};

pub(crate) type MessageList<'a, U> = Rc<RefCell<Vec<Rc<Message<'a, U>>>>>;

/// Message describes a proto message. Messages can be contained in either
/// another Message or a File, and may house further Messages and/or Enums. While
/// all Fields technically live on the Message, some may be contained within
/// OneOf blocks.
#[derive(Debug, Clone)]
pub struct Message<'a, U> {
    pub name: Name<U>,
    pub is_map_entry: bool,
    pub descriptor: &'a DescriptorProto,
    pub well_known_type: Option<WellKnownType>,
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
    defined_extensions: Rc<RefCell<Vec<Rc<Extension<'a, U>>>>>,
    /// `Extension`s applied to this `Message`
    applied_extensions: Rc<RefCell<Vec<Weak<Extension<'a, U>>>>>,
}

impl<'a, U> Message<'a, U> {
    pub(crate) fn new(
        descriptor: &'a DescriptorProto,
        container: Container<'a, U>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let fqn = format_fqn(&container, descriptor.name());
        let well_known_type = if container.package().map_or(false, |pkg| pkg.is_well_known()) {
            match WellKnownType::from_str(&fqn) {
                Ok(wkt) => Some(wkt),
                Err(_) => None,
            }
        } else {
            None
        };

        let msg = Rc::new(Message {
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
        });

        let container = Container::Message(msg.clone());
        {
            let mut msgs = msg.messages.borrow_mut();
            for md in msg.descriptor.nested_type.iter() {
                let msg = Message::new(md, container.clone(), util.clone());
                msgs.push(msg);
            }
        }
        {
            let mut enums = msg.enums.borrow_mut();
            for ed in descriptor.enum_type.iter() {
                let e = Enum::new(ed, container.clone(), util.clone());
                enums.push(e);
            }
        }
        {
            let mut oneofs = msg.oneofs.borrow_mut();
            for od in msg.descriptor.oneof_decl.iter() {
                let o = Oneof::new(od, container.clone(), util.clone());
                oneofs.push(o);
            }
        }
        {
            let mut def_exts = msg.defined_extensions.borrow_mut();
            for xd in descriptor.extension.iter() {
                let ext = Extension::new(xd, container.clone(), util.clone());
                def_exts.push(ext);
            }
        }

        msg
    }
    pub fn name(&self) -> Name<U> {
        self.name.clone()
    }
    pub fn util(&self) -> Rc<RefCell<U>> {
        self.util.clone()
    }
    pub fn build_target(&self) -> bool {
        self.container.build_target()
    }

    pub fn package(&self) -> Option<Rc<Package<'a, U>>> {
        self.container.package()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.well_known_type.is_some()
    }
    pub fn container(&self) -> Container<'a, U> {
        self.container.upgrade()
    }

    pub fn fields(&self) -> Iter<Field<'a, U>> {
        Iter::from(&self.fields)
    }

    pub fn messages(&self) -> Iter<Self> {
        Iter::from(&self.messages)
    }

    pub fn enums(&self) -> Iter<Enum<'a, U>> {
        Iter::from(&self.enums)
    }

    pub fn all_messages(&self) -> AllMessages<'a, U> {
        AllMessages::new(self.messages.clone())
    }

    pub fn all_enums(&self) -> AllEnums<'a, U> {
        AllEnums::new(self.enums.clone(), self.messages.clone())
    }

    pub fn dependents(&self) -> UpgradeIter<Message<'a, U>> {
        UpgradeIter::new(self.dependents.clone())
    }

    pub fn defined_extensions(&self) -> Iter<Extension<'a, U>> {
        Iter::from(&self.defined_extensions)
    }

    pub(crate) fn add_field(&self, field: Rc<Field<'a, U>>) {
        self.fields.borrow_mut().push(field);
    }

    pub(crate) fn add_dependent(&self, dependent: Weak<Message<'a, U>>) {
        self.dependents.borrow_mut().push(dependent);
    }
}

impl<'a, U> FullyQualified for Message<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
}

impl<'a, U> Named<U> for Message<'a, U> {
    fn name(&self) -> Name<U> {
        self.name.clone()
    }
}

impl<'a, U> NodeAtPath<'a, U> for Rc<Message<'a, U>> {
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
                DescriptorPath::EnumType => msg.enums.borrow().get(next).cloned().map(Node::Enum),
                DescriptorPath::Field => msg.fields.borrow().get(next).cloned().map(Node::Field),
                DescriptorPath::OneofDecl => {
                    msg.oneofs.borrow().get(next).cloned().map(Node::Oneof)
                }
                DescriptorPath::NestedType => {
                    msg.messages.borrow().get(next).cloned().map(Node::Message)
                }
            }
            .and_then(|n| n.node_at_path(&path[2..]))
        })
    }
}

pub struct AllMessages<'a, U> {
    q: VecDeque<Rc<Message<'a, U>>>,
}

impl<'a, U> AllMessages<'a, U> {
    pub(crate) fn new(msgs: MessageList<'a, U>) -> Self {
        Self {
            q: VecDeque::from_iter(msgs.borrow().iter().cloned()),
        }
    }
}

impl<'a, U> Iterator for AllMessages<'a, U> {
    type Item = Rc<Message<'a, U>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(msg) = self.q.pop_front() {
            for v in msg.messages.borrow().iter().cloned() {
                self.q.push_back(v);
            }
            Some(msg)
        } else {
            None
        }
    }
}
