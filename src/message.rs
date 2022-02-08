use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use crate::entity::BuildTarget;
use crate::lang::Lang;
use crate::name::Named;
use crate::protobuf::WellKnownType;
use crate::Package;
use crate::{entity::Container, entity::InternalContainer, Field, Name, OneOf};

/// Message describes a proto message. Messages can be contained in either
/// another Message or File, and may house further Messages and/or Enums. While
/// all Fields technically live on the Message, some may be contained within
/// OneOf blocks.
#[derive(Debug)]
pub struct Message<L: Lang> {
    name: Name<L>,
    fqn: String,
    desc: Rc<prost_types::DescriptorProto>,
    preserved_messages: RefCell<Vec<Rc<Message<L>>>>,
    messages: RefCell<Vec<Rc<Message<L>>>>,
    fields: RefCell<Vec<Rc<Field<L>>>>,
    one_ofs: RefCell<Vec<Rc<OneOf<L>>>>,
    dependents: RefCell<Vec<Weak<Message<L>>>>,
    is_map: bool,
    container: InternalContainer<L>,
    wkt: Option<WellKnownType>, // dependents_cache: RefCell<HashMap<String, Weak<Message<L>>>>,
}

impl<L: Lang> Named<L> for Message<L> {
    fn name(&self) -> Name<L> {
        self.name.clone()
    }
}
impl<L: Lang> BuildTarget for Message<L> {
    fn build_target(&self) -> bool {
        self.container.build_target()
    }
}

impl<L: Lang> Message<L> {
    pub(crate) fn new(
        desc: prost_types::DescriptorProto,
        container: InternalContainer<L>,
        lang: L,
    ) -> Self {
        let desc = Rc::new(desc);
        let fqn = match desc.name() {
            "" => String::from(""),
            n => format!("{}.{}", container.fully_qualified_name(), n),
        };

        let wkt = if container.package().is_well_known() {
            match WellKnownType::from_str(desc.name()) {
                Ok(wkt) => Some(wkt),
                Err(_) => None,
            }
        } else {
            None
        };
        let is_map = match desc.options {
            Some(o) => o.map_entry(),
            None => false,
        };

        Message {
            container,
            fqn,
            wkt,
            desc,
            name: Name::new(desc.name(), lang),
            is_map: desc.options.is_some().map_entry(),
            preserved_messages: RefCell::new(Vec::new()),
            messages: RefCell::new(Vec::new()),
            fields: RefCell::new(Vec::new()),
            one_ofs: RefCell::new(Vec::new()),
            dependents: RefCell::new(Vec::new()),
        }
    }
    pub fn package(&self) -> Rc<Package<L>> {
        return self.container.package();
    }

    pub fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
    /// returns the underlying proto descriptor for this message
    pub fn descriptor(&self) -> &prost_types::DescriptorProto {
        return &self.desc;
    }
    pub fn is_map(&self) -> bool {
        self.is_map
    }
    pub fn is_well_known_type(&self) -> bool {
        self.wkt.is_some()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.wkt.clone()
    }

    pub fn fields(&self) -> Vec<Rc<Field<L>>> {
        unimplemented!()
    }
    pub fn container(&self) -> Container<L> {
        self.container.upgrade()
    }
    pub fn messages(&self) -> Vec<Rc<Message<L>>> {
        self.messages.borrow().iter().map(Rc::clone).collect()
    }
    pub fn one_ofs(&self) -> Vec<Rc<OneOf<L>>> {
        self.one_ofs.borrow().iter().map(Rc::clone).collect()
    }
    pub fn dependencies(&self) -> Vec<Rc<Message<L>>> {
        self.dependents
            .borrow()
            .iter()
            .map(|w| w.upgrade().unwrap())
            .collect()
    }
    pub fn preserved_messages(&self) -> Vec<Rc<Message<L>>> {
        self.preserved_messages
            .borrow()
            .iter()
            .map(Rc::clone)
            .collect()
    }
    pub(crate) fn add_preserved_message(&self, msg: Message<L>) {
        self.preserved_messages.borrow_mut().push(Rc::new(msg));
    }
    pub(crate) fn add_message(&self, msg: Message<L>) {
        self.messages.borrow_mut().push(Rc::new(msg));
    }
    pub(crate) fn add_field(&self, field: Field<L>) {
        self.fields.borrow_mut().push(Rc::new(field));
    }
    pub(crate) fn add_one_of(&self, one_of: OneOf<L>) {
        self.one_ofs.borrow_mut().push(Rc::new(one_of));
    }
    pub(crate) fn add_dependent(&self, dependent: Weak<Message<L>>) {
        self.dependents.borrow_mut().push(dependent);
    }
}
