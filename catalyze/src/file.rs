use crate::container::Container;
use crate::iter::Iter;
use crate::package::WeakPackage;

use crate::proto::FileDescriptorPath;
use crate::proto::{FileDescriptor, Syntax};

use crate::*;
use std::cell::RefCell;

use std::collections::{HashSet, VecDeque};
use std::path::PathBuf;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub struct File(Rc<FileDetail>);

#[derive(Debug, Clone)]
struct FileDetail {
    descriptor: FileDescriptor,
    file_path: PathBuf,
    fqn: String,
    messages: RefCell<Vec<Message>>,
    enums: RefCell<Vec<Enum>>,
    services: RefCell<Vec<Service>>,
    defined_extensions: RefCell<Vec<Extension>>,
    build_target: bool,
    pkg_comments: RefCell<Comments>,
    comments: RefCell<Comments>,

    pkg: WeakPackage,
    dependents: RefCell<Vec<WeakFile>>,
    imports: RefCell<Vec<WeakFile>>,
    used_imports: Rc<RefCell<HashSet<String>>>,
    syntax: Syntax,
}

impl FileDetail {
    pub fn new(build_target: bool, descriptor: FileDescriptor, pkg: Package) -> Rc<Self> {
        let fqn = match descriptor.package() {
            "" => String::default(),
            p => format!(".{}", p),
        };
        Rc::new(Self {
            descriptor,
            pkg: pkg.into(),
            build_target,
            fqn,
            syntax: descriptor.syntax(),
            file_path: PathBuf::from(descriptor.name()),
            dependents: Rc::new(RefCell::new(Vec::new())),
            imports: Rc::new(RefCell::new(Vec::with_capacity(
                descriptor.dependencies().len(),
            ))),
            defined_extensions: Rc::new(RefCell::new(Vec::with_capacity(
                descriptor.extensions().len(),
            ))),
            messages: Rc::new(RefCell::new(Vec::with_capacity(
                descriptor.messages().len(),
            ))),
            enums: Rc::new(RefCell::new(Vec::with_capacity(descriptor.enums().len()))),
            services: Rc::new(RefCell::new(Vec::with_capacity(
                descriptor.services().len(),
            ))),
            pkg_comments: RefCell::new(Comments::default()),
            comments: RefCell::new(Comments::default()),
            used_imports: Rc::new(RefCell::new(HashSet::new())),
        })
    }
}

impl File {
    pub(crate) fn new(
        build_target: bool,
        desc: FileDescriptor,
        pkg: Package,
    ) -> Result<Self, Error> {
        let file = Self(FileDetail::new(build_target, desc, pkg))
            .hydrate_messages()?
            .hydrate_enums()
            .hydrate_services()
            .hydrate_extensions()
            .assign_comments();
        Ok(file)
    }

    fn assign_comments(self) -> Self {
        {
            for loc in self.descriptor().source_code_info() {
                match loc.file_descriptor_path() {
                    Ok(p) => match p {
                        FileDescriptorPath::Package => self.set_package_comments(loc.into()),
                        FileDescriptorPath::Syntax => self.set_comments(loc.into()),
                        _ => {
                            let n = self.node_at_path(loc.path());
                            if let Some(n) = n {
                                n.set_comments(loc.into())
                            }
                        }
                    },
                    Err(_) => continue,
                }
            }
        }
        self
    }

