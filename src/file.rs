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
struct FileDetail<'a> {
    desc: FileDescriptor<'a>,
    name: Name,
    file_path: PathBuf,
    fqn: String,
    messages: Rc<RefCell<Vec<Message<'a>>>>,
    enums: Rc<RefCell<Vec<Enum<'a>>>>,
    services: Rc<RefCell<Vec<Service<'a>>>>,
    defined_extensions: Rc<RefCell<Vec<Extension<'a>>>>,
    build_target: bool,
    pkg_comments: RefCell<Comments<'a>>,
    comments: RefCell<Comments<'a>>,

    pkg: WeakPackage<'a>,
    dependents: Rc<RefCell<Vec<WeakFile<'a>>>>,
    imports: Rc<RefCell<Vec<WeakFile<'a>>>>,
    used_imports: Rc<RefCell<HashSet<String>>>,
    syntax: Syntax,
}

impl<'a> FileDetail<'a> {
    pub fn new(build_target: bool, desc: FileDescriptor<'a>, pkg: Package<'a>) -> Rc<Self> {
        let name = desc.name().into();
        let fqn = match desc.package() {
            "" => String::default(),
            p => format!(".{}", p),
        };
        Rc::new(Self {
            name,
            desc,
            pkg: pkg.into(),
            build_target,
            fqn,
            syntax: desc.syntax(),
            file_path: PathBuf::from(desc.name()),
            dependents: Rc::new(RefCell::new(Vec::new())),
            imports: Rc::new(RefCell::new(Vec::with_capacity(desc.dependencies().len()))),
            defined_extensions: Rc::new(RefCell::new(Vec::with_capacity(desc.extensions().len()))),
            messages: Rc::new(RefCell::new(Vec::with_capacity(desc.messages().len()))),
            enums: Rc::new(RefCell::new(Vec::with_capacity(desc.enums().len()))),
            services: Rc::new(RefCell::new(Vec::with_capacity(desc.services().len()))),
            pkg_comments: RefCell::new(Comments::default()),
            comments: RefCell::new(Comments::default()),
            used_imports: Rc::new(RefCell::new(HashSet::new())),
        })
    }
}

#[derive(Debug, Clone)]
pub struct File<'a>(Rc<FileDetail<'a>>);

impl<'a> File<'a> {
    pub(crate) fn new(
        build_target: bool,
        desc: FileDescriptor<'a>,
        pkg: Package<'a>,
    ) -> Result<Self, anyhow::Error> {
        let file = Self(FileDetail::new(build_target, desc, pkg))
            .hydrate_msgs()?
            .hydrate_enums()
            .hydrate_services()
            .hydrate_exts()
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

    fn hydrate_exts(self) -> Self {
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
    fn hydrate_msgs(self) -> anyhow::Result<Self> {
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
    pub fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
    pub fn as_container(&self) -> Container<'a> {
        self.clone().into()
    }
    pub fn name(&self) -> &Name {
        &self.0.name
    }
    pub fn package(&self) -> Package<'a> {
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
    pub fn message(&self, name: &str) -> Option<Message<'a>> {
        let name = name.to_lowercase();
        self.all_messages().find(|m| {
            m.name().to_lowercase() == name || m.fully_qualified_name().to_lowercase() == name
        })
    }
    pub fn messages(&self) -> Iter<Message<'a>> {
        Iter::from(&self.0.messages)
    }
    pub fn enums(&self) -> Iter<Enum<'a>> {
        Iter::from(&self.0.enums)
    }
    pub fn services(&self) -> Iter<Service<'a>> {
        Iter::from(&self.0.services)
    }
    pub fn defined_extensions(&self) -> Iter<Extension<'a>> {
        Iter::from(&self.0.defined_extensions)
    }

    pub fn imports(&self) -> FileRefs<'a> {
        self.0.imports.clone().into()
    }

    pub fn dependents(&self) -> FileRefs<'a> {
        self.0.dependents.clone().into()
    }
    pub fn transitive_imports(&self) -> TransitiveImports<'a> {
        TransitiveImports::new(self.0.imports.clone())
    }

    /// all_messages returns an iterator of all top-level and nested messages from this
    /// file.
    pub fn all_messages(&self) -> AllMessages<'a> {
        AllMessages::new(self.0.messages.clone())
    }

    /// all_enums returns an iterator of all top-level and nested enums from this file.
    pub fn all_enums(&self) -> AllEnums<'a> {
        AllEnums::new(self.0.enums.clone(), self.0.messages.clone())
    }

    pub fn path(&self) -> PathBuf {
        self.0.file_path.clone()
    }

    pub(crate) fn add_import(&self, file: File<'a>) {
        self.0.imports.borrow_mut().push(file.into());
    }

    pub(crate) fn mark_import_as_used(&self, file: File<'a>) {
        self.0
            .used_imports
            .borrow_mut()
            .insert(file.fully_qualified_name());
    }

    pub(crate) fn add_dependent(&self, file: File<'a>) {
        self.0.dependents.borrow_mut().push(file.into());
    }
    fn downgrade(&self) -> WeakFile<'a> {
        WeakFile(Rc::downgrade(&self.0))
    }
    pub fn all_nodes(&self) -> AllNodes<'a> {
        AllNodes::new(self.into())
    }
    pub fn nodes(&self) -> Nodes<'a> {
        Nodes::new(vec![
            self.enums().into(),
            self.messages().into(),
            self.services().into(),
            self.defined_extensions().into(),
        ])
    }

    pub fn unused_imports(&self) -> Vec<File<'a>> {
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
    pub fn add_node(&self, n: Node<'a>) {
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

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a>> {
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

impl<'a> From<&WeakFile<'a>> for File<'a> {
    fn from(weak: &WeakFile<'a>) -> Self {
        weak.upgrade()
    }
}
impl<'a> From<WeakFile<'a>> for File<'a> {
    fn from(weak: WeakFile<'a>) -> Self {
        weak.upgrade()
    }
}

