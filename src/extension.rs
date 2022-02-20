use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use prost_types::FieldDescriptorProto;

use crate::{File, Name};

pub(crate) type ExtensionList<U> = Rc<RefCell<Vec<Rc<Extension<U>>>>>;
pub(crate) fn new_extension_list<U>(cap: usize) -> ExtensionList<U> {
    match cap {
        0 => Rc::new(RefCell::new(Vec::new())),
        cap => Rc::new(RefCell::new(Vec::with_capacity(cap))),
    }
}

#[derive(Debug, Clone)]
pub struct Extension<U> {
    pub name: Name<U>,
    pub descriptor: FieldDescriptorProto,
    pub(crate) file: Weak<File<U>>,
}

impl<U> Extension<U> {
    pub fn new(desc: FieldDescriptorProto, file: Rc<File<U>>, util: Rc<RefCell<U>>) -> Rc<Self> {
        let ext = Rc::new(Self {
            name: Name::new(desc.name(), util),
            descriptor: desc,
            file: Rc::downgrade(&file),
        });
        ext
    }
}

impl<U> Extension<U> {
    pub fn file(&self) -> Rc<File<U>> {
        self.file.upgrade().unwrap()
    }
}
