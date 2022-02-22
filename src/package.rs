use petgraph::visit::Walker;

use crate::util::Generic;
use crate::visit::{Accept, Visitor};
pub use crate::File;
use crate::Name;

use std::cell::RefCell;
use std::rc::Rc;
use std::vec::IntoIter;
use std::*;
#[derive(Debug, Clone)]
pub struct Package<'a, U> {
    pub(crate) files: Rc<RefCell<Vec<Rc<File<'a, U>>>>>,
    pub name: Name<U>,
    pub(crate) comments: Rc<RefCell<Vec<String>>>,
    util: Rc<RefCell<U>>,
}

impl Default for Package<'_, Generic> {
    fn default() -> Self {
        Self {
            files: Default::default(),
            name: Default::default(),
            comments: Default::default(),
            util: Default::default(),
        }
    }
}

impl<'a, U> Package<'a, U> {
    pub(crate) fn new(name: &str, util: Rc<RefCell<U>>) -> Rc<Self> {
        let mut pkg = Self {
            name: Name::new(name, util.clone()),
            files: Rc::new(RefCell::new(vec![])),
            comments: Rc::new(RefCell::new(Vec::default())),
            util,
        };

        return Rc::new(pkg);
    }

    pub(crate) fn add_file(&self, file: Rc<File<'a, U>>) {
        self.files.borrow_mut().push(file);
    }
    pub fn files(&self) -> impl Iterator<Item = Rc<File<'a, U>>> {
        self.files
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<Rc<File<'a, U>>>>()
            .into_iter()
    }
    pub fn is_well_known(&self) -> bool {
        self.name.is_well_known_package()
    }
    pub fn comments(&self) -> Vec<String> {
        self.comments.borrow().clone()
    }
}

impl<'a, U, V: Visitor<'a, U>> Accept<'a, U, V> for Rc<Package<'a, U>> {
    fn accept(&self, v: &mut V) -> Result<(), V::Error> {
        if v.done() {
            return Ok(());
        }
        v.visit_package(self.clone())?;
        for file in self.files() {
            file.accept(v)?;
        }
        Ok(())
    }
}
