use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use crate::container::BuildTarget;
use crate::iter::{AllEnums, AllMessages, UpgradeIter};
use crate::util::Unspecified;
use crate::{container::Container, container::InternalContainer, Field, Name, Oneof};
use crate::{iter::Iter, Package, WellKnownType};
use crate::{Enum, EnumList, FieldList, OneofList};

pub(crate) type WeakMessageList<U> = Rc<RefCell<Vec<Weak<Message<U>>>>>;
pub(crate) type MessageList<U> = Rc<RefCell<Vec<Rc<Message<U>>>>>;

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
    pub(crate) enums: EnumList<U>,
    pub(crate) preserved_messages: MessageList<U>,
    pub(crate) messages: MessageList<U>,
    pub(crate) fields: FieldList<U>,
    pub(crate) oneofs: OneofList<U>,
    pub(crate) dependents: WeakMessageList<U>,
    pub(crate) container: InternalContainer<U>,
}

impl Default for Message<Unspecified> {
    fn default() -> Self {
        Self {
            fully_qualified_name: Default::default(),
            descriptor: Default::default(),
            is_map_entry: Default::default(),
            name: Name::new("", Unspecified),
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
        let r = Rc::new(Message {
            name,
            container: container.downgrade(),
            fully_qualified_name,
            well_known_type,
            descriptor,
            is_map_entry,
            enums: Rc::new(RefCell::new(Vec::new())),
            preserved_messages: Rc::new(RefCell::new(Vec::new())),
            messages: Rc::new(RefCell::new(Vec::new())),
            fields: Rc::new(RefCell::new(Vec::new())),
            oneofs: Rc::new(RefCell::new(Vec::new())),
            dependents: Rc::new(RefCell::new(Vec::new())),
        });

        r.add_message(r.clone());
        r
    }

    pub fn package(&self) -> Option<Rc<Package<U>>> {
        self.container.package()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.well_known_type.is_some()
    }

    pub fn fields(&self) -> Vec<Rc<Field<U>>> {
        self.fields.borrow().clone()
    }
    pub fn container(&self) -> Container<U> {
        self.container.upgrade()
    }

    pub fn messages(&self) -> Iter<Message<U>> {
        Iter::new(self.messages.clone())
    }

    pub fn all_messages(&self) -> AllMessages<U> {
        AllMessages::new(self.messages.clone())
    }
    pub fn enums(&self) -> Iter<Enum<U>> {
        Iter::new(self.enums.clone())
    }
    pub fn all_enums(&self) -> AllEnums<U> {
        AllEnums::new(self.enums.clone(), self.messages.clone())
    }
    pub(crate) fn add_enum(&self, e: Rc<Enum<U>>) {
        self.enums.borrow_mut().push(e);
    }

    pub fn one_ofs(&self) -> Vec<Rc<Oneof<U>>> {
        self.oneofs.borrow().clone()
    }
    pub fn dependencies(&self) -> UpgradeIter<Message<U>> {
        UpgradeIter::new(self.dependents.clone())
    }
    pub fn preserved_messages(&self) -> Vec<Rc<Message<U>>> {
        self.preserved_messages.borrow().clone()
    }
    pub(crate) fn add_preserved_message(&self, msg: Rc<Message<U>>) {
        self.preserved_messages.borrow_mut().push(msg);
    }
    pub(crate) fn add_message(&self, msg: Rc<Message<U>>) {
        self.messages.borrow_mut().push(msg);
    }
    pub(crate) fn add_field(&self, field: Rc<Field<U>>) {
        self.fields.borrow_mut().push(field);
    }
    pub(crate) fn add_one_of(&self, one_of: Rc<Oneof<U>>) {
        self.oneofs.borrow_mut().push(one_of);
    }
    pub(crate) fn add_dependent(&self, dependent: Weak<Message<U>>) {
        self.dependents.borrow_mut().push(dependent);
    }
}
