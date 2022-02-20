use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use prost_types::DescriptorProto;

use crate::container::BuildTarget;
use crate::iter::UpgradeIter;
use crate::util::Generic;
use crate::{container::Container, container::InternalContainer, Name};
use crate::{AllEnums, EnumList, FieldList, Node, OneofList};
use crate::{Package, WellKnownType};

pub(crate) type MessageList<U> = Rc<RefCell<Vec<Rc<Message<U>>>>>;

/// Message describes a proto message. Messages can be contained in either
/// another Message or File, and may house further Messages and/or Enums. While
/// all Fields technically live on the Message, some may be contained within
/// OneOf blocks.
#[derive(Debug, Clone)]
pub struct Message<U> {
    pub fully_qualified_name: String,
    pub descriptor: DescriptorProto,
    pub is_map_entry: bool,
    pub name: Name<U>,
    pub well_known_type: Option<WellKnownType>, // dependents_cache: RefCell<HashMap<String, Weak<Message<U>>>>,
    pub enums: EnumList<U>,
    pub preserved_messages: MessageList<U>,
    pub messages: MessageList<U>,
    pub maps: MessageList<U>,
    pub fields: FieldList<U>,
    pub oneofs: OneofList<U>,
    pub(crate) dependents: Rc<RefCell<Vec<Weak<Message<U>>>>>,
    pub(crate) container: InternalContainer<U>,
}

impl<U> Message<U> {
    pub(crate) fn new(
        desc: DescriptorProto,
        container: Container<U>,
        lang: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let fully_qualified_name = match desc.name() {
            "" => String::from(""),
            n => format!("{}.{}", container.fully_qualified_name(), n),
        };

        let well_known_type = if container.package().map_or(false, |pkg| pkg.is_well_known()) {
            match WellKnownType::from_str(desc.name()) {
                Ok(wkt) => Some(wkt),
                Err(_) => None,
            }
        } else {
            None
        };

        let name = Name::new(desc.name(), lang);
        Rc::new(Message {
            name,
            container: container.downgrade(),
            fully_qualified_name,
            well_known_type,
            descriptor: desc.clone(),
            is_map_entry: desc.options.as_ref().map_or(false, |o| o.map_entry()),
            enums: Rc::new(RefCell::new(Vec::with_capacity(desc.enum_type.len()))),
            fields: Rc::new(RefCell::new(Vec::with_capacity(desc.field.len()))),
            oneofs: Rc::new(RefCell::new(Vec::with_capacity(desc.oneof_decl.len()))),
            preserved_messages: Rc::new(RefCell::new(Vec::with_capacity(desc.nested_type.len()))),
            messages: Rc::new(RefCell::new(Vec::new())),
            maps: Rc::new(RefCell::new(Vec::new())),
            dependents: Rc::new(RefCell::new(Vec::new())),
        })
    }

    pub fn build_target(&self) -> bool {
        self.container.build_target()
    }

    pub fn package(&self) -> Option<Rc<Package<U>>> {
        self.container.package()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.well_known_type.is_some()
    }
    pub fn container(&self) -> Container<U> {
        self.container.upgrade()
    }
    pub fn all_messages(&self) -> AllMessages<U> {
        AllMessages::new(self.messages.clone())
    }

    pub fn all_enums(&self) -> AllEnums<U> {
        AllEnums::new(self.enums.clone(), self.messages.clone())
    }

    pub fn dependents(&self) -> UpgradeIter<Message<U>> {
        UpgradeIter::new(self.dependents.clone())
    }

    pub(crate) fn add_dependent(&self, dependent: Weak<Message<U>>) {
        self.dependents.borrow_mut().push(dependent);
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<U>> {
        todo!()
    }
}

impl Default for Message<Generic> {
    fn default() -> Self {
        Self {
            fully_qualified_name: Default::default(),
            descriptor: DescriptorProto::default(),
            is_map_entry: Default::default(),
            name: Name::new("", Rc::new(RefCell::new(Generic::default()))),
            enums: Default::default(),
            well_known_type: Default::default(),
            preserved_messages: Default::default(),
            messages: Default::default(),
            maps: Default::default(),
            fields: Default::default(),
            oneofs: Default::default(),
            dependents: Default::default(),
            container: InternalContainer::File(Weak::new()),
        }
    }
}

pub struct AllMessages<U> {
    q: VecDeque<Rc<Message<U>>>,
}

impl<U> AllMessages<U> {
    pub(crate) fn new(msgs: MessageList<U>) -> Self {
        Self {
            q: VecDeque::from_iter(msgs.borrow().iter().cloned()),
        }
    }
}

impl<U> Iterator for AllMessages<U> {
    type Item = Rc<Message<U>>;
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
