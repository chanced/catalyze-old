use crate::language::{Language, NotSpecified};
pub use crate::File;

use std::cell::RefCell;
use std::rc::{Rc, Weak};
#[derive(Debug, Clone)]
pub struct Package<L: Language = NotSpecified> {
    fd: prost_types::FileDescriptorProto,
    files: RefCell<Vec<Rc<File<L>>>>,
}

impl<L: Language> Package<L> {
    pub fn add_file(&self, file: File<L>) {
        self.files.borrow_mut().push(Rc::new(file));
    }
}
