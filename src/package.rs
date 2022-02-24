use crate::util::Generic;
use crate::visit::Visitor;
pub use crate::File;
use crate::Name;

use std::cell::RefCell;
use std::rc::Rc;
use std::*;
#[derive(Debug, Clone)]
pub struct Package<'a, U> {
    pub name: Name<U>,
    pub util: Rc<RefCell<U>>,
    pub(crate) files: Rc<RefCell<Vec<Rc<File<'a, U>>>>>,
    pub(crate) comments: Rc<RefCell<Vec<String>>>,
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
        Rc::new(Self {
            name: Name::new(name, util.clone()),
            files: Rc::new(RefCell::new(vec![])),
            comments: Rc::new(RefCell::new(Vec::default())),
            util,
        })
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
