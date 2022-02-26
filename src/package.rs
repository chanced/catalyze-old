use crate::iter::Iter;
pub use crate::File;
use crate::Name;

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::*;

#[derive(Debug, Clone)]
struct PackageDetail<'a, U> {
    name: Name<U>,
    util: Rc<RefCell<U>>,
    files: Rc<RefCell<Vec<File<'a, U>>>>,
    comments: Rc<RefCell<Vec<String>>>,
}

#[derive(Debug)]
pub struct Package<'a, U>(Rc<PackageDetail<'a, U>>);

impl<'a, U> Clone for Package<'a, U> {
    fn clone(&self) -> Self {
        Package(self.0.clone())
    }
}

impl<'a, U> Package<'a, U> {
    pub(crate) fn downgrade(pkg: Option<Package<'a, U>>) -> Option<WeakPackage<'a, U>> {
        pkg.map(|p| p.into())
    }

    pub(crate) fn new(name: &str, util: Rc<RefCell<U>>) -> Self {
        Self(Rc::new(PackageDetail {
            name: Name::new(name, util.clone()),
            files: Rc::new(RefCell::new(vec![])),
            comments: Rc::new(RefCell::new(Vec::default())),
            util,
        }))
    }

    pub(crate) fn add_file(&self, file: File<'a, U>) {
        self.0.files.borrow_mut().push(file);
    }

    pub fn files(&self) -> impl Iterator<Item = File<'a, U>> {
        Iter::from(&self.0.files)
    }
    pub fn is_well_known(&self) -> bool {
        self.0.name.is_well_known_package()
    }

    pub fn comments(&self) -> slice::Iter<String> {
        self.0.comments.borrow().iter()
    }
}

impl<'a, U> Into<WeakPackage<'a, U>> for Package<'a, U> {
    fn into(self) -> WeakPackage<'a, U> {
        WeakPackage(Rc::downgrade(&self.0))
    }
}

#[derive(Debug)]
pub struct WeakPackage<'a, U>(Weak<PackageDetail<'a, U>>);
impl<'a, U> Clone for WeakPackage<'a, U> {
    fn clone(&self) -> Self {
        WeakPackage(self.0.clone())
    }
}

impl<'a, U> WeakPackage<'a, U> {
    pub(crate) fn upgrade(&self) -> Package<'a, U> {
        Package(self.0.upgrade().unwrap())
    }
}
