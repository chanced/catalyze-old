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
struct FileDetail<'a, U> {
    desc: FileDescriptor<'a>,
    name: Name<U>,
    file_path: PathBuf,
    fqn: String,
    messages: Rc<RefCell<Vec<Message<'a, U>>>>,
    enums: Rc<RefCell<Vec<Enum<'a, U>>>>,
    services: ServiceList<'a, U>,
    defined_extensions: ExtensionList<'a, U>,
    build_target: bool,
    pkg_comments: RefCell<Comments<'a>>,
    comments: RefCell<Comments<'a>>,
    util: Rc<U>,
    pkg: WeakPackage<'a, U>,
    dependents: Rc<RefCell<Vec<WeakFile<'a, U>>>>,
    dependencies: Rc<RefCell<Vec<WeakFile<'a, U>>>>,
    syntax: Syntax,
}

#[derive(Debug)]
pub struct File<'a, U>(Rc<FileDetail<'a, U>>);

impl<'a, U> File<'a, U> {
    pub(crate) fn new(
        build_target: bool,
        desc: FileDescriptor<'a>,
        pkg: Package<'a, U>,
    ) -> Result<Self, anyhow::Error> {
        let util = pkg.util();
        let name = Name::new(desc.name(), util.clone());
        let fqn = match desc.package() {
            "" => String::default(),
            p => format!(".{}", p),
        };

        let file = Self(Rc::new(FileDetail {
            name,
            desc,
            util,
            pkg: pkg.into(),
            build_target,
            fqn,
            syntax: desc.syntax(),
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
                let msg = Message::new(md, container.clone())?;
                msgs.push(msg);
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
                            if let Some(n) = n {
                                n.set_comments(loc.into())
                            }
                        }
                    },
                    Err(_) => continue,
                }
            }
        }

        Ok(file)
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
    pub fn comments(&self) -> Comments<'a> {
        *self.0.comments.borrow()
    }
    pub fn file_path(&self) -> &PathBuf {
        &self.0.file_path
    }
    /// Returns comments attached to the package in this File if any exist.
    pub fn package_comments(&self) -> Comments<'a> {
        *self.0.pkg_comments.borrow()
    }
    pub fn descriptor(&self) -> FileDescriptor<'a> {
        self.0.desc
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        *self.0.comments.borrow_mut() = comments;
    }
    pub(crate) fn set_package_comments(&self, comments: Comments<'a>) {
        *self.0.pkg_comments.borrow_mut() = comments;
    }
    pub fn util(&self) -> Rc<U> {
        self.0.util.clone()
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

    pub fn imports(&self) -> Files<'a, U> {
        self.0.dependencies.clone().into()
    }

    pub fn dependents(&self) -> Files<'a, U> {
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
    pub fn all_nodes(&self) -> AllNodes<'a, U> {
        AllNodes::new(self.into())
    }
    pub fn nodes(&self) -> Nodes<'a, U> {
        Nodes::new(vec![
            self.enums().into(),
            self.messages().into(),
            self.services().into(),
            self.defined_extensions().into(),
        ])
    }
    #[cfg(test)]
    pub fn add_node(&self, n: Node<'a, U>) {
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

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
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

impl<'a, U> PartialEq for File<'a, U> {
    fn eq(&self, other: &Self) -> bool {
        self.file_path() == other.file_path()
    }
}

impl<'a, U> PartialEq<WeakFile<'a, U>> for File<'a, U> {
    fn eq(&self, other: &WeakFile<'a, U>) -> bool {
        self.file_path() == other.upgrade().file_path()
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
impl<'a, U> PartialEq<File<'a, U>> for WeakFile<'a, U> {
    fn eq(&self, other: &File<'a, U>) -> bool {
        self.upgrade().file_path() == other.file_path()
    }
}
impl<'a, U> PartialEq for WeakFile<'a, U> {
    fn eq(&self, other: &Self) -> bool {
        self.upgrade().file_path() == other.upgrade().file_path()
    }
}

#[derive(Debug)]
pub struct TransitiveImports<'a, U> {
    queue: VecDeque<File<'a, U>>,
    processed: HashSet<Name<U>>,
}
impl<'a, U> TransitiveImports<'a, U> {
    pub(crate) fn new(files: Rc<RefCell<Vec<WeakFile<'a, U>>>>) -> Self {
        Self {
            queue: VecDeque::from_iter(files.borrow().iter().map(|f| f.into())),
            processed: HashSet::new(),
        }
    }
}
impl<'a, U> Iterator for TransitiveImports<'a, U> {
    type Item = File<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(file) = self.queue.pop_front() {
            if !self.processed.contains(&file.name()) {
                self.processed.insert(file.name());
                for d in file.imports() {
                    self.queue.push_back(d);
                }
                return Some(file);
            }
        }
        None
    }
}

/// FileRefIter is an iterator that upgrades weak references to `File`s.
pub struct Files<'a, U> {
    files: Rc<RefCell<Vec<WeakFile<'a, U>>>>,
    index: usize,
}
impl<'a, U> Files<'a, U> {
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

impl<'a, U> From<&Rc<RefCell<Vec<WeakFile<'a, U>>>>> for Files<'a, U> {
    fn from(files: &Rc<RefCell<Vec<WeakFile<'a, U>>>>) -> Self {
        Files {
            files: files.clone(),
            index: 0,
        }
    }
}
impl<'a, U> From<Rc<RefCell<Vec<WeakFile<'a, U>>>>> for Files<'a, U> {
    fn from(files: Rc<RefCell<Vec<WeakFile<'a, U>>>>) -> Self {
        Files {
            files: files.clone(),
            index: 0,
        }
    }
}

impl<'a, U> From<WeakFile<'a, U>> for Files<'a, U> {
    fn from(file: WeakFile<'a, U>) -> Self {
        Files {
            files: Rc::new(RefCell::new(vec![file])),
            index: 0,
        }
    }
}

impl<'a, U> From<Option<WeakFile<'a, U>>> for Files<'a, U> {
    fn from(file: Option<WeakFile<'a, U>>) -> Self {
        file.map_or(Self::empty(), Into::into)
    }
}
impl<'a, U> Iterator for Files<'a, U> {
    type Item = File<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        let files = self.files.borrow();
        if let Some(file) = files.get(self.index) {
            self.index += 1;
            return Some(file.into());
        }
        None
    }
}
