use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use crate::container::BuildTarget;
use crate::lang::Lang;
use crate::{container::Container, container::InternalContainer, Field, Name, OneOf};
use crate::{Package, WellKnownType};

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
    preserved_messages: RefCell<Vec<Rc<Message<L>>>>,
    messages: RefCell<Vec<Rc<Message<L>>>>,
    fields: RefCell<Vec<Rc<Field<L>>>>,
    one_ofs: RefCell<Vec<Rc<OneOf<L>>>>,
    dependents: RefCell<Vec<Weak<Message<L>>>>,
    container: InternalContainer<L>,
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
    ) -> Self {
        let fully_qualified_name = match descriptor.name() {
            "" => String::from(""),
            n => format!("{}.{}", container.fully_qualified_name(), n),
        };
        let well_known_type = if container.package().is_well_known() {
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
        Message {
            name,
            container,
            fully_qualified_name,
            well_known_type,
            descriptor,
            is_map_entry,
            preserved_messages: RefCell::new(Vec::new()),
            messages: RefCell::new(Vec::new()),
            fields: RefCell::new(Vec::new()),
            one_ofs: RefCell::new(Vec::new()),
            dependents: RefCell::new(Vec::new()),
        }
    }
    pub fn package(&self) -> Rc<Package<L>> {
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
    pub fn all_messages(&self) -> Vec<Rc<Message<L>>> {
        let mut res: Vec<Rc<Message<L>>> = Vec::default();
        let mut stack: VecDeque<Rc<Message<L>>> = VecDeque::default();
        stack.extend(self.messages());
        res.extend(self.messages());
        while let Some(next) = stack.pop_front() {
            stack.extend(next.messages());
            res.extend(next.messages());
        }
        res
    }

    pub fn messages(&self) -> Vec<Rc<Message<L>>> {
        self.messages.borrow().clone()
    }
    pub fn one_ofs(&self) -> Vec<Rc<OneOf<L>>> {
        self.one_ofs.borrow().clone()
    }
    pub fn dependencies(&self) -> Vec<Rc<Message<L>>> {
        self.dependents
            .borrow()
            .iter()
            .map(|w| w.upgrade().unwrap())
            .collect()
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
