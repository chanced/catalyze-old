use std::{cell::RefCell, rc::Rc};

use crate::{lang::Lang, Method, Name};

pub(crate) type ServiceList<L> = Rc<RefCell<Vec<Rc<Service<L>>>>>;

#[derive(Debug, Clone)]
pub struct Service<L: Lang> {
    pub name: Name<L>,
    methods: Vec<Rc<Method<L>>>,
}
