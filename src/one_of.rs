use std::{cell::RefCell, rc::Rc};

use crate::{
    container::{Container, InternalContainer},
    util::Lang,
    Field, Name,
};

pub(crate) type OneofList<L> = Rc<RefCell<Vec<Rc<Oneof<L>>>>>;

#[derive(Debug, Clone)]
pub struct Oneof<L> {
    pub name: Name<L>,
    pub desc: prost_types::OneofDescriptorProto,
    fields: RefCell<Vec<Rc<Field<L>>>>,
    container: InternalContainer<L>,
}

impl<L: Lang> Oneof<L> {
    pub(crate) fn new(
        desc: prost_types::OneofDescriptorProto,
        container: Container<L>,
        lang: L,
    ) -> Self {
        Self {
            name: Name::new(desc.name(), lang),
            desc,
            fields: RefCell::new(Vec::new()),
            container: container.downgrade(),
        }
    }
    pub fn fields(&self) -> Vec<Rc<Field<L>>> {
        self.fields.borrow().clone()
    }
    pub fn container(&self) -> Container<L> {
        self.container.upgrade()
    }
    pub(crate) fn add_field(&self, field: Rc<Field<L>>) {
        self.fields.borrow_mut().push(field);
    }
}
