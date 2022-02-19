use prost_types::FileDescriptorProto;

use crate::container::BuildTarget;
use crate::iter::{AllEnums, AllMessages, TransitiveImports, UpgradeIter};
use crate::util::Generic;
use crate::{Enum, Extension, Message, Name, Node, Package, Service};
use std::cell::RefCell;

use std::path::PathBuf;
// use std::path::PathBuf;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub struct File<U> {
    pub fully_qualified_name: String,
    pub descriptor: Rc<FileDescriptorProto>,
    pub name: Name<U>,
    pub file_path: PathBuf,
    pub build_target: bool,
    pub def_exts: Vec<Rc<Extension<U>>>,
    pub messages: Vec<Rc<Message<U>>>,
    pub enums: Vec<Rc<Enum<U>>>,
    pub services: Vec<Rc<Service<U>>>,
    pub src_info: Option<prost_types::SourceCodeInfo>,
    pub pkg_info: Option<prost_types::SourceCodeInfo>,
    pub util: Rc<RefCell<U>>,
    pub(crate) pkg: Option<Weak<Package<U>>>,
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
        descriptor: Rc<FileDescriptorProto>,
        package: Option<Rc<Package<U>>>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let pkg = package.map(|p| Rc::downgrade(&p));
        let name = Name::new(descriptor.name(), util.clone());
        let fully_qualified_name = match descriptor.package() {
            "" => String::default(),
            p => format!(".{}", p),
        };
        let file_path = PathBuf::from(descriptor.name());
        Rc::new(Self {
            name,
            descriptor,
            pkg,
            fully_qualified_name,
            build_target,
            file_path,
            dependents: Rc::new(RefCell::new(Vec::default())),
            dependencies: Rc::new(RefCell::new(Vec::default())),
            def_exts: Vec::default(),
            messages: Vec::default(),
            enums: Vec::default(),
            services: Vec::default(),
            src_info: None,
            pkg_info: None,
            util,
        })
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
        AllMessages::new(&self.messages)
    }

    /// all_enums returns an iterator of all top-level and nested enums from this file.
    pub fn all_enums(&self) -> AllEnums<U> {
        AllEnums::new(&self.enums, &self.messages)
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
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<U>> {
        todo!()
    }
}

impl Default for File<Generic> {
    fn default() -> Self {
        Self {
            fully_qualified_name: Default::default(),
            descriptor: Rc::new(FileDescriptorProto::default()),
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
