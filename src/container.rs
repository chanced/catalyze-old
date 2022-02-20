use std::rc::{Rc, Weak};

use crate::{
    iter::{AllEnums, AllMessages, Iter},
    Enum,
};
use crate::{File, Message, Name, Package};

// pub enum Entity {

// }
#[derive(Debug, Clone)]
pub(crate) enum InternalContainer<U> {
    File(Weak<File<U>>),
    Message(Weak<Message<U>>),
}

impl<U> From<Rc<File<U>>> for InternalContainer<U> {
    fn from(file: Rc<File<U>>) -> Self {
        InternalContainer::File(Rc::downgrade(&file))
    }
}

impl<U> From<Rc<Message<U>>> for InternalContainer<U> {
    fn from(message: Rc<Message<U>>) -> Self {
        InternalContainer::Message(Rc::downgrade(&message))
    }
}

impl<U> InternalContainer<U> {
    // TODO: should this return Option<Container<U>>?
    pub(crate) fn upgrade(&self) -> Container<U> {
        match self {
            InternalContainer::File(f) => Container::File(f.upgrade().unwrap()),
            InternalContainer::Message(m) => Container::Message(m.upgrade().unwrap()),
        }
    }
    // pub(crate) fn add_message(&self, message: Rc<Message<U>>) {
    //     match self {
    //         InternalContainer::File(f) => {
    //             f.upgrade().unwrap().add_message(message);
    //         }
    //         InternalContainer::Message(m) => {
    //             m.upgrade().unwrap().add_message(message);
    //         }
    //     }
    // }
    pub(crate) fn fully_qualified_name(&self) -> String {
        match self {
            InternalContainer::File(f) => f.upgrade().unwrap().fully_qualified_name.clone(),
            InternalContainer::Message(m) => m.upgrade().unwrap().fully_qualified_name.clone(),
        }
    }

    pub(crate) fn package(&self) -> Option<Rc<Package<U>>> {
        match self {
            InternalContainer::File(f) => f.upgrade().unwrap().package(),
            InternalContainer::Message(m) => m.upgrade().unwrap().package(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Container<U> {
    File(Rc<File<U>>),
    Message(Rc<Message<U>>),
}

impl<U> From<Rc<File<U>>> for Container<U> {
    fn from(file: Rc<File<U>>) -> Self {
        Container::File(file)
    }
}

impl<U> From<Rc<Message<U>>> for Container<U> {
    fn from(message: Rc<Message<U>>) -> Self {
        Container::Message(message)
    }
}

impl<U> Container<U> {
    pub fn fully_qualified_name(&self) -> &str {
        match self {
            Container::File(f) => &f.fully_qualified_name,
            Container::Message(m) => &m.fully_qualified_name,
        }
    }
    // pub(crate) fn msgs(&self) -> Rc<RefCell<Vec<Rc<Message<U>>>>> {
    //     match self {
    //         Container::File(f) => f.messages.clone(),
    //         Container::Message(m) => m.messages.clone(),
    //     }
    // }
    pub fn name(&self) -> &Name<U> {
        match self {
            Container::File(f) => &f.name,
            Container::Message(m) => &m.name,
        }
    }
    pub fn messages(&self) -> Iter<Message<U>> {
        match self {
            Container::File(f) => Iter::from(&f.messages),
            Container::Message(m) => Iter::from(&m.messages),
        }
    }
    pub fn all_messages(&self) -> AllMessages<U> {
        match self {
            Container::File(f) => f.all_messages(),
            Container::Message(m) => m.all_messages(),
        }
    }
    pub fn all_enums(&self) -> AllEnums<U> {
        match self {
            Container::File(f) => f.all_enums(),
            Container::Message(m) => m.all_enums(),
        }
    }
    pub fn enums(&self) -> Iter<Enum<U>> {
        match self {
            Container::File(f) => Iter::from(&f.enums),
            Container::Message(m) => Iter::from(&m.enums),
        }
    }
    pub fn package(&self) -> Option<Rc<Package<U>>> {
        match self {
            Container::File(f) => f.package(),
            Container::Message(m) => m.package(),
        }
    }
    pub(crate) fn downgrade(&self) -> InternalContainer<U> {
        match self {
            Container::File(f) => InternalContainer::File(Rc::downgrade(f)),
            Container::Message(m) => InternalContainer::Message(Rc::downgrade(m)),
        }
    }

    // pub(crate) fn add_message(&self, msg: Rc<Message<U>>) {
    //     match self {
    //         Container::File(f) => f.add_message(msg),
    //         Container::Message(m) => m.add_message(msg),
    //     }
    // }

    // pub(crate) fn add_enum(&self, e: Rc<Enum<U>>) {
    //     match self {
    //         Container::File(f) => f.add_enum(e),
    //         Container::Message(m) => m.add_enum(e),
    //     }
    // }
}

pub trait BuildTarget {
    fn build_target(&self) -> bool;
}
impl<U> BuildTarget for Container<U> {
    fn build_target(&self) -> bool {
        match self {
            Container::File(f) => f.build_target(),
            Container::Message(m) => m.build_target(),
        }
    }
}

impl<U> BuildTarget for InternalContainer<U> {
    fn build_target(&self) -> bool {
        match self {
            InternalContainer::File(f) => f.upgrade().unwrap().build_target(),
            InternalContainer::Message(m) => m.upgrade().unwrap().build_target(),
        }
    }
}
