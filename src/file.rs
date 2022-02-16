use crate::container::BuildTarget;
use crate::iter::{AllEnums, AllMessages, Iter, TransitiveImports, UpgradeIter};
use crate::util::{Lang, Unspecified};
use crate::{
    Enum, EnumList, Extension, ExtensionList, Message, MessageList, Name, Package, Service,
    ServiceList,
};
use std::cell::RefCell;

use std::path::PathBuf;
// use std::path::PathBuf;
use std::rc::{Rc, Weak};

pub(crate) type WeakFileList<U> = Rc<RefCell<Vec<Weak<File<U>>>>>;

pub(crate) type FileList<U> = Rc<RefCell<Vec<Rc<File<U>>>>>;

pub(crate) fn new_file_list<U>() -> FileList<U> {
    Rc::new(RefCell::new(Vec::default()))
}

#[derive(Debug, Clone)]
pub struct File<U> {
    pub fully_qualified_name: String,
    pub descriptor: prost_types::FileDescriptorProto,
    pub name: Name<U>,
    pub file_path: PathBuf,
    pub build_target: bool,
    pub(crate) pkg: Option<Weak<Package<U>>>,
    pub(crate) dependents: WeakFileList<U>,
    pub(crate) imports: WeakFileList<U>,
    pub(crate) def_exts: ExtensionList<U>,
    pub(crate) messages: MessageList<U>,
    pub(crate) enums: EnumList<U>,
    pub(crate) services: ServiceList<U>,
    pub(crate) src_info: Rc<RefCell<Option<Rc<prost_types::SourceCodeInfo>>>>,
    pub(crate) pkg_info: Rc<RefCell<Option<Rc<prost_types::SourceCodeInfo>>>>,
}

impl<U> BuildTarget for File<U> {
    fn build_target(&self) -> bool {
        self.build_target
    }
}

impl<U> File<U> {
    pub(crate) fn new(
        build_target: bool,
        descriptor: prost_types::FileDescriptorProto,
        package: Option<Rc<Package<U>>>,
        lang: U,
    ) -> Rc<Self> {
        let pkg = package.clone().map(|p| Rc::downgrade(&p));
        let name = Name::new(descriptor.name(), lang);
        let fully_qualified_name = match descriptor.package() {
            "" => String::from(""),
            p => format!(".{}", p),
        };
        let file_path = PathBuf::from(descriptor.name());
        let f = Rc::new(Self {
            name,
            descriptor,
            pkg,
            fully_qualified_name,
            build_target,
            file_path,
            dependents: Rc::new(RefCell::new(Vec::new())),
            imports: Rc::new(RefCell::new(Vec::new())),
            def_exts: Rc::new(RefCell::new(Vec::new())),
            messages: Rc::new(RefCell::new(Vec::new())),
            enums: Rc::new(RefCell::new(Vec::new())),
            services: Rc::new(RefCell::new(Vec::new())),
            src_info: Rc::new(RefCell::new(None)),
            pkg_info: Rc::new(RefCell::new(None)),
        });

        if let Some(p) = package {
            p.add_file(f.clone());
        }
        f
    }

    pub fn transitive_imports(&self) -> TransitiveImports<U> {
        TransitiveImports::new(self.imports.clone())
    }

    /// all_messages returns an iterator of all top-level and nested messages from this
    /// file.
    pub fn all_messages(&self) -> AllMessages<U> {
        AllMessages::new(self.messages.clone())
    }

    pub fn enums(&self) -> Iter<Enum<U>> {
        Iter::new(self.enums.clone())
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
    pub fn package_source_code_info(&self) -> Option<Rc<prost_types::SourceCodeInfo>> {
        self.pkg_info.borrow().clone()
    }
    pub fn source_code_info(&self) -> Option<Rc<prost_types::SourceCodeInfo>> {
        self.src_info.borrow().clone()
    }

    /// returns an iterator of top-level messages for this File. Nested messages
    /// are not included.
    pub fn messages(&self) -> Iter<Message<U>> {
        Iter::new(self.messages.clone())
    }
    /// returns an iterator of services for this File.
    pub fn services(&self) -> Iter<Service<U>> {
        Iter::new(self.services.clone())
    }
    /// defined_extensions returns an iterator of extensions defined in this
    /// file.
    pub fn defined_extensions(&self) -> Iter<Extension<U>> {
        Iter::new(self.def_exts.clone())
    }
    pub fn dependencies(&self) -> UpgradeIter<File<U>> {
        UpgradeIter::new(self.imports.clone())
    }
    /// dependents returns an iterator of all files where the given file was
    /// directly or transitively imported.
    pub fn dependents(&self) -> UpgradeIter<File<U>> {
        UpgradeIter::new(self.dependents.clone())
    }
    pub(crate) fn add_enum(&self, e: Rc<Enum<U>>) {
        self.enums.borrow_mut().push(e);
    }
    pub(crate) fn add_import(&self, file: Rc<File<U>>) {
        self.imports.borrow_mut().push(Rc::downgrade(&file));
    }
    pub(crate) fn add_dependent(&self, file: Rc<File<U>>) {
        self.dependents.borrow_mut().push(Rc::downgrade(&file));
    }
    pub(crate) fn add_extension(&self, ext: Rc<Extension<U>>) {
        self.def_exts.borrow_mut().push(ext);
    }
    pub(crate) fn add_message(&self, msg: Rc<Message<U>>) {
        self.messages.borrow_mut().push(msg);
    }
    pub(crate) fn add_service(&self, service: Rc<Service<U>>) {
        self.services.borrow_mut().push(service);
    }
    pub(crate) fn set_src_info(&self, src_info: prost_types::SourceCodeInfo) {
        self.src_info.replace(Some(Rc::new(src_info)));
    }
    pub(crate) fn set_pkg_info(&self, pkg_info: prost_types::SourceCodeInfo) {
        self.pkg_info.replace(Some(Rc::new(pkg_info)));
    }
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
            imports: Default::default(),
            def_exts: Default::default(),
            messages: Default::default(),
            enums: Default::default(),
            services: Default::default(),
            src_info: Default::default(),
            pkg_info: Default::default(),
        }
    }
}