impl<'a> PartialEq for File<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.file_path() == other.file_path()
    }
}

impl<'a> PartialEq<WeakFile<'a>> for File<'a> {
    fn eq(&self, other: &WeakFile<'a>) -> bool {
        self.file_path() == other.upgrade().file_path()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WeakFile<'a>(Weak<FileDetail<'a>>);

impl<'a> WeakFile<'a> {
    pub fn fully_qualified_name(&self) -> String {
        self.upgrade().fully_qualified_name()
    }
    pub fn package(&self) -> Package<'a> {
        self.upgrade().package()
    }

    pub fn build_target(&self) -> bool {
        self.upgrade().build_target()
    }
    fn upgrade(&self) -> File<'a> {
        File(self.0.upgrade().expect("Failed to upgrade weak file"))
    }
}

impl<'a> From<File<'a>> for WeakFile<'a> {
    fn from(file: File<'a>) -> Self {
        file.downgrade()
    }
}
impl<'a> From<&File<'a>> for WeakFile<'a> {
    fn from(file: &File<'a>) -> Self {
        file.downgrade()
    }
}
impl<'a> PartialEq<File<'a>> for WeakFile<'a> {
    fn eq(&self, other: &File<'a>) -> bool {
        self.upgrade().file_path() == other.file_path()
    }
}
impl<'a> PartialEq for WeakFile<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.upgrade().file_path() == other.upgrade().file_path()
    }
}

#[derive(Debug, Clone)]
pub struct TransitiveImports<'a> {
    queue: VecDeque<File<'a>>,
    processed: HashSet<Name>,
}
impl<'a> TransitiveImports<'a> {
    pub(crate) fn new(files: Rc<RefCell<Vec<WeakFile<'a>>>>) -> Self {
        Self {
            queue: VecDeque::from_iter(files.borrow().iter().map(|f| f.into())),
            processed: HashSet::new(),
        }
    }
}
impl<'a> Iterator for TransitiveImports<'a> {
    type Item = File<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(file) = self.queue.pop_front() {
            if !self.processed.contains(file.name()) {
                self.processed.insert(file.name().clone());
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
pub struct FileRefs<'a> {
    files: Rc<RefCell<Vec<WeakFile<'a>>>>,
    index: usize,
}
impl<'a> FileRefs<'a> {
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

impl<'a> From<&Rc<RefCell<Vec<WeakFile<'a>>>>> for FileRefs<'a> {
    fn from(files: &Rc<RefCell<Vec<WeakFile<'a>>>>) -> Self {
        FileRefs {
            files: files.clone(),
            index: 0,
        }
    }
}
impl<'a> From<Rc<RefCell<Vec<WeakFile<'a>>>>> for FileRefs<'a> {
    fn from(files: Rc<RefCell<Vec<WeakFile<'a>>>>) -> Self {
        FileRefs {
            files: files.clone(),
            index: 0,
        }
    }
}

impl<'a> From<WeakFile<'a>> for FileRefs<'a> {
    fn from(file: WeakFile<'a>) -> Self {
        FileRefs {
            files: Rc::new(RefCell::new(vec![file])),
            index: 0,
        }
    }
}

impl<'a> From<Option<WeakFile<'a>>> for FileRefs<'a> {
    fn from(file: Option<WeakFile<'a>>) -> Self {
        file.map_or(Self::empty(), Into::into)
    }
}
impl<'a> Iterator for FileRefs<'a> {
    type Item = File<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let files = self.files.borrow();
        if let Some(file) = files.get(self.index) {
            self.index += 1;
            return Some(file.into());
        }
        None
    }
}
