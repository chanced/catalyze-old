use crate::container::Container;
use crate::iter::{FileRefIter, Iter, TransitiveImports};
use crate::package::WeakPackage;

use crate::proto::path::FileDescriptorPath;
use crate::proto::FileDescriptor;

use crate::{
    AllEnums, AllMessages, Comments, Enum, EnumList, Extension, ExtensionList, FullyQualified,
    Message, MessageList, Name, Node, NodeAtPath, Package, Service, ServiceList,
};
use std::cell::RefCell;

use std::path::PathBuf;
// use std::path::PathBuf;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
struct FileDetail<'a, U> {
    desc: FileDescriptor<'a>,
    name: Name<U>,
    file_path: PathBuf,
    build_target: bool,
    pkg_comments: RefCell<Comments<'a, U>>,
    comments: RefCell<Comments<'a, U>>,
    util: RefCell<Rc<U>>,
    fqn: String,
    pkg: WeakPackage<'a, U>,
    messages: MessageList<'a, U>,
    enums: EnumList<'a, U>,
    services: ServiceList<'a, U>,
    dependents: Rc<RefCell<Vec<WeakFile<'a, U>>>>,
    dependencies: Rc<RefCell<Vec<WeakFile<'a, U>>>>,
    defined_extensions: ExtensionList<'a, U>,
}

#[derive(Debug)]
pub struct File<'a, U>(Rc<FileDetail<'a, U>>);

impl<'a, U> File<'a, U> {
    pub(crate) fn new(build_target: bool, desc: FileDescriptor<'a>, pkg: Package<'a, U>) -> Self {
        let util = pkg.util();
        let name = Name::new(desc.name(), util.clone());
        let fqn = match desc.package() {
            "" => String::default(),
            p => format!(".{}", p),
        };

        let file = Self(Rc::new(FileDetail {
            name,
            desc,
            util: RefCell::new(util),
            pkg: pkg.into(),
            build_target,
            fqn,
            file_path: PathBuf::from(desc.name()),
            dependents: Rc::new(RefCell::new(Vec::new())),
            dependencies: Rc::new(RefCell::new(Vec::with_capacity(desc.dependencies().len()))),
            defined_extensions: Rc::new(RefCell::new(Vec::with_capacity(desc.extensions().len()))),
            messages: Rc::new(RefCell::new(Vec::with_capacity(desc.messages().len()))),
            enums: Rc::new(RefCell::new(Vec::with_capacity(desc.enums().len()))),
            services: Rc::new(RefCell::new(Vec::with_capacity(desc.services().len()))),
            pkg_comments: RefCell::new(Comments::default()),
            comments: RefCell::new(Comments::default()),
        }));

        let container: Container<'a, U> = file.clone().into();
        {
            let mut msgs = file.0.messages.borrow_mut();
            for md in desc.messages() {
                let msg = Message::new(md, container.clone());

                // TODO: handle map & oneof sitautions
                todo!();
                // msgs.push(msg);
            }
        }
        {
            let mut enums = file.0.enums.borrow_mut();
            for ed in desc.enums() {
                let e = Enum::new(ed, container.clone());
                enums.push(e);
            }
        }
        {
            let mut services = file.0.services.borrow_mut();
            for sd in desc.services() {
                let svc = Service::new(sd, file.clone());
                services.push(svc);
            }
        }
        {
            let mut exts = file.0.defined_extensions.borrow_mut();
            for ed in desc.extensions() {
                let ext = Extension::new(ed, container.clone());
                exts.push(ext);
            }
        }
        {
            for loc in desc.source_code_info() {
                match loc.file_descriptor_path() {
                    Ok(p) => match p {
                        FileDescriptorPath::Package => file.set_package_comments(loc.into()),
                        FileDescriptorPath::Syntax => file.set_comments(loc.into()),
                        _ => {
                            let n = file.node_at_path(loc.path());
                            if n.is_some() {
                                n.unwrap().set_comments(loc.into())
                            }
                        }
                    },
                    Err(_) => continue,
                }
            }
        }

        file
    }
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.pkg.clone().into()
    }
    pub fn build_target(&self) -> bool {
        self.0.build_target
    }
    pub fn comments(&self) -> Comments<'a, U> {
        *self.0.comments.borrow()
    }

    /// Returns comments attached to the package in this File if any exist.
    pub fn package_comments(&self) -> Comments<'a, U> {
        *self.0.pkg_comments.borrow()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a, U>) {
        *self.0.comments.borrow_mut() = comments;
    }
    pub(crate) fn set_package_comments(&self, comments: Comments<'a, U>) {
        *self.0.pkg_comments.borrow_mut() = comments;
    }
    pub fn util(&self) -> Rc<U> {
        self.0.util.borrow().clone()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.0.util.replace(util);
    }
    pub fn messages(&self) -> Iter<Message<'a, U>> {
        Iter::from(&self.0.messages)
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

    pub fn imports(&self) -> FileRefIter<'a, U> {
        self.0.dependencies.clone().into()
    }

    pub fn dependents(&self) -> FileRefIter<'a, U> {
        self.0.dependents.clone().into()
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

    pub fn path(&self) -> PathBuf {
        self.0.file_path.clone()
    }

    pub(crate) fn add_dependency(&self, file: File<'a, U>) {
        self.0.dependencies.borrow_mut().push(file.into());
    }

    pub(crate) fn add_dependent(&self, file: File<'a, U>) {
        self.0.dependents.borrow_mut().push(file.into());
    }
    fn downgrade(&self) -> WeakFile<'a, U> {
        WeakFile(Rc::downgrade(&self.0))
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
impl<'a, U> From<&WeakFile<'a, U>> for File<'a, U> {
    fn from(weak: &WeakFile<'a, U>) -> Self {
        weak.upgrade()
    }
}
impl<'a, U> From<WeakFile<'a, U>> for File<'a, U> {
    fn from(weak: WeakFile<'a, U>) -> Self {
        weak.upgrade()
    }
}
impl<'a, U> Clone for File<'a, U> {
    fn clone(&self) -> Self {
        File(self.0.clone())
    }
}
impl<'a, U> FullyQualified for File<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}

