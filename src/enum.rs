use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    container::{Container, InternalContainer},
    lang::{Lang, Unspecified},
    Message, Name, Package,
};

pub(crate) type EnumList<L> = Rc<RefCell<Vec<Rc<Enum<L>>>>>;

#[derive(Debug, Clone)]
pub struct Enum<L> {
    pub name: Name<L>,
    pub(crate) container: InternalContainer<L>,
}

impl<L: Lang> Enum<L> {
    pub fn new(name: Name<L>, container: Container<L>) -> Self {
        Enum {
            name,
            container: container.downgrade(),
        }
    }
    pub fn fully_qualified_name(&self) -> String {
        self.container.fully_qualified_name()
    }
    pub fn package(&self) -> Option<Rc<Package<L>>> {
        self.container.package()
    }
    pub fn add_message(&self, message: Rc<Message<L>>) {
        self.container.add_message(message);
    }
}

impl Default for Enum<Unspecified> {
    fn default() -> Self {
        Self {
            name: Name::new("", Unspecified),
            container: InternalContainer::File(Weak::new()),
        }
    }
}
