use crate::iter::Iter;
pub use crate::File;
use crate::{FullyQualified, Name, NodeAtPath, PackageComments};

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::*;

#[derive(Debug, Clone)]
struct PackageDetail<'a, U> {
    name: Name<U>,
    fqn: String,
    util: RefCell<Rc<U>>,
    files: Rc<RefCell<Vec<File<'a, U>>>>,
}

#[derive(Debug)]
pub struct Package<'a, U>(Rc<PackageDetail<'a, U>>);

impl<'a, U> Clone for Package<'a, U> {
    fn clone(&self) -> Self {
        Package(self.0.clone())
    }
}

impl<'a, U> Package<'a, U> {
    pub fn new(name: &str, util: Rc<U>) -> Self {
        let fqn = if name == "" {
            "".to_string()
        } else {
            format!(".{}", name)
        };
        Self(Rc::new(PackageDetail {
            fqn,
            name: Name::new(name, util.clone()),
            files: Rc::new(RefCell::new(vec![])),
            util: RefCell::new(util),
        }))
    }

    pub fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
    pub fn util(&self) -> Rc<U> {
        self.0.util.borrow().clone()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.0.util.replace(util);
    }
    pub(crate) fn add_file(&self, file: File<'a, U>) {
        self.0.files.borrow_mut().push(file.clone());
    }

    pub fn files(&self) -> Iter<File<'a, U>> {
        Iter::from(&self.0.files)
    }
    pub fn is_well_known(&self) -> bool {
        self.0.name.is_well_known_package()
    }
    fn downgrade(&self) -> WeakPackage<'a, U> {
        WeakPackage(Rc::downgrade(&self.0))
    }
}
impl<'a, U> FullyQualified for Package<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}

impl<'a, U> From<WeakPackage<'a, U>> for Package<'a, U> {
    fn from(pkg: WeakPackage<'a, U>) -> Self {
        pkg.upgrade()
    }
}
impl<'a, U> From<&WeakPackage<'a, U>> for Package<'a, U> {
    fn from(pkg: &WeakPackage<'a, U>) -> Self {
        pkg.upgrade()
    }
}

#[cfg(test)]
impl<'a> Default for Package<'a, crate::util::Generic> {
    fn default() -> Self {
        Self::new("", Rc::new(crate::util::Generic::default()))
    }
}

#[derive(Debug)]
pub struct WeakPackage<'a, U>(Weak<PackageDetail<'a, U>>);
impl<'a, U> WeakPackage<'a, U> {
    fn upgrade(&self) -> Package<'a, U> {
        Package(self.0.upgrade().expect("Failed to upgrade weak package"))
    }
}
impl<'a, U> Clone for WeakPackage<'a, U> {
    fn clone(&self) -> Self {
        WeakPackage(self.0.clone())
    }
}

impl<'a, U> From<Package<'a, U>> for WeakPackage<'a, U> {
    fn from(pkg: Package<'a, U>) -> Self {
        pkg.downgrade()
    }
}
impl<'a, U> From<&Package<'a, U>> for WeakPackage<'a, U> {
    fn from(pkg: &Package<'a, U>) -> Self {
        pkg.downgrade()
    }
}
