use crate::iter::Iter;
pub use crate::File;
use crate::{AllNodes, Name, Nodes, WELL_KNNOWN_TYPE_PACKAGE};

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::*;

#[derive(Debug, Clone)]
struct PackageDetail<'a> {
    name: Name,
    fqn: String,

    files: Rc<RefCell<Vec<File<'a>>>>,
    is_wk: bool,
}

#[derive(Debug)]
pub struct Package<'a>(Rc<PackageDetail<'a>>);

impl<'a> Clone for Package<'a> {
    fn clone(&self) -> Self {
        Package(self.0.clone())
    }
}

impl<'a> Package<'a> {
    pub fn new(name: &str) -> Self {
        let fqn = if name.is_empty() {
            "".to_string()
        } else {
            format!(".{}", name)
        };

        Self(Rc::new(PackageDetail {
            fqn,
            name: Name::new(name),
            files: Rc::new(RefCell::new(vec![])),
            is_wk: name == WELL_KNNOWN_TYPE_PACKAGE,
        }))
    }

    pub fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
    pub fn name(&self) -> Name {
        self.0.name.clone()
    }
    pub fn nodes(&self) -> Nodes<'a> {
        Nodes::new(vec![self.files().into()])
    }
    pub fn all_nodes(&self) -> AllNodes<'a> {
        AllNodes::new(self.clone().into())
    }

    pub fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
    pub fn files(&self) -> Iter<File<'a>> {
        Iter::from(&self.0.files)
    }
    // pub(crate) fn add_extension(&self, extension: Extension<'a>) {
    //     self.0.extensions.borrow_mut().push(extension);
    // }
    // pub fn extensions(&self) -> Iter<Extension<'a>> {
    //     Iter::from(&self.0.extensions)
    // }
    pub fn is_well_known_type(&self) -> bool {
        self.0.is_wk
    }
    pub(crate) fn add_file(&self, file: File<'a>) {
        self.0.files.borrow_mut().push(file.clone());
    }
    fn downgrade(&self) -> WeakPackage<'a> {
        WeakPackage(Rc::downgrade(&self.0))
    }
}

impl<'a> From<WeakPackage<'a>> for Package<'a> {
    fn from(pkg: WeakPackage<'a>) -> Self {
        pkg.upgrade()
    }
}
impl<'a> From<&WeakPackage<'a>> for Package<'a> {
    fn from(pkg: &WeakPackage<'a>) -> Self {
        pkg.upgrade()
    }
}

#[derive(Debug)]
pub struct WeakPackage<'a>(Weak<PackageDetail<'a>>);
impl<'a> WeakPackage<'a> {
    pub fn upgrade(&self) -> Package<'a> {
        Package(self.0.upgrade().expect("Failed to upgrade weak package"))
    }
}
impl<'a> Clone for WeakPackage<'a> {
    fn clone(&self) -> Self {
        WeakPackage(self.0.clone())
    }
}

impl<'a> From<Package<'a>> for WeakPackage<'a> {
    fn from(pkg: Package<'a>) -> Self {
        pkg.downgrade()
    }
}
impl<'a> From<&Package<'a>> for WeakPackage<'a> {
    fn from(pkg: &Package<'a>) -> Self {
        pkg.downgrade()
    }
}
