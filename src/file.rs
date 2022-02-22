use prost_types::FileDescriptorProto;

use crate::container::{BuildTarget, Container};
use crate::iter::{Iter, TransitiveImports, UpgradeIter};
use crate::path::FileDescriptorPath;
use crate::visit::{Accept, Visitor};
use crate::{
    AllEnums, AllMessages, Enum, EnumList, Extension, ExtensionList, Message, MessageList, Name,
    Node, NodeAtPath, Package, Service, ServiceList,
};
use std::cell::RefCell;

use std::path::PathBuf;
// use std::path::PathBuf;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub struct File<'a, U> {
    pub fully_qualified_name: String,
    pub descriptor: &'a FileDescriptorProto,
    pub name: Name<U>,
    pub file_path: PathBuf,
    pub build_target: bool,
    pub pkg_info: Option<prost_types::SourceCodeInfo>,
    pub src_info: Option<prost_types::SourceCodeInfo>,
    pub util: Rc<RefCell<U>>,
    pkg: Option<Weak<Package<'a, U>>>,
    messages: MessageList<'a, U>,
    enums: EnumList<'a, U>,
    services: ServiceList<'a, U>,
    dependents: Rc<RefCell<Vec<Weak<File<'a, U>>>>>,
    dependencies: Rc<RefCell<Vec<Weak<File<'a, U>>>>>,
    defined_extensions: Rc<RefCell<Vec<Rc<Extension<'a, U>>>>>,
}

impl<'a, U> BuildTarget for File<'a, U> {
    fn build_target(&self) -> bool {
        self.build_target
    }
}

impl<'a, U> File<'a, U> {
    pub(crate) fn new(
        build_target: bool,
        descriptor: &'a FileDescriptorProto,
        package: Option<Rc<Package<'a, U>>>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let pkg = package.map(|p| Rc::downgrade(&p));
        let name = Name::new(descriptor.name(), util.clone());
        let fully_qualified_name = match descriptor.package() {
            "" => String::default(),
            p => format!(".{}", p),
        };
        let file_path = PathBuf::from(descriptor.name());

        let file = Rc::new(Self {
            name,
            util: util.clone(),
            descriptor,
            pkg,
            fully_qualified_name,
            build_target,
            file_path,
            dependents: Rc::new(RefCell::new(Vec::new())),
            dependencies: Rc::new(RefCell::new(Vec::with_capacity(
                descriptor.dependency.len(),
            ))),
            defined_extensions: Rc::new(RefCell::new(Vec::with_capacity(
                descriptor.extension.len(),
            ))),
            messages: Rc::new(RefCell::new(Vec::with_capacity(
                descriptor.message_type.len(),
            ))),
            enums: Rc::new(RefCell::new(Vec::with_capacity(descriptor.enum_type.len()))),
            services: Rc::new(RefCell::new(Vec::with_capacity(descriptor.service.len()))),
            src_info: None,
            pkg_info: None,
        });

        let container = Container::File(file.clone());
        {
            let mut msgs = file.messages.borrow_mut();
            for md in file.descriptor.message_type.iter() {
                let msg = Message::new(md, container.clone(), util.clone());
                msgs.push(msg);
            }
        }
        {
            let mut enums = file.enums.borrow_mut();
            for ed in file.descriptor.enum_type.iter() {
                let e = Enum::new(ed, container.clone(), util.clone());
                enums.push(e);
            }
        }
        {
            let mut services = file.services.borrow_mut();
            for sd in file.descriptor.service.iter() {
                let svc = Service::new(sd, container.clone(), util.clone());
                services.push(svc);
            }
        }
        {
            let mut exts = file.defined_extensions.borrow_mut();
            for ed in file.descriptor.extension.iter() {
                let ext = Extension::new(ed, container.clone(), util.clone());
                exts.push(ext);
            }
        }

        file
    }
    pub fn messages(&self) -> Iter<Message<'a, U>> {
        Iter::from(&self.messages)
    }
    pub fn enums(&self) -> Iter<Enum<'a, U>> {
        Iter::from(&self.enums)
    }
    pub fn services(&self) -> Iter<Service<'a, U>> {
        Iter::from(&self.services)
    }
    pub fn defined_extensions(&self) -> Iter<Extension<'a, U>> {
        Iter::from(&self.defined_extensions)
    }

    pub fn imports(&self) -> UpgradeIter<File<'a, U>> {
        UpgradeIter::new(self.dependencies.clone())
    }
    pub fn dependents(&self) -> UpgradeIter<File<'a, U>> {
        UpgradeIter::new(self.dependents.clone())
    }
    pub fn dependencies(&self) -> UpgradeIter<File<'a, U>> {
        UpgradeIter::new(self.dependencies.clone())
    }

    pub fn transitive_imports(&self) -> TransitiveImports<'a, U> {
        TransitiveImports::new(self.dependencies.clone())
    }

    /// all_messages returns an iterator of all top-level and nested messages from this
    /// file.
    pub fn all_messages(&self) -> AllMessages<'a, U> {
        AllMessages::new(self.messages.clone())
    }

    /// all_enums returns an iterator of all top-level and nested enums from this file.
    pub fn all_enums(&self) -> AllEnums<'a, U> {
        AllEnums::new(self.enums.clone(), self.messages.clone())
    }

    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }
    pub fn package(&self) -> Option<Rc<Package<'a, U>>> {
        self.pkg.clone().map(|p| p.upgrade().unwrap())
    }

    pub(crate) fn add_dependency(&self, file: Rc<File<'a, U>>) {
        self.dependencies.borrow_mut().push(Rc::downgrade(&file));
    }

    pub(crate) fn add_dependent(&self, file: Rc<File<'a, U>>) {
        self.dependents.borrow_mut().push(Rc::downgrade(&file));
    }
}
trait Hydrate<'a, U> {
    fn hydrate(&self) -> Rc<File<'a, U>>;
    fn hydrate_messages(&self);
    fn hydrate_enums(&self);
    fn hydrate_services(&self);
    fn hydrate_extensions(&self);
}

impl<'a, U> NodeAtPath<'a, U> for Rc<File<'a, U>> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
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

impl<'a, U, V: Visitor<'a, U>> Accept<'a, U, V> for Rc<File<'a, U>> {
    fn accept(&self, v: &mut V) -> Result<(), V::Error> {
        if v.done() {
            return Ok(());
        }
        v.visit_file(self.clone())?;
        for msg in self.messages() {
            msg.accept(v)?;
        }
        for enm in self.enums() {
            enm.accept(v)?;
        }
        for svc in self.services() {
            svc.accept(v)?;
        }
        for ext in self.defined_extensions() {
            ext.accept(v)?;
        }
        Ok(())
    }
}
