use crate::iter::Iter;
pub use crate::File;
use crate::{AllNodes, IntoNode, Node, Nodes, WELL_KNNOWN_TYPE_PACKAGE};

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::*;

#[derive(Debug, Clone)]
struct PackageDetail {
    fqn: String,
    files: Rc<RefCell<Vec<File>>>,
    is_well_known: bool,
}

#[derive(Debug, Clone)]
pub struct Package(Rc<PackageDetail>);
impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", self.0.fqn)
    }
}
impl IntoNode for Package {
    fn into_node(self) -> Node {
        Node::Package(self)
    }
}

impl Package {
    pub fn new(name: &str) -> Self {
        let fqn = if name.is_empty() {
            "".to_string()
        } else {
            format!(".{}", name)
        };

        Self(Rc::new(PackageDetail {
            fqn,
            files: Rc::new(RefCell::new(vec![])),
            is_well_known: name == WELL_KNNOWN_TYPE_PACKAGE,
        }))
    }

    pub fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
    }
    pub fn name(&self) -> &str {
        &self.0.name
    }
    pub fn nodes(&self) -> Nodes {
        Nodes::new(vec![self.files().into()])
    }
    pub fn all_nodes(&self) -> AllNodes {
        AllNodes::new(self.clone().into())
    }
    pub fn files(&self) -> Iter<File> {
        Iter::from(&self.0.files)
    }
    // pub(crate) fn add_extension(&self, extension: Extension) {
    //     self.0.extensions.borrow_mut().push(extension);
    // }
    // pub fn extensions(&self) -> Iter<Extension> {
    //     Iter::from(&self.0.extensions)
    // }
    pub fn is_well_known_type(&self) -> bool {
        self.0.is_well_known
    }
    pub(crate) fn add_file(&self, file: File) {
        self.0.files.borrow_mut().push(file.clone());
    }
    fn downgrade(&self) -> WeakPackage {
        WeakPackage(Rc::downgrade(&self.0))
    }
}

impl From<WeakPackage> for Package {
    fn from(pkg: WeakPackage) -> Self {
        pkg.upgrade()
    }
}
impl From<&WeakPackage> for Package {
    fn from(pkg: &WeakPackage) -> Self {
        pkg.upgrade()
    }
}

#[derive(Debug, Clone)]
pub struct WeakPackage(Weak<PackageDetail>);
impl WeakPackage {
    pub fn upgrade(&self) -> Package {
        Package(self.0.upgrade().expect("Failed to upgrade weak package"))
    }
}

impl From<Package> for WeakPackage {
    fn from(pkg: Package) -> Self {
        pkg.downgrade()
    }
}
impl From<&Package> for WeakPackage {
    fn from(pkg: &Package) -> Self {
        pkg.downgrade()
    }
}
