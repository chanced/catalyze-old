use std::{cell::RefCell, rc::Rc};

use crate::Name;

pub(crate) type ExtensionList<U> = Rc<RefCell<Vec<Rc<Extension<U>>>>>;

pub(crate) fn new_extension_list<U>() -> ExtensionList<U> {
    Rc::new(RefCell::new(Vec::new()))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extension<U> {
    name: Name<U>,
}