#[cfg(test)]
impl<'a> Default for File<'a, crate::util::Generic> {
    fn default() -> Self {
        File(Rc::new(FileDetail {
            build_target: false,
            fqn: "".to_string(),
            file_path: PathBuf::new(),
            name: Name::default(),
            pkg: Package::default().into(),
            dependencies: Default::default(),
            dependents: Default::default(),
            messages: Default::default(),
            enums: Default::default(),
            services: Default::default(),
            defined_extensions: Default::default(),
            comments: Default::default(),
            pkg_comments: RefCell::new(Comments::default()),
            util: RefCell::new(Rc::new(crate::util::Generic::default())),
            desc: FileDescriptor::default(),
        }))
    }
}

#[derive(Debug)]
pub(crate) struct WeakFile<'a, U>(Weak<FileDetail<'a, U>>);

impl<'a, U> WeakFile<'a, U> {
    pub fn fully_qualified_name(&self) -> String {
        self.upgrade().fully_qualified_name()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.upgrade().package()
    }
    pub fn name(&self) -> Name<U> {
        self.upgrade().name()
    }
    pub fn path(&self) -> PathBuf {
        self.upgrade().path()
    }
    pub fn build_target(&self) -> bool {
        self.upgrade().build_target()
    }
    fn upgrade(&self) -> File<'a, U> {
        File(self.0.upgrade().expect("Failed to upgrade weak file"))
    }
}

impl<'a, U> Clone for WeakFile<'a, U> {
    fn clone(&self) -> Self {
        WeakFile(self.0.clone())
    }
}
impl<'a, U> From<File<'a, U>> for WeakFile<'a, U> {
    fn from(file: File<'a, U>) -> Self {
        file.downgrade()
    }
}
impl<'a, U> From<&File<'a, U>> for WeakFile<'a, U> {
    fn from(file: &File<'a, U>) -> Self {
        file.downgrade()
    }
}
