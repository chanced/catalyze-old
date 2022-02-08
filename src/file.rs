use crate::entity::BuildTarget;
use crate::lang::Lang;
use crate::name::Named;
use crate::{Enum, Extension, Message, Name, Package, Service};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
// use std::path::PathBuf;
use std::rc::{Rc, Weak};
#[derive(Debug, Clone)]
pub struct File<L: Lang> {
    fqn: String,
    desc: Rc<prost_types::FileDescriptorProto>,
    name: Name<L>,
    pkg: Weak<Package<L>>,
    dependents: RefCell<Vec<Weak<File<L>>>>,
    dependencies: RefCell<Vec<Weak<File<L>>>>,
    exts: RefCell<Vec<Rc<Extension<L>>>>,
    msgs: RefCell<Vec<Rc<Message<L>>>>,
    enums: RefCell<Vec<Rc<Enum<L>>>>,
    services: RefCell<Vec<Rc<Service<L>>>>,
    src_info: RefCell<Option<Rc<prost_types::SourceCodeInfo>>>,
    pkg_info: RefCell<Option<Rc<prost_types::SourceCodeInfo>>>,
    build_target: bool,
    file_path: PathBuf,
}

impl<L: Lang> BuildTarget for File<L> {
    fn build_target(&self) -> bool {
        self.build_target
    }
}

impl<L: Lang> File<L> {
    pub fn name(&self) -> Name<L> {
        self.name.clone()
    }
    pub(crate) fn new(
        build_target: bool,
        desc: prost_types::FileDescriptorProto,
        package: Rc<Package<L>>,
        lang: L,
    ) -> Self {
        let desc = Rc::new(desc);
        let pkg = Rc::downgrade(&package);
        let name = Name::new(desc.name(), lang);
        let fqn = match desc.package() {
            "" => String::from(""),
            p => format!(".{}", p),
        };
        let file_path = PathBuf::from(desc.name());
        Self {
            desc,
            name,
            pkg,
            fqn,
            build_target,
            file_path,
            dependents: RefCell::new(Vec::new()),
            dependencies: RefCell::new(Vec::new()),
            exts: RefCell::new(Vec::new()),
            msgs: RefCell::new(Vec::new()),
            enums: RefCell::new(Vec::new()),
            services: RefCell::new(Vec::new()),
            src_info: RefCell::new(None),
            pkg_info: RefCell::new(None),
        }
    }

    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }
    pub fn package(&self) -> Rc<Package<L>> {
        self.pkg.upgrade().unwrap()
    }
    pub fn package_source_code_info(&self) -> Option<Rc<prost_types::SourceCodeInfo>> {
        self.pkg_info.borrow().clone()
    }
    pub fn source_code_info(&self) -> Option<Rc<prost_types::SourceCodeInfo>> {
        self.src_info.borrow().clone()
    }
    pub fn descriptor(&self) -> Rc<prost_types::FileDescriptorProto> {
        self.desc.clone()
    }

    pub fn all_messages(&self) -> Vec<Rc<Message<L>>> {
        todo!()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
    pub fn messages(&self) -> Vec<Rc<Message<L>>> {
        self.msgs.borrow().iter().map(Rc::clone).collect()
    }
    pub fn enums(&self) -> Vec<Rc<Enum<L>>> {
        self.enums.borrow().iter().map(Rc::clone).collect()
    }
    pub fn services(&self) -> Vec<Rc<Service<L>>> {
        self.services.borrow().iter().map(Rc::clone).collect()
    }
    pub fn extensions(&self) -> Vec<Rc<Extension<L>>> {
        self.exts.borrow().iter().map(Rc::clone).collect()
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
        self.msgs.borrow_mut().push(msg);
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
