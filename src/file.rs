use prost_types::FileDescriptorProto;

use crate::container::{BuildTarget, Container};
use crate::iter::{Iter, TransitiveImports, UpgradeIter};
use crate::path::FileDescriptorPath;
use crate::util::Generic;
use crate::{
    AllEnums, AllMessages, Enum, EnumList, Extension, ExtensionList, Message, MessageList, Name,
    Node, NodeAtPath, Package, Service, ServiceList,
};
use std::cell::RefCell;

use std::path::PathBuf;
// use std::path::PathBuf;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub struct File<U> {
    pub fully_qualified_name: String,
    pub descriptor: FileDescriptorProto,
    pub name: Name<U>,
    pub file_path: PathBuf,
    pub build_target: bool,
    pub pkg_info: Option<prost_types::SourceCodeInfo>,
    pub src_info: Option<prost_types::SourceCodeInfo>,
    pub util: Rc<RefCell<U>>,
    pub(crate) pkg: Option<Weak<Package<U>>>,
    pub(crate) def_exts: ExtensionList<U>,
    pub(crate) messages: MessageList<U>,
    pub(crate) enums: EnumList<U>,
    pub(crate) services: ServiceList<U>,
    pub(crate) dependents: Rc<RefCell<Vec<Weak<File<U>>>>>,
    pub(crate) dependencies: Rc<RefCell<Vec<Weak<File<U>>>>>,
}

impl<U> BuildTarget for File<U> {
    fn build_target(&self) -> bool {
        self.build_target
    }
}

impl<U> File<U> {
    pub(crate) fn new(
        build_target: bool,
        desc: FileDescriptorProto,
        package: Option<Rc<Package<U>>>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let pkg = package.map(|p| Rc::downgrade(&p));
        let name = Name::new(desc.name(), util.clone());
        let fully_qualified_name = match desc.package() {
            "" => String::default(),
            p => format!(".{}", p),
        };
        let file_path = PathBuf::from(desc.name());

        let file = Rc::new(Self {
            name,
            util,
            descriptor: desc.clone(),
            pkg,
            fully_qualified_name,
            build_target,
            file_path,
            dependents: Rc::new(RefCell::new(Vec::new())),
            dependencies: Rc::new(RefCell::new(Vec::with_capacity(desc.dependency.len()))),
            def_exts: Rc::new(RefCell::new(Vec::with_capacity(desc.extension.len()))),
            messages: Rc::new(RefCell::new(Vec::with_capacity(desc.message_type.len()))),
            enums: Rc::new(RefCell::new(Vec::with_capacity(desc.enum_type.len()))),
            services: Rc::new(RefCell::new(Vec::with_capacity(desc.service.len()))),
            src_info: None,
            pkg_info: None,
        });
        file.hydrate();
        // for ed in desc.enum_type.iter().cloned() {
        //     let msg = Enum::new(ed, Container::File(file.clone()), util.clone());
        //     *(file.enums).borrow_mut().push(msg.clone());
        // }

        file
    }
    pub fn messages(&self) -> Iter<Message<U>> {
        Iter::from(&self.messages)
    }
    pub fn enums(&self) -> Iter<Enum<U>> {
        Iter::from(&self.enums)
    }
    pub fn services(&self) -> Iter<Service<U>> {
        Iter::from(&self.services)
    }
    pub fn extensions(&self) -> Iter<Extension<U>> {
        Iter::from(&self.def_exts)
    }
    pub fn imports(&self) -> UpgradeIter<File<U>> {
        UpgradeIter::new(self.dependencies.clone())
    }
    pub fn dependents(&self) -> UpgradeIter<File<U>> {
        UpgradeIter::new(self.dependents.clone())
    }
    pub fn dependencies(&self) -> UpgradeIter<File<U>> {
        UpgradeIter::new(self.dependencies.clone())
    }

    pub fn transitive_imports(&self) -> TransitiveImports<U> {
        TransitiveImports::new(self.dependencies.clone())
    }

