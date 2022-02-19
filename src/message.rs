use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use prost_types::DescriptorProto;

use crate::container::BuildTarget;
use crate::iter::{AllEnums, AllMessages, UpgradeIter};
use crate::util::Generic;
use crate::{container::Container, container::InternalContainer, Field, Name, Oneof};
use crate::{Enum, Node};
use crate::{Package, WellKnownType};

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
    pub enums: Vec<Rc<Enum<U>>>,
    pub preserved_messages: Vec<Rc<Message<U>>>,
    pub messages: Vec<Rc<Message<U>>>,
    pub maps: Vec<Rc<Message<U>>>,
    pub fields: Vec<Rc<Field<U>>>,
    pub oneofs: Vec<Rc<Oneof<U>>>,
    pub(crate) dependents: Rc<RefCell<Vec<Weak<Message<U>>>>>,
    pub(crate) container: InternalContainer<U>,
}

impl<U> Message<U> {
    pub(crate) fn new(
        md: &DescriptorProto,
        container: Container<U>,
        lang: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let fully_qualified_name = match md.name() {
            "" => String::from(""),
            n => format!("{}.{}", container.fully_qualified_name(), n),
        };

        let well_known_type = if container.package().map_or(false, |pkg| pkg.is_well_known()) {
            match WellKnownType::from_str(md.name()) {
                Ok(wkt) => Some(wkt),
                Err(_) => None,
            }
        } else {
            None
        };

        let name = Name::new(md.name(), lang);
        Rc::new(Message {
            name,
            container: container.downgrade(),
            fully_qualified_name,
            well_known_type,
            descriptor: md.clone(),
            is_map_entry: md.options.as_ref().map_or(false, |o| o.map_entry()),
            enums: Vec::default(),
            preserved_messages: Vec::default(),
            messages: Vec::default(),
            fields: Vec::default(),
            oneofs: Vec::default(),
            maps: Vec::default(),
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
        AllMessages::new(&self.messages)
    }

    pub fn all_enums(&self) -> AllEnums<U> {
        AllEnums::new(&self.enums, &self.messages)
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
