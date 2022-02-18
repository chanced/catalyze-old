use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::str::FromStr;

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
    pub descriptor: prost_types::DescriptorProto,
    pub is_map_entry: bool,
    pub name: Name<U>,
    pub well_known_type: Option<WellKnownType>, // dependents_cache: RefCell<HashMap<String, Weak<Message<U>>>>,
    pub enums: Vec<Rc<Enum<U>>>,
    pub preserved_messages: Vec<Rc<Message<U>>>,
    pub messages: Vec<Rc<Message<U>>>,
    pub fields: Vec<Rc<Field<U>>>,
    pub oneofs: Vec<Rc<Oneof<U>>>,
    pub(crate) dependents: Rc<RefCell<Vec<Weak<Message<U>>>>>,
    pub(crate) container: InternalContainer<U>,
}

impl Default for Message<Generic> {
    fn default() -> Self {
        Self {
            fully_qualified_name: Default::default(),
            descriptor: Default::default(),
            is_map_entry: Default::default(),
            name: Name::new("", Generic),
            enums: Default::default(),
            well_known_type: Default::default(),
            preserved_messages: Default::default(),
            messages: Default::default(),
            fields: Default::default(),
            oneofs: Default::default(),
            dependents: Default::default(),
            container: InternalContainer::File(Weak::new()),
        }
    }
}

impl<U> BuildTarget for Message<U> {
    fn build_target(&self) -> bool {
        self.container.build_target()
    }
}

impl<U> Message<U> {
    pub(crate) fn new(
        descriptor: prost_types::DescriptorProto,
        container: Container<U>,
        lang: U,
    ) -> Rc<Self> {
        let fully_qualified_name = match descriptor.name() {
            "" => String::from(""),
            n => format!("{}.{}", container.fully_qualified_name(), n),
        };

        let well_known_type = if container.package().map_or(false, |pkg| pkg.is_well_known()) {
            match WellKnownType::from_str(descriptor.name()) {
                Ok(wkt) => Some(wkt),
                Err(_) => None,
            }
        } else {
            None
        };
        // let is_map = match descriptor.options {
        //     Some(o) => o.map_entry(),
        //     None => false,
        // };
        // let is_map_entry = match descriptor.options {
        //     Some(o) => o.map_entry(),
        //     None => false,
        // };
        let name = Name::new(descriptor.name(), lang);
        let is_map_entry = false;
        let m = Message {
            name,
            container: container.downgrade(),
            fully_qualified_name,
            well_known_type,
            descriptor,
            is_map_entry,
            enums: Vec::default(),
            preserved_messages: Vec::default(),
            messages: Vec::default(), //with_capacity(descriptor.nested_type.len()),
            fields: Vec::default(),   //with_capacity(descriptor.field.len()),
            oneofs: Vec::default(),   //Vec::with_capacity(descriptor.oneof_decl.len()),
            dependents: Rc::new(RefCell::new(Vec::new())),
        };

        Rc::new(m)
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
