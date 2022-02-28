use prost_types::FileDescriptorProto;

use crate::container::{BuildTarget, Container, WeakContainer};
use crate::iter::{AllEnums, AllMessages, Iter, TransitiveImports, UpgradeIter};
use crate::package::WeakPackage;
use crate::path::FileDescriptorPath;

use crate::traits::{Downgrade, Upgrade};
use crate::{
    Enum, EnumList, Extension, ExtensionList, FullyQualified, Message, MessageList, Name, Node,
    NodeAtPath, Package, Service, ServiceList,
};
use std::cell::RefCell;

use std::ops::Deref;
use std::path::PathBuf;
// use std::path::PathBuf;
use std::rc::{Rc, Weak};
use std::slice;

#[derive(Debug, Clone)]
struct FileDetail<'a, U> {
    desc: &'a FileDescriptorProto,
    name: Name<U>,
    file_path: PathBuf,
    build_target: bool,
    pkg_info: Option<prost_types::SourceCodeInfo>,
    src_info: Option<prost_types::SourceCodeInfo>,
    util: Rc<RefCell<U>>,
    fqn: String,
    pkg: Option<WeakPackage<'a, U>>,
    messages: MessageList<'a, U>,
    enums: EnumList<'a, U>,
    services: ServiceList<'a, U>,
    dependents: Rc<RefCell<Vec<WeakFile<'a, U>>>>,
    dependencies: Rc<RefCell<Vec<WeakFile<'a, U>>>>,
    defined_extensions: ExtensionList<'a, U>,
}

#[derive(Debug)]
pub struct File<'a, U>(Rc<FileDetail<'a, U>>);
impl<'a, U> Clone for File<'a, U> {
    fn clone(&self) -> Self {
        File(self.0.clone())
    }
}

impl<'a, U> BuildTarget for File<'a, U> {
    fn build_target(&self) -> bool {
        self.0.build_target
    }
}

impl<'a, U> File<'a, U> {
    pub(crate) fn new(
        build_target: bool,
        desc: &'a FileDescriptorProto,
        pkg: Option<Package<'a, U>>,
        util: Rc<RefCell<U>>,
    ) -> Self {
        let name = Name::new(desc.name(), util.clone());
        let fqn = match desc.package() {
            "" => String::default(),
            p => format!(".{}", p),
        };

        let file = Self(Rc::new(FileDetail {
            name,
            desc,
            util: util.clone(),
            pkg: Package::downgrade(pkg),
            build_target,
            fqn,
            file_path: PathBuf::from(desc.name()),
            dependents: Rc::new(RefCell::new(Vec::new())),
            dependencies: Rc::new(RefCell::new(Vec::with_capacity(desc.dependency.len()))),
            defined_extensions: Rc::new(RefCell::new(Vec::with_capacity(desc.extension.len()))),
            messages: Rc::new(RefCell::new(Vec::with_capacity(desc.message_type.len()))),
            enums: Rc::new(RefCell::new(Vec::with_capacity(desc.enum_type.len()))),
            services: Rc::new(RefCell::new(Vec::with_capacity(desc.service.len()))),
            src_info: None,
            pkg_info: None,
        }));

        let container: Container<'a, U> = file.into();

        let mut msgs = file.0.messages.borrow_mut();
        for md in desc.message_type.iter() {
            let msg = Message::new(md, container.clone(), util.clone());
            msgs.push(msg);
        }

        let mut enums = file.0.enums.borrow_mut();
        for ed in desc.enum_type.iter() {
            let e = Enum::new(ed, container.clone(), util.clone());
            enums.push(e);
        }

        let mut services = file.0.services.borrow_mut();
        for sd in desc.service.iter() {
            let svc = Service::new(sd, container.clone(), util.clone());
            services.push(svc);
        }

        let mut exts = file.0.defined_extensions.borrow_mut();
        for ed in desc.extension.iter() {
            let ext = Extension::new(ed, container.clone(), util.clone());
            exts.push(ext);
        }

        file
    }
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }

    pub fn messages(&self) -> slice::Iter<Message<'a, U>> {
        self.0.messages.borrow().iter()
    }
    pub fn enums(&self) -> Iter<Enum<'a, U>> {
        Iter::from(&self.0.enums)
    }
    pub fn services(&self) -> Iter<Service<'a, U>> {
        Iter::from(&self.0.services)
    }
    pub fn defined_extensions(&self) -> Iter<Extension<'a, U>> {
        Iter::from(&self.0.defined_extensions)
    }

    pub fn imports(&self) -> UpgradeIter<File<'a, U>> {
        UpgradeIter::new(self.0.dependencies.clone())
    }
    pub fn dependents(&self) -> UpgradeIter<File<'a, U>> {
        UpgradeIter::new(self.0.dependents.clone())
    }
    pub fn dependencies(&self) -> UpgradeIter<File<'a, U>> {
        UpgradeIter::new(self.0.dependencies.clone())
    }

    pub fn transitive_imports(&self) -> TransitiveImports<'a, U> {
        TransitiveImports::new(self.0.dependencies.clone())
    }

    /// all_messages returns an iterator of all top-level and nested messages from this
    /// file.
    pub fn all_messages(&self) -> AllMessages<'a, U> {
        AllMessages::new(self.0.messages.clone())
    }

    /// all_enums returns an iterator of all top-level and nested enums from this file.
    pub fn all_enums(&self) -> AllEnums<'a, U> {
        AllEnums::new(self.0.enums.clone(), self.0.messages.clone())
    }

    pub fn file_path(&self) -> &PathBuf {
        &self.0.file_path
    }
    pub fn package(&self) -> Option<Package<'a, U>> {
        self.0.pkg.clone().map(|p| p.upgrade())
    }

    pub(crate) fn add_dependency(&self, file: File<'a, U>) {
        self.0.dependencies.borrow_mut().push(file.downgrade());
    }

    pub(crate) fn add_dependent(&self, file: File<'a, U>) {
        self.0.dependents.borrow_mut().push(file.downgrade());
    }
}
impl<'a, U> Downgrade for File<'a, U> {
    type Target = WeakFile<'a, U>;
    fn downgrade(self) -> Self::Target {
        WeakFile(Rc::downgrade(&self.0))
    }
}

