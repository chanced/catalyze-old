use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{container::InternalContainer, util::Generic, Message, Name, Node, Package};

#[derive(Debug, Clone)]
pub struct Enum<U> {
    pub name: Name<U>,
    pub fully_qualified_name: String,
    pub(crate) container: InternalContainer<U>,
    pub(crate) dependents: Rc<RefCell<Vec<Weak<Message<U>>>>>,
}

impl<U> Enum<U> {
    // pub fn new(name: Name<U>, container: Container<U>) -> Self {
    //     Enum {
    //         name,

    //         container: container.downgrade(),
    //         dependents: Rc::new(RefCell::new(Vec::default())),
    //     }
    // }

    pub fn package(&self) -> Option<Rc<Package<U>>> {
        self.container.package()
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<U>> {
        todo!()
    }
}

impl Default for Enum<Generic> {
    fn default() -> Self {
        Self {
            name: Name::default(),
            fully_qualified_name: String::default(),
            container: InternalContainer::File(Weak::new()),
            dependents: Rc::new(RefCell::new(Vec::default())),
        }
    }
}
