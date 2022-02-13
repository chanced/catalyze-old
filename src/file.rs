use crate::container::BuildTarget;
use crate::lang::Lang;
use crate::{Enum, Extension, Message, Name, Package, Service};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::Map;
use std::path::PathBuf;
// use std::path::PathBuf;
use std::rc::{Rc, Weak};
use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct File<L: Lang> {
    pub fully_qualified_name: String,
    pub descriptor: prost_types::FileDescriptorProto,
    pub name: Name<L>,
    pub file_path: PathBuf,
    pub build_target: bool,
    pkg: Weak<Package<L>>,
    dependents: RefCell<Vec<Weak<File<L>>>>,
    dependencies: RefCell<Vec<Weak<File<L>>>>,
    exts: RefCell<Vec<Rc<Extension<L>>>>,
    msgs: RefCell<Vec<Rc<Message<L>>>>,
    enums: RefCell<Vec<Rc<Enum<L>>>>,
    services: RefCell<Vec<Rc<Service<L>>>>,
    src_info: RefCell<Option<Rc<prost_types::SourceCodeInfo>>>,
    pkg_info: RefCell<Option<Rc<prost_types::SourceCodeInfo>>>,
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
        package: Rc<Package<L>>,
        lang: L,
    ) -> Self {
        let pkg = Rc::downgrade(&package);
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
    // AllMessages returns all the top-level and nested messages from this
    // Entity.
    pub fn all_messages(&self) -> Vec<Rc<Message<L>>> {
        let mut res: Vec<Rc<Message<L>>> = Vec::default();
        let mut stack: VecDeque<Rc<Message<L>>> = VecDeque::default();
        stack.extend(self.messages());
        res.extend(self.messages());
        while let Some(next) = stack.pop_front() {
            stack.extend(next.messages());
            res.extend(next.messages());
        }
        res
    }
    /// Returns top-level messages for this Message. Nested messages are not
    /// included.
    pub fn messages(&self) -> Vec<Rc<Message<L>>> {
        self.msgs.borrow().iter().cloned().collect()
    }
    pub fn enums(&self) -> Vec<Rc<Enum<L>>> {
        self.enums.borrow().iter().cloned().collect()
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
