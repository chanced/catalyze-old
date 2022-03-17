use crate::Iter;
use crate::Syntax;
use crate::WeakFile;
use crate::{AllEnums, AllMessages, Enum, File, Message, Name, Node, Package, WeakMessage};

// pub enum Entity {

// }

#[derive(Debug, Clone)]
pub enum Container<'a> {
    File(File<'a>),
    Message(Message<'a>),
}

impl<'a> Container<'a> {
    pub fn node(&self) -> Node<'a> {
        match self {
            Container::File(f) => Node::File(f.clone()),
            Container::Message(m) => Node::Message(m.clone()),
        }
    }
    pub fn name(&self) -> &Name {
        match self {
            Container::File(f) => f.name(),
            Container::Message(m) => m.name(),
        }
    }
    pub fn messages(&self) -> Iter<Message<'a>> {
        match self {
            Container::File(f) => f.messages(),
            Container::Message(m) => m.messages(),
        }
    }
    pub fn all_messages(&self) -> AllMessages<'a> {
        match self {
            Container::File(f) => f.all_messages(),
            Container::Message(m) => m.all_messages(),
        }
    }
    pub fn all_enums(&self) -> AllEnums<'a> {
        match self {
            Container::File(f) => f.all_enums(),
            Container::Message(m) => m.all_enums(),
        }
    }
    pub fn enums(&self) -> Iter<Enum<'a>> {
        match self {
            Container::File(f) => f.enums(),
            Container::Message(m) => m.enums(),
        }
    }
    pub fn package(&self) -> Package<'a> {
        match self {
            Container::File(f) => f.package(),
            Container::Message(m) => m.package(),
        }
    }

    pub fn syntax(&self) -> Syntax {
        match self {
            Container::File(f) => f.syntax(),
            Container::Message(m) => m.syntax(),
        }
    }

    pub fn fully_qualified_name(&self) -> String {
        match self {
            Container::File(f) => f.fully_qualified_name(),
            Container::Message(m) => m.fully_qualified_name(),
        }
    }
    pub(crate) fn register_import(&self, arg: File<'a>) {
        match self {
            Container::File(f) => f.mark_import_as_used(arg),
            Container::Message(m) => m.register_import(arg),
        }
    }
}

impl<'a> From<File<'a>> for Container<'a> {
    fn from(f: File<'a>) -> Self {
        Container::File(f)
    }
}
impl<'a> From<&File<'a>> for Container<'a> {
    fn from(f: &File<'a>) -> Self {
        Container::File(f.clone())
    }
}
impl<'a> From<Message<'a>> for Container<'a> {
    fn from(f: Message<'a>) -> Self {
        Container::Message(f)
    }
}
impl<'a> From<&Message<'a>> for Container<'a> {
    fn from(f: &Message<'a>) -> Self {
        Container::Message(f.clone())
    }
}
impl<'a> From<WeakContainer<'a>> for Container<'a> {
    fn from(f: WeakContainer<'a>) -> Self {
        match f {
            WeakContainer::File(f) => Container::File(f.into()),
            WeakContainer::Message(m) => Container::Message(m.into()),
        }
    }
}
impl<'a> From<&WeakContainer<'a>> for Container<'a> {
    fn from(f: &WeakContainer<'a>) -> Self {
        match f {
            WeakContainer::File(f) => Container::File(f.into()),
            WeakContainer::Message(m) => Container::Message(m.clone().into()),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum WeakContainer<'a> {
    File(WeakFile<'a>),
    Message(WeakMessage<'a>),
}

impl<'a> WeakContainer<'a> {
    pub fn package(&self) -> Package<'a> {
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
    pub fn file(&self) -> File<'a> {
        match self {
            WeakContainer::File(f) => f.into(),
            WeakContainer::Message(m) => m.file(),
        }
    }

    pub(crate) fn weak_file(&self) -> WeakFile<'a> {
        match self {
            WeakContainer::File(f) => f.clone(),
            WeakContainer::Message(m) => m.weak_file(),
        }
    }
}
impl<'a> From<&File<'a>> for WeakContainer<'a> {
    fn from(f: &File<'a>) -> Self {
        WeakContainer::File(f.into())
    }
}
impl<'a> From<File<'a>> for WeakContainer<'a> {
    fn from(f: File<'a>) -> Self {
        WeakContainer::File(f.into())
    }
}
impl<'a> From<&Message<'a>> for WeakContainer<'a> {
    fn from(m: &Message<'a>) -> Self {
        WeakContainer::Message(m.into())
    }
}
impl<'a> From<Message<'a>> for WeakContainer<'a> {
    fn from(m: Message<'a>) -> Self {
        WeakContainer::Message(m.into())
    }
}

impl<'a> From<Container<'a>> for WeakContainer<'a> {
    fn from(c: Container<'a>) -> Self {
        match c {
            Container::File(f) => f.into(),
            Container::Message(m) => m.into(),
        }
    }
}
impl<'a> From<&Container<'a>> for WeakContainer<'a> {
    fn from(c: &Container<'a>) -> Self {
        match c {
            Container::File(f) => f.into(),
            Container::Message(m) => m.into(),
        }
    }
}
