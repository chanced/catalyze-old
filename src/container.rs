use std::rc::{Rc, Weak};

use crate::file::{FileDetail, WeakFile};
use crate::iter::{AllEnums, AllMessages, Iter};
use crate::{Enum, File, FullyQualified, Message, Name, Node, Package, WeakMessage};

// pub enum Entity {

// }
#[derive(Debug, Clone)]
pub(crate) enum WeakContainer<'a, U> {
    File(WeakFile<'a, U>),
    Message(WeakMessage<'a, U>),
}

impl<'a, U> WeakContainer<'a, U> {
    // TODO: should this return Option<Container<'a, U>>?
    pub(crate) fn upgrade(&self) -> Container<'a, U> {
        match self {
            WeakContainer::File(f) => Container::File(File(f.upgrade().unwrap())),
            WeakContainer::Message(m) => Container::Message(m.upgrade().unwrap()),
        }
    }

    pub(crate) fn package(&self) -> Option<Package<'a, U>> {
        match self {
            WeakContainer::File(f) => self.upgrade().package(),
            WeakContainer::Message(m) => self.upgrade().package(),
        }
    }
}
#[derive(Debug)]
pub enum Container<'a, U> {
    File(File<'a, U>),
    Message(Message<'a, U>),
}

impl<U> Clone for Container<'_, U> {
    fn clone(&self) -> Self {
        match self {
            Self::File(f) => Self::File(f.clone()),
            Self::Message(m) => Self::Message(m.clone()),
        }
    }
}

impl<'a, U> Container<'a, U> {
    pub fn node(&self) -> Node<'a, U> {
        match self {
            Container::File(f) => Node::File(f.clone()),
            Container::Message(m) => Node::Message(m.clone()),
        }
    }
    pub fn name(&self) -> &Name<U> {
        match self {
            Container::File(f) => &f.name,
            Container::Message(m) => &m.name,
        }
    }
    pub fn messages(&self) -> Iter<Message<'a, U>> {
        match self {
            Container::File(f) => f.messages(),
            Container::Message(m) => m.messages(),
        }
    }
    pub fn all_messages(&self) -> AllMessages<'a, U> {
        match self {
            Container::File(f) => f.all_messages(),
            Container::Message(m) => m.all_messages(),
        }
    }
    pub fn all_enums(&self) -> AllEnums<'a, U> {
        match self {
            Container::File(f) => f.all_enums(),
            Container::Message(m) => m.all_enums(),
        }
    }
    pub fn enums(&self) -> Iter<Enum<'a, U>> {
        match self {
            Container::File(f) => f.enums(),
            Container::Message(m) => m.enums(),
        }
    }
    pub fn package(&self) -> Option<Package<'a, U>> {
        match self {
            Container::File(f) => f.package(),
            Container::Message(m) => m.package(),
        }
    }
    pub(crate) fn downgrade(&self) -> WeakContainer<'a, U> {
        match self {
            Container::File(f) => WeakContainer::File(Rc::downgrade(f)),
            Container::Message(m) => WeakContainer::Message(Rc::downgrade(m)),
        }
    }
}

pub trait BuildTarget {
    fn build_target(&self) -> bool;
}
impl<'a, U> BuildTarget for Container<'a, U> {
    fn build_target(&self) -> bool {
        match self {
            Container::File(f) => f.build_target(),
            Container::Message(m) => m.build_target(),
        }
    }
}

impl<'a, U> BuildTarget for WeakContainer<'a, U> {
    fn build_target(&self) -> bool {
        match self {
            WeakContainer::File(f) => f.upgrade().unwrap().build_target(),
            WeakContainer::Message(m) => m.upgrade().unwrap().build_target(),
        }
    }
}

impl<'a, U> FullyQualified for Container<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            Container::File(f) => f.fully_qualified_name(),
            Container::Message(m) => m.fully_qualified_name(),
        }
    }
}

impl<'a, U> FullyQualified for WeakContainer<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            WeakContainer::File(f) => f.upgrade().unwrap().fully_qualified_name(),
            WeakContainer::Message(m) => m.upgrade().unwrap().fully_qualified_name(),
        }
    }
}
