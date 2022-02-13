use std::rc::{Rc, Weak};

use crate::{lang::Lang, File, Message, Name, Package};

// pub enum Entity {

// }
#[derive(Debug, Clone)]
pub(crate) enum InternalContainer<L: Lang> {
    File(Weak<File<L>>),
    Message(Weak<Message<L>>),
}

impl<L: Lang> From<Rc<File<L>>> for InternalContainer<L> {
    fn from(file: Rc<File<L>>) -> Self {
        InternalContainer::File(Rc::downgrade(&file))
    }
}

impl<L: Lang> From<Rc<Message<L>>> for InternalContainer<L> {
    fn from(message: Rc<Message<L>>) -> Self {
        InternalContainer::Message(Rc::downgrade(&message))
    }
}

impl<L: Lang> InternalContainer<L> {
    // TODO: should this return Option<Container<L>>?
    pub(crate) fn upgrade(&self) -> Container<L> {
        match self {
            InternalContainer::File(f) => Container::File(f.upgrade().unwrap()),
            InternalContainer::Message(m) => Container::Message(m.upgrade().unwrap()),
        }
    }
    pub(crate) fn add_message(&self, message: Rc<Message<L>>) {
        match self {
            InternalContainer::File(f) => {
                f.upgrade().unwrap().add_message(message);
            }
            InternalContainer::Message(m) => {
                m.upgrade().unwrap().add_message(message);
            }
        }
    }
    pub(crate) fn fully_qualified_name(&self) -> String {
        match self {
            InternalContainer::File(f) => f.upgrade().unwrap().fully_qualified_name.clone(),
            InternalContainer::Message(m) => m.upgrade().unwrap().fully_qualified_name.clone(),
        }
    }

    pub(crate) fn package(&self) -> Rc<Package<L>> {
        match self {
            InternalContainer::File(f) => f.upgrade().unwrap().package(),
            InternalContainer::Message(m) => m.upgrade().unwrap().package(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Container<L: Lang> {
    File(Rc<File<L>>),
    Message(Rc<Message<L>>),
}

impl<L: Lang> Container<L> {
    pub fn fully_qualified_name(&self) -> &str {
        match self {
            Container::File(f) => &f.fully_qualified_name,
            Container::Message(m) => &m.fully_qualified_name,
        }
    }
    pub fn package(&self) -> Rc<Package<L>> {
        match self {
            Container::File(f) => f.package(),
            Container::Message(m) => m.package(),
        }
    }
    pub(crate) fn downgrade(&self) -> InternalContainer<L> {
        match self {
            Container::File(f) => InternalContainer::File(Rc::downgrade(&f)),
            Container::Message(m) => InternalContainer::Message(Rc::downgrade(&m)),
        }
    }
}

impl<L: Lang> Container<L> {
    pub fn name(&self) -> Name<L> {
        match self {
            Container::File(f) => f.name.clone(),
            Container::Message(m) => m.name.clone(),
        }
    }
}

pub trait BuildTarget {
    fn build_target(&self) -> bool;
}

impl<L: Lang> BuildTarget for Container<L> {
    fn build_target(&self) -> bool {
        match self {
            Container::File(f) => f.build_target(),
            Container::Message(m) => m.build_target(),
        }
    }
}

impl<L: Lang> BuildTarget for InternalContainer<L> {
    fn build_target(&self) -> bool {
        match self {
            InternalContainer::File(f) => f.upgrade().unwrap().build_target(),
            InternalContainer::Message(m) => m.upgrade().unwrap().build_target(),
        }
    }
}

impl<L: Lang> InternalContainer<L> {
    pub(crate) fn name(&self) -> Name<L> {
        match self {
            InternalContainer::File(f) => f.upgrade().unwrap().name.clone(),
            InternalContainer::Message(m) => m.upgrade().unwrap().name.clone(),
        }
    }
}
