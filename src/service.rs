use std::{cell::RefCell, rc::Rc};

use crate::{Method, Name};

pub(crate) type ServiceList<U> = Rc<RefCell<Vec<Rc<Service<U>>>>>;

#[derive(Debug, Clone)]
pub struct Service<U> {
    pub name: Name<U>,
    methods: Vec<Rc<Method<U>>>,
}
