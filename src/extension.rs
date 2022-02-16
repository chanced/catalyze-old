use std::{cell::RefCell, rc::Rc};

use crate::{lang::Lang, Name};

pub(crate) type ExtensionList<L> = Rc<RefCell<Vec<Rc<Extension<L>>>>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extension<L: Lang> {
    name: Name<L>,
}