impl<'a, U> Into<Node<'a, U>> for File<'a, U> {
    fn into(self) -> Node<'a, U> {
        Node::File(self)
    }
}
impl<'a, U> Into<Node<'a, U>> for &File<'a, U> {
    fn into(self) -> Node<'a, U> {
        Node::File(self.clone())
    }
}

impl<'a, U> NodeAtPath<'a, U> for File<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            return Some(self.into());
        }
        if path.len() % 2 == 1 {
            return None;
        }
        let next = path[1] as usize;
        FileDescriptorPath::try_from(path[0]).ok().and_then(|p| {
            match p {
                FileDescriptorPath::MessageType => self
                    .0
                    .messages
                    .borrow()
                    .get(next)
                    .cloned()
                    .map(Node::Message),
                FileDescriptorPath::EnumType => {
                    self.0.enums.borrow().get(next).cloned().map(Node::Enum)
                }
                FileDescriptorPath::Service => self
                    .0
                    .services
                    .borrow()
                    .get(next)
                    .cloned()
                    .map(Node::Service),
                _ => None,
            }
            .and_then(|n| n.node_at_path(&path[2..]))
        })
    }
}

impl<'a, U> FullyQualified for File<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
    }
}

impl<'a, U> Into<Container<'a, U>> for File<'a, U> {
    fn into(self) -> Container<'a, U> {
        Container::File(self)
    }
}
impl<'a, U> Into<WeakContainer<'a, U>> for File<'a, U> {
    fn into(self) -> WeakContainer<'a, U> {
        WeakContainer::File(self.downgrade())
    }
}

impl<'a, U> Deref for File<'a, U> {
    type Target = Node<'a, U>;
    fn deref(&self) -> &Self::Target {
        &Node::File(File(self.0.clone()))
    }
}

#[derive(Debug)]
pub(crate) struct WeakFile<'a, U>(Weak<FileDetail<'a, U>>);

impl<'a, U> WeakFile<'a, U> {
    pub(crate) fn package(&self) -> Option<Package<'a, U>> {
        self.upgrade().package()
    }
}

impl<'a, U> Upgrade for WeakFile<'a, U> {
    type Target = File<'a, U>;
    fn upgrade(self) -> Self::Target {
        File(self.0.upgrade().expect("File was dropped"))
    }
}

impl<'a, U> Clone for WeakFile<'a, U> {
    fn clone(&self) -> Self {
        WeakFile(self.0.clone())
    }
}
