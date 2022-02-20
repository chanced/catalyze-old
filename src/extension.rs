use std::{cell::RefCell, rc::Rc};

use prost_types::FieldDescriptorProto;

use crate::Name;

pub(crate) type ExtensionList<U> = Rc<RefCell<Vec<Rc<Extension<U>>>>>;
pub(crate) fn new_extension_list<U>(cap: usize) -> ExtensionList<U> {
    match cap {
        0 => Rc::new(RefCell::new(Vec::new())),
        cap => Rc::new(RefCell::new(Vec::with_capacity(cap))),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extension<U> {
    name: Name<U>,
    placeholder_until_i_fill_in_this_field: String,
    // TODO: see above.
}

impl<U> Extension<U> {
    pub fn new(descriptor: Rc<FieldDescriptorProto>, util: Rc<RefCell<U>>) -> Self {
        Self {
            name: Name::new(descriptor.name(), util),
            placeholder_until_i_fill_in_this_field: String::new(),
        }
    }
}
