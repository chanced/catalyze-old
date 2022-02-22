use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use prost_types::FieldDescriptorProto;

use crate::{
    container::{Container, WeakContainer},
    visit::{Accept, Visitor},
    File, Name,
};

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
    pub(crate) container: WeakContainer<'a, U>,
}

impl<'a, U> Extension<'a, U> {
    pub fn new(
        desc: &'a FieldDescriptorProto,
        container: Container<'a, U>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let ext = Rc::new(Self {
            name: Name::new(desc.name(), util),
            descriptor: desc,
            container: container.downgrade(),
        });
        ext
    }
}

impl<'a, U, V: Visitor<'a, U>> Accept<'a, U, V> for Rc<Extension<'a, U>> {
    fn accept(&self, v: &mut V) -> Result<(), V::Error> {
        if v.done() {
            return Ok(());
        }
        v.visit_extension(self.clone())?;
        Ok(())
    }
}
