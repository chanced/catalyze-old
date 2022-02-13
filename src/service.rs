use std::rc::Rc;

use crate::{lang::Lang, Method, Name};

#[derive(Debug, Clone)]
pub struct Service<L: Lang> {
    pub name: Name<L>,
    methods: Vec<Rc<Method<L>>>,
}