    fn hydrate_extensions(self) -> Self {
        {
            let mut exts = self.0.defined_extensions.borrow_mut();
            let container = self.as_container();
            for ed in self.descriptor().extensions() {
                let ext = Extension::new(ed, container.clone());
                exts.push(ext);
            }
        }
        self
    }
    fn hydrate_messages(self) -> Result<Self, Error> {
        {
            let container = self.clone().as_container();
            let mut msgs = self.0.messages.borrow_mut();
            for md in self.descriptor().messages() {
                let msg = Message::new(md, container.clone())?;
                msgs.push(msg);
            }
        }
        Ok(self)
    }
    fn hydrate_enums(self) -> Self {
        {
            let container = self.as_container();

            let mut enums = self.0.enums.borrow_mut();
            for ed in self.descriptor().enums() {
                let e = Enum::new(ed, container.clone());
                enums.push(e);
            }
        }
        self
    }
    fn hydrate_services(self) -> Self {
        {
            let mut services = self.0.services.borrow_mut();
            for sd in self.descriptor().services() {
                let svc = Service::new(sd, self.clone());
                services.push(svc);
            }
        }
        self
    }
    pub fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
    }
    pub fn as_container(&self) -> Container {
        self.clone().into()
    }
    pub fn name(&self) -> &str {
        &self.0.descriptor.name()
    }
    pub fn package(&self) -> Package {
        self.0.pkg.clone().into()
    }
    pub fn build_target(&self) -> bool {
        self.0.build_target
    }
    pub fn comments(&self) -> Comments {
        *self.0.comments.borrow()
    }
    pub fn file_path(&self) -> &PathBuf {
        &self.0.file_path
    }
    /// Returns comments attached to the package in this File if any exist.
    pub fn package_comments(&self) -> Comments {
        *self.0.pkg_comments.borrow()
    }
    pub fn descriptor(&self) -> FileDescriptor {
        self.0.descriptor
    }
    pub(crate) fn set_comments(&self, comments: Comments) {
        *self.0.comments.borrow_mut() = comments;
    }
    pub(crate) fn set_package_comments(&self, comments: Comments) {
        *self.0.pkg_comments.borrow_mut() = comments;
    }
    pub fn message(&self, name: &str) -> Option<Message> {
        let name = name.to_lowercase();
        self.all_messages().find(|m| {
            m.name().to_lowercase() == name || m.fully_qualified_name().to_lowercase() == name
        })
    }
    pub fn messages(&self) -> Iter<Message> {
        Iter::from(&self.0.messages)
    }
    pub fn enum_(&self, name: &str) -> Option<Enum> {
        self.all_enums().find(|e| e.name() == name)
    }
    pub fn enum_s(&self) -> Iter<Enum> {
        self.enums()
    }
    pub fn enums(&self) -> Iter<Enum> {
        Iter::from(&self.0.enums)
    }
    pub fn services(&self) -> Iter<Service> {
        Iter::from(&self.0.services)
    }
    pub fn defined_extensions(&self) -> Iter<Extension> {
        Iter::from(&self.0.defined_extensions)
    }

    pub fn imports(&self) -> FileRefs {
        self.0.imports.clone().into()
    }

    pub fn dependents(&self) -> FileRefs {
        self.0.dependents.clone().into()
    }
    pub fn transitive_imports(&self) -> TransitiveImports {
        TransitiveImports::new(self.0.imports.clone())
    }

    /// all_messages returns an iterator of all top-level and nested messages from this
    /// file.
    pub fn all_messages(&self) -> AllMessages {
        AllMessages::new(self.0.messages.clone())
    }

    /// all_enums returns an iterator of all top-level and nested enums from this file.
    pub fn all_enums(&self) -> AllEnums {
        AllEnums::new(self.0.enums.clone(), self.0.messages.clone())
    }

    pub fn path(&self) -> PathBuf {
        self.0.file_path.clone()
    }

    pub(crate) fn add_import(&self, file: File) {
        self.0.imports.borrow_mut().push(file.into());
    }

    pub(crate) fn mark_import_as_used(&self, file: File) {
        self.0
            .used_imports
            .borrow_mut()
            .insert(file.fully_qualified_name().to_string());
    }

    pub(crate) fn add_dependent(&self, file: File) {
        self.0.dependents.borrow_mut().push(file.into());
    }
    fn downgrade(&self) -> WeakFile {
        WeakFile(Rc::downgrade(&self.0))
    }
    pub fn all_nodes(&self) -> AllNodes {
        AllNodes::new(self.into())
    }
    pub fn nodes(&self) -> Nodes {
        Nodes::new(vec![
            self.enums().into(),
            self.messages().into(),
            self.services().into(),
            self.defined_extensions().into(),
        ])
    }

    pub fn unused_imports(&self) -> Vec<File> {
        let used_imports = self.0.used_imports.borrow();
        let mut unused_imports = Vec::new();
        for file in self.0.imports.borrow().iter() {
            if !used_imports.contains(&file.fully_qualified_name()) {
                unused_imports.push(file.into());
            }
        }
        unused_imports
    }

    #[cfg(test)]
    pub fn add_node(&self, n: Node) {
        match n {
            Node::Message(m) => self.0.messages.borrow_mut().push(m),
            Node::Enum(e) => self.0.enums.borrow_mut().push(e),
            Node::Service(s) => self.0.services.borrow_mut().push(s),
            Node::Extension(e) => self.0.defined_extensions.borrow_mut().push(e),
            _ => panic!("unexpected node type"),
        }
    }

    pub fn syntax(&self) -> crate::proto::Syntax {
        self.0.syntax
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node> {
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

    pub fn service(&self, name: &str) -> Option<Service> {
        self.services().find(|s| s.name() == name)
    }
}

impl From<&WeakFile> for File {
    fn from(weak: &WeakFile) -> Self {
        weak.upgrade()
    }
}
impl From<WeakFile> for File {
    fn from(weak: WeakFile) -> Self {
        weak.upgrade()
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.file_path() == other.file_path()
    }
}

impl PartialEq<WeakFile> for File {
    fn eq(&self, other: &WeakFile) -> bool {
        self.file_path() == other.upgrade().file_path()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WeakFile(Weak<FileDetail>);

impl WeakFile {
    pub fn fully_qualified_name(&self) -> &str {
        self.upgrade().fully_qualified_name()
    }
    pub fn package(&self) -> Package {
        self.upgrade().package()
    }

    pub fn build_target(&self) -> bool {
        self.upgrade().build_target()
    }
    fn upgrade(&self) -> File {
        File(self.0.upgrade().expect("Failed to upgrade weak file"))
    }
}

impl From<File> for WeakFile {
    fn from(file: File) -> Self {
        file.downgrade()
    }
}
impl From<&File> for WeakFile {
    fn from(file: &File) -> Self {
        file.downgrade()
    }
}
impl PartialEq<File> for WeakFile {
    fn eq(&self, other: &File) -> bool {
        self.upgrade().file_path() == other.file_path()
    }
}
impl PartialEq for WeakFile {
    fn eq(&self, other: &Self) -> bool {
        self.upgrade().file_path() == other.upgrade().file_path()
    }
}

#[derive(Debug, Clone)]
pub struct TransitiveImports {
    queue: VecDeque<File>,
    processed: HashSet<String>,
}
impl TransitiveImports {
    pub(crate) fn new(files: RefCell<Vec<WeakFile>>) -> Self {
        Self {
            queue: VecDeque::from_iter(files.borrow().iter().map(|f| f.into())),
            processed: HashSet::new(),
        }
    }
}
impl Iterator for TransitiveImports {
    type Item = File;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(file) = self.queue.pop_front() {
            if !self.processed.contains(file.name()) {
                self.processed.insert(file.name().to_string());
                for d in file.imports() {
                    self.queue.push_back(d);
                }
                return Some(file);
            }
        }
        None
    }
}

/// An iterator that upgrades weak file references to `File`s.
pub struct FileRefs {
    files: Rc<RefCell<Vec<WeakFile>>>,
    index: usize,
}
impl FileRefs {
    pub fn len(&self) -> usize {
        self.files.borrow().len()
    }
    pub fn is_empty(&self) -> bool {
        self.files.borrow().is_empty()
    }
    pub fn empty() -> Self {
        Self {
            files: Rc::new(RefCell::new(Vec::new())),
            index: 0,
        }
    }
}

impl From<&Rc<RefCell<Vec<WeakFile>>>> for FileRefs {
    fn from(files: &Rc<RefCell<Vec<WeakFile>>>) -> Self {
        FileRefs {
            files: files.clone(),
            index: 0,
        }
    }
}
impl From<Rc<RefCell<Vec<WeakFile>>>> for FileRefs {
    fn from(files: Rc<RefCell<Vec<WeakFile>>>) -> Self {
        FileRefs {
            files: files.clone(),
            index: 0,
        }
    }
}

impl From<WeakFile> for FileRefs {
    fn from(file: WeakFile) -> Self {
        FileRefs {
            files: Rc::new(RefCell::new(vec![file])),
            index: 0,
        }
    }
}

impl From<Option<WeakFile>> for FileRefs {
    fn from(file: Option<WeakFile>) -> Self {
        file.map_or(Self::empty(), Into::into)
    }
}
impl Iterator for FileRefs {
    type Item = File;
    fn next(&mut self) -> Option<Self::Item> {
        let files = self.files.borrow();
        if let Some(file) = files.get(self.index) {
            self.index += 1;
            return Some(file.into());
        }
        None
    }
}
