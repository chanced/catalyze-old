use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    container::{Container, InternalContainer},
    util::{Lang, Unspecified},
    Message, Name, Package,
};

pub(crate) type EnumList<U> = Rc<RefCell<Vec<Rc<Enum<U>>>>>;

#[derive(Debug, Clone)]
pub struct Enum<U> {
    pub name: Name<U>,
    pub(crate) container: InternalContainer<U>,
}

impl<U> Enum<U> {
    pub fn new(name: Name<U>, container: Container<U>) -> Self {
        Enum {
            name,
            container: container.downgrade(),
        }
    }
    pub fn fully_qualified_name(&self) -> String {
        self.container.fully_qualified_name()
    }
    pub fn package(&self) -> Option<Rc<Package<U>>> {
        self.container.package()
    }
    pub fn add_message(&self, message: Rc<Message<U>>) {
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
