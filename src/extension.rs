use std::{cell::RefCell, rc::Rc};

use crate::{util::Lang, Name};

pub(crate) type ExtensionList<L> = Rc<RefCell<Vec<Rc<Extension<L>>>>>;

pub(crate) fn new_extension_list<L: Lang>() -> ExtensionList<L> {
    Rc::new(RefCell::new(Vec::new()))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extension<L> {
    name: Name<L>,
}
