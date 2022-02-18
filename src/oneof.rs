use std::rc::Rc;

use crate::{
    container::{Container, InternalContainer},
    Field, Name, Node,
};

#[derive(Debug, Clone)]
pub struct Oneof<U> {
    pub name: Name<U>,
    pub desc: prost_types::OneofDescriptorProto,
    pub fully_qualified_name: String,
    pub fields: Vec<Rc<Field<U>>>,
    container: InternalContainer<U>,
}

impl<U> Oneof<U> {
    // pub(crate) fn new(
    //     desc: prost_types::OneofDescriptorProto,
    //     container: Container<U>,
    //     lang: U,
    // ) -> Self {
    //     Self {
    //         name: Name::new(desc.name(), lang),
    //         desc,
    //         fields: Vec::with_capacity(desc.),
    //         container: container.downgrade(),
    //     }
    // }

    pub fn container(&self) -> Container<U> {
        self.container.upgrade()
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<U>> {
        todo!()
    }

    // pub(crate) fn add_field(&self, field: Rc<Field<U>>) {
    //     self.fields.borrow_mut().push(field);
    // }
}
