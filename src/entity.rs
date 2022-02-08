use std::rc::{Rc, Weak};

use crate::{lang::Lang, name::Named, File, Message, Package};

// pub enum Entity {

// }
#[derive(Debug)]
pub(crate) enum InternalContainer<L: Lang> {
    File(Weak<File<L>>),
    Message(Weak<Message<L>>),
}

impl<L: Lang> InternalContainer<L> {
    // TODO: should this return Option<Container<L>>?
    pub(crate) fn upgrade(&self) -> Container<L> {
        match self {
            InternalContainer::File(f) => Container::File(f.upgrade().unwrap()),
            InternalContainer::Message(m) => Container::Message(m.upgrade().unwrap()),
        }
    }
    pub(crate) fn fully_qualified_name(&self) -> String {
        match self {
            InternalContainer::File(f) => f.upgrade().unwrap().fully_qualified_name(),
            InternalContainer::Message(m) => m.upgrade().unwrap().fully_qualified_name(),
        }
    }

    pub(crate) fn package(&self) -> Rc<Package<L>> {
        match self {
            InternalContainer::File(f) => f.upgrade().unwrap().package(),
            InternalContainer::Message(m) => m.upgrade().unwrap().package(),
        }
    }
}

pub enum Container<L: Lang> {
    File(Rc<File<L>>),
    Message(Rc<Message<L>>),
}

impl<L: Lang> Container<L> {
    pub fn fully_qualified_name(&self) -> String {
        match self {
            Container::File(f) => f.fully_qualified_name(),
            Container::Message(m) => m.fully_qualified_name(),
        }
    }
    pub fn package(&self) -> Rc<Package<L>> {
        match self {
            Container::File(f) => f.package(),
            Container::Message(m) => m.package(),
        }
    }
}

pub trait BuildTarget {
    fn build_target(&self) -> bool;
}

impl<L: Lang> InternalContainer<L> {
    pub(crate) fn new_file(target: Rc<File<L>>) -> Self {
        InternalContainer::File(Rc::downgrade(&target))
    }
    pub(crate) fn new_message(target: Rc<Message<L>>) -> Self {
        InternalContainer::Message(Rc::downgrade(&target))
    }
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

impl<L: Lang> Named<L> for InternalContainer<L> {
    fn name(&self) -> crate::Name<L> {
        match self {
            InternalContainer::File(f) => f.upgrade().unwrap().name(),
            InternalContainer::Message(m) => m.upgrade().unwrap().name(),
        }
    }
}
