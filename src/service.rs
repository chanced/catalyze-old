use std::rc::Rc;

use crate::{lang::Lang, Method, Name};

#[derive(Debug)]
pub struct Service<L: Lang> {
    name: Name<L>,
    methods: Vec<Rc<Method<L>>>,
}
