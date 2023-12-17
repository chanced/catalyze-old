use crate::Extension;
use crate::Iter;
use crate::Syntax;
use crate::WeakFile;
use crate::{AllEnums, AllMessages, Enum, File, Message, Node, Package, WeakMessage};

// pub enum Entity {

// }

#[derive(Debug, Clone)]
pub enum Container {
    File(File),
    Message(Message),
}

impl Container {
    pub fn node(&self) -> Node {
        match self {
            Container::File(f) => Node::File(f.clone()),
            Container::Message(m) => Node::Message(m.clone()),
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Container::File(f) => f.name(),
            Container::Message(m) => m.name(),
        }
    }
    pub fn messages(&self) -> Iter<Message> {
        match self {
            Container::File(f) => f.messages(),
            Container::Message(m) => m.messages(),
        }
    }
    pub fn all_messages(&self) -> AllMessages {
        match self {
            Container::File(f) => f.all_messages(),
            Container::Message(m) => m.all_messages(),
        }
    }
    pub fn all_enums(&self) -> AllEnums {
        match self {
            Container::File(f) => f.all_enums(),
            Container::Message(m) => m.all_enums(),
        }
    }
    pub fn enums(&self) -> Iter<Enum> {
        match self {
            Container::File(f) => f.enums(),
            Container::Message(m) => m.enums(),
        }
    }
    pub fn package(&self) -> Package {
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
    pub fn defined_extensions(&self) -> Iter<Extension> {
        match self {
            Container::File(f) => f.defined_extensions(),
            Container::Message(m) => m.defined_extensions(),
        }
    }
    pub fn fully_qualified_name(&self) -> &str {
        match self {
            Container::File(f) => f.fully_qualified_name(),
            Container::Message(m) => m.fully_qualified_name(),
        }
    }
    pub(crate) fn register_import(&self, arg: File) {
        match self {
            Container::File(f) => f.mark_import_as_used(arg),
            Container::Message(m) => m.register_import(arg),
        }
    }
}

impl From<File> for Container {
    fn from(f: File) -> Self {
        Container::File(f)
    }
}
impl From<&File> for Container {
    fn from(f: &File) -> Self {
        Container::File(f.clone())
    }
}
impl From<Message> for Container {
    fn from(f: Message) -> Self {
        Container::Message(f)
    }
}
impl From<&Message> for Container {
    fn from(f: &Message) -> Self {
        Container::Message(f.clone())
    }
}
impl From<WeakContainer> for Container {
    fn from(f: WeakContainer) -> Self {
        match f {
            WeakContainer::File(f) => Container::File(f.into()),
            WeakContainer::Message(m) => Container::Message(m.into()),
        }
    }
}
impl From<&WeakContainer> for Container {
    fn from(f: &WeakContainer) -> Self {
        match f {
            WeakContainer::File(f) => Container::File(f.into()),
            WeakContainer::Message(m) => Container::Message(m.clone().into()),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum WeakContainer {
    File(WeakFile),
    Message(WeakMessage),
}

impl WeakContainer {
    pub fn package(&self) -> Package {
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
    pub fn file(&self) -> File {
        match self {
            WeakContainer::File(f) => f.into(),
            WeakContainer::Message(m) => m.file(),
        }
    }

    pub(crate) fn weak_file(&self) -> WeakFile {
        match self {
            WeakContainer::File(f) => f.clone(),
            WeakContainer::Message(m) => m.weak_file(),
        }
    }
}
impl From<&File> for WeakContainer {
    fn from(f: &File) -> Self {
        WeakContainer::File(f.into())
    }
}
impl From<File> for WeakContainer {
    fn from(f: File) -> Self {
        WeakContainer::File(f.into())
    }
}
impl From<&Message> for WeakContainer {
    fn from(m: &Message) -> Self {
        WeakContainer::Message(m.into())
    }
}
impl From<Message> for WeakContainer {
    fn from(m: Message) -> Self {
        WeakContainer::Message(m.into())
    }
}

impl From<Container> for WeakContainer {
    fn from(c: Container) -> Self {
        match c {
            Container::File(f) => f.into(),
            Container::Message(m) => m.into(),
        }
    }
}
impl From<&Container> for WeakContainer {
    fn from(c: &Container) -> Self {
        match c {
            Container::File(f) => f.into(),
            Container::Message(m) => m.into(),
        }
    }
}
