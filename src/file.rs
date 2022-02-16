use crate::container::BuildTarget;
use crate::iter::{AllEnums, AllMessages, Iter};
use crate::lang::{Lang, Unspecified};
use crate::{
    Enum, EnumList, Extension, ExtensionList, Message, MessageList, Name, Package, Service,
    ServiceList,
};
use std::cell::RefCell;

use std::path::PathBuf;
// use std::path::PathBuf;
use std::rc::{Rc, Weak};

pub(crate) type WeakFileList<L> = Rc<RefCell<Vec<Weak<File<L>>>>>;

pub(crate) type FileList<L> = Rc<RefCell<Vec<Rc<File<L>>>>>;

#[derive(Debug, Clone)]
pub struct File<L: Lang> {
    pub fully_qualified_name: String,
    pub descriptor: prost_types::FileDescriptorProto,
    pub name: Name<L>,
    pub file_path: PathBuf,
    pub build_target: bool,
    pub(crate) pkg: Option<Weak<Package<L>>>,
    pub(crate) dependents: WeakFileList<L>,
    pub(crate) dependencies: WeakFileList<L>,
    pub(crate) exts: ExtensionList<L>,
    pub(crate) messages: MessageList<L>,
    pub(crate) enums: EnumList<L>,
    pub(crate) services: ServiceList<L>,
    pub(crate) src_info: Rc<RefCell<Option<Rc<prost_types::SourceCodeInfo>>>>,
    pub(crate) pkg_info: Rc<RefCell<Option<Rc<prost_types::SourceCodeInfo>>>>,
}

impl Default for File<Unspecified> {
    fn default() -> Self {
        Self {
            fully_qualified_name: Default::default(),
            descriptor: Default::default(),
            name: Name::<Unspecified>::default(),
            file_path: Default::default(),
            build_target: Default::default(),
            pkg: Default::default(),
            dependents: Default::default(),
            dependencies: Default::default(),
            exts: Default::default(),
            messages: Default::default(),
            enums: Default::default(),
            services: Default::default(),
            src_info: Default::default(),
            pkg_info: Default::default(),
        }
    }
}

impl<L: Lang> BuildTarget for File<L> {
    fn build_target(&self) -> bool {
        self.build_target
    }
}

impl<L: Lang> File<L> {
    pub(crate) fn new(
        build_target: bool,
        descriptor: prost_types::FileDescriptorProto,
        package: Option<Rc<Package<L>>>,
        lang: L,
    ) -> Self {
        let pkg = package.map(|p| Rc::downgrade(&p));

        let name = Name::new(descriptor.name(), lang);
        let fully_qualified_name = match descriptor.package() {
            "" => String::from(""),
            p => format!(".{}", p),
        };
        let file_path = PathBuf::from(descriptor.name());
        Self {
            name,
            descriptor,
            pkg,
            fully_qualified_name,
            build_target,
            file_path,
            dependents: Rc::new(RefCell::new(Vec::new())),
            dependencies: Rc::new(RefCell::new(Vec::new())),
            exts: Rc::new(RefCell::new(Vec::new())),
            messages: Rc::new(RefCell::new(Vec::new())),
            enums: Rc::new(RefCell::new(Vec::new())),
            services: Rc::new(RefCell::new(Vec::new())),
            src_info: Rc::new(RefCell::new(None)),
            pkg_info: Rc::new(RefCell::new(None)),
        }
    }
    /// all_messages returns an iterator of all top-level and nested messages from this
    /// file.
    pub fn all_messages(&self) -> AllMessages<L> {
        AllMessages::new(self.messages.clone())
    }

    pub fn enums(&self) -> Iter<Enum<L>> {
        Iter::new(self.enums.clone())
    }

    /// all_enums returns an iterator of all top-level and nested enums from this file.
    pub fn all_enums(&self) -> AllEnums<L> {
        AllEnums::new(self.enums.clone(), self.messages.clone())
    }

    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }
    pub fn package(&self) -> Option<Rc<Package<L>>> {
        self.pkg.clone().map(|p| p.upgrade().unwrap())
    }
    pub fn package_source_code_info(&self) -> Option<Rc<prost_types::SourceCodeInfo>> {
        self.pkg_info.borrow().clone()
    }
    pub fn source_code_info(&self) -> Option<Rc<prost_types::SourceCodeInfo>> {
        self.src_info.borrow().clone()
    }

    /// Returns top-level messages for this Message. Nested messages are not
    /// included.
    pub fn messages(&self) -> Iter<Message<L>> {
        Iter::new(self.messages.clone())
    }

    pub fn services(&self) -> Vec<Rc<Service<L>>> {
        self.services.borrow().iter().cloned().collect()
    }
    pub fn extensions(&self) -> Vec<Rc<Extension<L>>> {
        self.exts.borrow().iter().cloned().collect()
    }
    pub fn dependencies(&self) -> Vec<Rc<File<L>>> {
        self.dependencies
            .borrow()
            .iter()
            .map(|d| d.upgrade().unwrap())
            .collect()
    }
    pub fn dependents(&self) -> Vec<Rc<File<L>>> {
        self.dependents
            .borrow()
            .iter()
            .map(|d| d.upgrade().unwrap())
            .collect()
    }

    pub(crate) fn add_enum(&self, e: Rc<Enum<L>>) {
        self.enums.borrow_mut().push(e);
    }
    pub(crate) fn add_dependency(&self, file: Rc<File<L>>) {
        self.dependencies.borrow_mut().push(Rc::downgrade(&file));
    }
    pub(crate) fn add_dependent(&self, file: Rc<File<L>>) {
        self.dependents.borrow_mut().push(Rc::downgrade(&file));
    }
    pub(crate) fn add_extension(&self, ext: Rc<Extension<L>>) {
        self.exts.borrow_mut().push(ext);
    }
    pub(crate) fn add_message(&self, msg: Rc<Message<L>>) {
        self.messages.borrow_mut().push(msg);
    }
    pub(crate) fn add_service(&self, service: Rc<Service<L>>) {
        self.services.borrow_mut().push(service);
    }
    pub(crate) fn set_src_info(&self, src_info: prost_types::SourceCodeInfo) {
        self.src_info.replace(Some(Rc::new(src_info)));
    }
    pub(crate) fn set_pkg_info(&self, pkg_info: prost_types::SourceCodeInfo) {
        self.pkg_info.replace(Some(Rc::new(pkg_info)));
    }
}
