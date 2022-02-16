use std::{cell::RefCell, rc::Rc};

use crate::{
    container::{Container, InternalContainer},
    util::Lang,
    Field, Name,
};

pub(crate) type OneofList<U> = Rc<RefCell<Vec<Rc<Oneof<U>>>>>;

#[derive(Debug, Clone)]
pub struct Oneof<U> {
    pub name: Name<U>,
    pub desc: prost_types::OneofDescriptorProto,
    fields: RefCell<Vec<Rc<Field<U>>>>,
    container: InternalContainer<U>,
}

impl<U> Oneof<U> {
    pub(crate) fn new(
        desc: prost_types::OneofDescriptorProto,
        container: Container<U>,
        lang: U,
    ) -> Self {
        Self {
            name: Name::new(desc.name(), lang),
            desc,
            fields: RefCell::new(Vec::new()),
            container: container.downgrade(),
        }
    }
    pub fn fields(&self) -> Vec<Rc<Field<U>>> {
        self.fields.borrow().clone()
    }
    pub fn container(&self) -> Container<U> {
        self.container.upgrade()
    }
    pub(crate) fn add_field(&self, field: Rc<Field<U>>) {
        self.fields.borrow_mut().push(field);
    }
}
