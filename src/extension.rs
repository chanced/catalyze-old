use std::{cell::RefCell, rc::Rc};

use prost_types::FieldDescriptorProto;

use crate::{
    container::{Container, WeakContainer},
    format_fqn, FullyQualified, Name,
};

pub(crate) type ExtensionList<'a, U> = Rc<RefCell<Vec<Extension<'a, U>>>>;
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
    fqn: String,
    container: WeakContainer<'a, U>,
}

impl<'a, U> Extension<'a, U> {
    pub(crate) fn new(
        desc: &'a FieldDescriptorProto,
        container: Container<'a, U>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let ext = Rc::new(Self {
            name: Name::new(desc.name(), util),
            descriptor: desc,
            container: container.downgrade(),
            fqn: format_fqn(&container, desc.name()),
        });
        ext
    }
    pub fn name(&self) -> Name<U> {
        self.name.clone()
    }
}

impl<U> FullyQualified for Extension<'_, U> {
    fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
}
