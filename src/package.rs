use crate::file::{new_file_list, FileList};
use crate::util::Generic;
pub use crate::File;
use crate::Name;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Package<U> {
    pub(crate) files: FileList<U>,
    pub name: Name<U>,
    pub(crate) comments: Rc<RefCell<Vec<String>>>,
}

impl Default for Package<Generic> {
    fn default() -> Self {
        Self {
            files: Default::default(),
            name: Default::default(),
            comments: Default::default(),
        }
    }
}

impl<U> Package<U> {
    pub(crate) fn new(name: &str, util: U) -> Rc<Self> {
        let name = Name::new(name, util);

        Rc::new(Self {
            name,
            files: new_file_list(),
            comments: Rc::new(RefCell::new(Vec::default())),
        })
    }

    pub(crate) fn add_file(&self, file: Rc<File<U>>) {
        self.files.borrow_mut().push(file);
    }
    pub fn files(&self) -> Vec<Rc<File<U>>> {
        self.files.borrow().iter().map(Rc::clone).collect()
    }
    pub fn is_well_known(&self) -> bool {
        self.name.is_well_known_package()
    }
    pub fn comments(&self) -> Vec<String> {
        self.comments.borrow().clone()
    }
}