    /// all_messages returns an iterator of all top-level and nested messages from this
    /// file.
    pub fn all_messages(&self) -> AllMessages<U> {
        AllMessages::new(self.messages.clone())
    }

    /// all_enums returns an iterator of all top-level and nested enums from this file.
    pub fn all_enums(&self) -> AllEnums<U> {
        AllEnums::new(self.enums.clone(), self.messages.clone())
    }

    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }
    pub fn package(&self) -> Option<Rc<Package<U>>> {
        self.pkg.clone().map(|p| p.upgrade().unwrap())
    }

    pub(crate) fn add_dependency(&self, file: Rc<File<U>>) {
        self.dependencies.borrow_mut().push(Rc::downgrade(&file));
    }

    pub(crate) fn add_dependent(&self, file: Rc<File<U>>) {
        self.dependents.borrow_mut().push(Rc::downgrade(&file));
    }
}
trait Hydrate<U> {
    fn hydrate(&self);
    fn hydrate_messages(&self);
    fn hydrate_enums(&self);
    fn hydrate_services(&self);
    fn hydrate_extensions(&self);
}

impl<U> Hydrate<U> for Rc<File<U>> {
    fn hydrate(&self) {
        self.hydrate_messages();
        self.hydrate_enums();
        self.hydrate_services();
        self.hydrate_extensions();
    }
    fn hydrate_messages(&self) {
        let file = self.clone();
        let mut msgs = file.messages.borrow_mut();
        for md in file.descriptor.message_type.iter().cloned() {
            let msg = Message::new(md, Container::File(file.clone()), file.util.clone());
            msgs.push(msg);
        }
    }
    fn hydrate_enums(&self) {
        let file = self.clone();
        let mut enums = file.enums.borrow_mut();
        for ed in file.descriptor.enum_type.iter().cloned() {
            let e = Enum::new(ed, Container::File(file.clone()), file.util.clone());
            enums.push(e);
        }
    }

    fn hydrate_services(&self) {
        let file = self.clone();
        let mut services = file.services.borrow_mut();
        for sd in file.descriptor.service.iter().cloned() {
            let svc = Service::new(sd, Container::File(file.clone()), file.util.clone());
            services.push(svc);
        }
    }

    fn hydrate_extensions(&self) {
        let file = self.clone();
        let mut exts = file.def_exts.borrow_mut();
        for ed in file.descriptor.extension.iter().cloned() {
            let ext = Extension::new(ed, file.clone(), file.util.clone());
            exts.push(ext);
        }
    }
}

impl<U> NodeAtPath<U> for Rc<File<U>> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<U>> {
        if path.is_empty() {
            return Some(Node::File(self.clone()));
        }
        if path.len() % 2 == 1 {
            return None;
        }
        let next = path[1] as usize;
        FileDescriptorPath::try_from(path[0]).ok().and_then(|p| {
            match p {
                FileDescriptorPath::MessageType => {
                    self.messages.borrow().get(next).cloned().map(Node::Message)
                }
                FileDescriptorPath::EnumType => {
                    self.enums.borrow().get(next).cloned().map(Node::Enum)
                }
                FileDescriptorPath::Service => {
                    self.services.borrow().get(next).cloned().map(Node::Service)
                }
                _ => None,
            }
            .and_then(|n| n.node_at_path(&path[2..]))
        })
    }
}

impl Default for File<Generic> {
    fn default() -> Self {
        Self {
            fully_qualified_name: Default::default(),
            descriptor: FileDescriptorProto::default(),
            name: Name::<Generic>::default(),
            file_path: Default::default(),
            build_target: Default::default(),
            pkg: Default::default(),
            dependents: Default::default(),
            dependencies: Default::default(),
            def_exts: Default::default(),
            messages: Default::default(),
            enums: Default::default(),
            services: Default::default(),
            src_info: Default::default(),
            pkg_info: Default::default(),
            util: Rc::new(RefCell::new(Generic)),
        }
    }
}
