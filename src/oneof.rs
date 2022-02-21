use std::{cell::RefCell, rc::Rc};

use crate::{
    container::{Container, InternalContainer},
    Field, Name, Node,
};

pub(crate) type OneofList<'a, U> = Rc<RefCell<Vec<Rc<Oneof<'a, U>>>>>;
pub(crate) fn new_oneof_list<'a, U>(cap: usize) -> OneofList<'a, U> {
    match cap {
        0 => Rc::new(RefCell::new(Vec::new())),
        cap => Rc::new(RefCell::new(Vec::with_capacity(cap))),
    }
}

#[derive(Debug, Clone)]
pub struct Oneof<'a, U> {
    pub name: Name<U>,
    pub desc: &'a prost_types::OneofDescriptorProto,
    pub fully_qualified_name: String,
    pub fields: Vec<Rc<Field<'a, U>>>,
    container: InternalContainer<'a, U>,
}

impl<'a, U> Oneof<'a, U> {
    // pub(crate) fn new(
    //     desc: &'a prost_types::OneofDescriptorProto,
    //     container: Container<'a, U>,
    //     lang: U,
    // ) -> Self {
    //     Self {
    //         name: Name::new(desc.name(), lang),
    //         desc,
    //         fields: Vec::with_capacity(desc.),
    //         container: container.downgrade(),
    //     }
    // }

    pub fn container(&self) -> Container<'a, U> {
        self.container.upgrade()
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        todo!()
    }

    // pub(crate) fn add_field(&self, field: Rc<Field<'a, U>>) {
    //     self.fields.borrow_mut().push(field);
    // }
}
