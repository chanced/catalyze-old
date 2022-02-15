use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use crate::container::BuildTarget;
use crate::iter::{AllMessages, UpgradeIter};
use crate::lang::{Lang, Unspecified};
use crate::{container::Container, container::InternalContainer, Field, Name, OneOf};
use crate::{iter::Iter, Package, WellKnownType};

/// Message describes a proto message. Messages can be contained in either
/// another Message or File, and may house further Messages and/or Enums. While
/// all Fields technically live on the Message, some may be contained within
/// OneOf blocks.
#[derive(Debug, Clone)]
pub struct Message<L: Lang> {
    pub fully_qualified_name: String,
    pub descriptor: prost_types::DescriptorProto,
    pub is_map_entry: bool,
    pub name: Name<L>,
    pub well_known_type: Option<WellKnownType>, // dependents_cache: RefCell<HashMap<String, Weak<Message<L>>>>,
    pub(crate) preserved_messages: Rc<RefCell<Vec<Rc<Message<L>>>>>,
    pub(crate) messages: Rc<RefCell<Vec<Rc<Message<L>>>>>,
    pub(crate) fields: Rc<RefCell<Vec<Rc<Field<L>>>>>,
    pub(crate) one_ofs: Rc<RefCell<Vec<Rc<OneOf<L>>>>>,
    pub(crate) dependents: Rc<RefCell<Vec<Weak<Message<L>>>>>,
    pub(crate) container: InternalContainer<L>,
}

impl Default for Message<Unspecified> {
    fn default() -> Self {
        Self {
            fully_qualified_name: Default::default(),
            descriptor: Default::default(),
            is_map_entry: Default::default(),
            name: Name::new("", Unspecified),
            well_known_type: Default::default(),
            preserved_messages: Default::default(),
            messages: Default::default(),
            fields: Default::default(),
            one_ofs: Default::default(),
            dependents: Default::default(),
            container: InternalContainer::File(Weak::new()),
        }
    }
}

impl<L: Lang> BuildTarget for Message<L> {
    fn build_target(&self) -> bool {
        self.container.build_target()
    }
}

impl<L: Lang> Message<L> {
    pub(crate) fn new(
        descriptor: prost_types::DescriptorProto,
        container: InternalContainer<L>,
        lang: L,
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
        Rc::new(Message {
            name,
            container,
            fully_qualified_name,
            well_known_type,
            descriptor,
            is_map_entry,
            preserved_messages: Rc::new(RefCell::new(Vec::new())),
            messages: Rc::new(RefCell::new(Vec::new())),
            fields: Rc::new(RefCell::new(Vec::new())),
            one_ofs: Rc::new(RefCell::new(Vec::new())),
            dependents: Rc::new(RefCell::new(Vec::new())),
        })
    }

    pub fn package(&self) -> Option<Rc<Package<L>>> {
        self.container.package()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.well_known_type.is_some()
    }

    pub fn fields(&self) -> Vec<Rc<Field<L>>> {
        self.fields.borrow().clone()
    }
    pub fn container(&self) -> Container<L> {
        self.container.upgrade()
    }
    pub fn all_messages(&self) -> AllMessages<L> {
        AllMessages::new(self.messages.clone())
    }

    pub fn messages(&self) -> Iter<Message<L>> {
        Iter::new(self.messages.clone())
    }

    pub fn one_ofs(&self) -> Vec<Rc<OneOf<L>>> {
        self.one_ofs.borrow().clone()
    }
    pub fn dependencies(&self) -> UpgradeIter<Message<L>> {
        UpgradeIter::new(self.dependents.clone())
    }
    pub fn preserved_messages(&self) -> Vec<Rc<Message<L>>> {
        self.preserved_messages.borrow().clone()
    }
    pub(crate) fn add_preserved_message(&self, msg: Rc<Message<L>>) {
        self.preserved_messages.borrow_mut().push(msg);
    }
    pub(crate) fn add_message(&self, msg: Rc<Message<L>>) {
        self.messages.borrow_mut().push(msg);
    }
    pub(crate) fn add_field(&self, field: Rc<Field<L>>) {
        self.fields.borrow_mut().push(field);
    }
    pub(crate) fn add_one_of(&self, one_of: Rc<OneOf<L>>) {
        self.one_ofs.borrow_mut().push(one_of);
    }
    pub(crate) fn add_dependent(&self, dependent: Weak<Message<L>>) {
        self.dependents.borrow_mut().push(dependent);
    }
}
