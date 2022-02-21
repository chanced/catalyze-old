use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use prost_types::FieldDescriptorProto;

use crate::{File, Name};

pub(crate) type ExtensionList<'a, U> = Rc<RefCell<Vec<Rc<Extension<'a, U>>>>>;
pub(crate) fn new_extension_list<'a, U>(cap: usize) -> ExtensionList<'a, U> {
    match cap {
        0 => Rc::new(RefCell::new(Vec::new())),
        cap => Rc::new(RefCell::new(Vec::with_capacity(cap))),
    }
}

#[derive(Debug, Clone)]
pub struct Extension<'a, U> {
    pub name: Name<U>,
    pub descriptor: &'a FieldDescriptorProto,
    pub(crate) file: Weak<File<'a, U>>,
}

impl<'a, U> Extension<'a, U> {
    pub fn new(
        desc: &'a FieldDescriptorProto,
        file: Rc<File<'a, U>>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let ext = Rc::new(Self {
            name: Name::new(desc.name(), util),
            descriptor: desc,
            file: Rc::downgrade(&file),
        });
        ext
    }
}

impl<'a, U> Extension<'a, U> {
    pub fn file(&self) -> Rc<File<'a, U>> {
        self.file.upgrade().unwrap()
    }
}
