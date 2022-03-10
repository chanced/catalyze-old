use std::rc::Rc;

use crate::file::WeakFile;
use crate::iter::Iter;
use crate::{
    AllEnums, AllMessages, Enum, File, FullyQualified, Message, Name, Node, Package, WeakMessage,
};

// pub enum Entity {

// }

#[derive(Debug)]
pub enum Container<'a, U> {
    File(File<'a, U>),
    Message(Message<'a, U>),
}

impl<'a, U> Clone for Container<'a, U> {
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
    pub fn name(&self) -> Name<U> {
        match self {
            Container::File(f) => f.name(),
            Container::Message(m) => m.name(),
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
    pub fn package(&self) -> Package<'a, U> {
        match self {
            Container::File(f) => f.package(),
            Container::Message(m) => m.package(),
        }
    }
    pub fn util(&self) -> Rc<U> {
        match self {
            Container::File(f) => f.util(),
            Container::Message(m) => m.util(),
        }
    }
}

impl<'a, U> From<File<'a, U>> for Container<'a, U> {
    fn from(f: File<'a, U>) -> Self {
        Container::File(f)
    }
}
impl<'a, U> From<&File<'a, U>> for Container<'a, U> {
    fn from(f: &File<'a, U>) -> Self {
        Container::File(f.clone())
    }
}
impl<'a, U> From<Message<'a, U>> for Container<'a, U> {
    fn from(f: Message<'a, U>) -> Self {
        Container::Message(f)
    }
}
impl<'a, U> From<&Message<'a, U>> for Container<'a, U> {
    fn from(f: &Message<'a, U>) -> Self {
        Container::Message(f.clone())
    }
}
impl<'a, U> From<WeakContainer<'a, U>> for Container<'a, U> {
    fn from(f: WeakContainer<'a, U>) -> Self {
        match f {
            WeakContainer::File(f) => Container::File(f.into()),
            WeakContainer::Message(m) => Container::Message(m.into()),
        }
    }
}
impl<'a, U> From<&WeakContainer<'a, U>> for Container<'a, U> {
    fn from(f: &WeakContainer<'a, U>) -> Self {
        match f {
            WeakContainer::File(f) => Container::File(f.into()),
            WeakContainer::Message(m) => Container::Message(m.clone().into()),
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

#[derive(Debug)]
pub(crate) enum WeakContainer<'a, U> {
    File(WeakFile<'a, U>),
    Message(WeakMessage<'a, U>),
}

impl<'a, U> WeakContainer<'a, U> {
    pub fn package(&self) -> Package<'a, U> {
        match self {
            WeakContainer::File(f) => f.package(),
            WeakContainer::Message(m) => m.package(),
        }
    }
    pub fn build_target(&self) -> bool {
        match self {
            WeakContainer::File(f) => f.build_target(),
            WeakContainer::Message(m) => m.build_target(),
        }
    }
    pub fn file(&self) -> File<'a, U> {
        match self {
            WeakContainer::File(f) => f.into(),
            WeakContainer::Message(m) => m.file(),
        }
    }

    pub(crate) fn weak_file(&self) -> WeakFile<'a, U> {
        match self {
            WeakContainer::File(f) => f.clone(),
            WeakContainer::Message(m) => m.weak_file(),
        }
    }
}
impl<'a, U> From<&File<'a, U>> for WeakContainer<'a, U> {
    fn from(f: &File<'a, U>) -> Self {
        WeakContainer::File(f.into())
    }
}
impl<'a, U> From<File<'a, U>> for WeakContainer<'a, U> {
    fn from(f: File<'a, U>) -> Self {
        WeakContainer::File(f.into())
    }
}
impl<'a, U> From<&Message<'a, U>> for WeakContainer<'a, U> {
    fn from(m: &Message<'a, U>) -> Self {
        WeakContainer::Message(m.into())
    }
}
impl<'a, U> From<Message<'a, U>> for WeakContainer<'a, U> {
    fn from(m: Message<'a, U>) -> Self {
        WeakContainer::Message(m.into())
    }
}

impl<'a, U> From<Container<'a, U>> for WeakContainer<'a, U> {
    fn from(c: Container<'a, U>) -> Self {
        match c {
            Container::File(f) => f.into(),
            Container::Message(m) => m.into(),
        }
    }
}
impl<'a, U> From<&Container<'a, U>> for WeakContainer<'a, U> {
    fn from(c: &Container<'a, U>) -> Self {
        match c {
            Container::File(f) => f.into(),
            Container::Message(m) => m.into(),
        }
    }
}
impl<'a, U> FullyQualified for WeakContainer<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            WeakContainer::File(f) => f.fully_qualified_name(),
            WeakContainer::Message(m) => m.fully_qualified_name(),
        }
    }
}
impl<'a, U> Clone for WeakContainer<'a, U> {
    fn clone(&self) -> Self {
        match self {
            Self::File(arg0) => Self::File(arg0.clone()),
            Self::Message(arg0) => Self::Message(arg0.clone()),
        }
    }
}
