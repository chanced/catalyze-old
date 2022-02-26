use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    container::{Container, WeakContainer},
    format_fqn,
    iter::Iter,
    traits::{Downgrade, Upgrade},
    Field, FullyQualified, Name, Node, NodeAtPath,
};

pub(crate) type OneofList<'a, U> = Rc<RefCell<Vec<Oneof<'a, U>>>>;

#[derive(Debug, Clone)]
pub struct OneofDetail<'a, U> {
    pub name: Name<U>,
    pub descriptor: &'a prost_types::OneofDescriptorProto,
    fqn: String,
    fields: Rc<RefCell<Vec<Field<'a, U>>>>,
    container: WeakContainer<'a, U>,
}

#[derive(Debug)]
pub struct Oneof<'a, U>(Rc<OneofDetail<'a, U>>);
impl<'a, U> Clone for Oneof<'a, U> {
    fn clone(&self) -> Self {
        Oneof(self.0.clone())
    }
}

impl<'a, U> Oneof<'a, U> {
    pub(crate) fn new(
        desc: &'a prost_types::OneofDescriptorProto,
        container: Container<'a, U>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        Oneof(Rc::new(OneofDetail {
            name: Name::new(desc.name(), util),
            fqn: format_fqn(&container, desc.name()),
            descriptor: desc,
            container: container.downgrade(),
            fields: Rc::new(RefCell::new(Vec::new())),
        }))
    }
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }

    pub fn container(&self) -> Container<'a, U> {
        self.0.container.upgrade()
    }

    pub fn fields(&self) -> Iter<Field<'a, U>> {
        Iter::from(&self.0.fields)
    }
    pub(crate) fn add_field(&self, field: Field<'a, U>) {
        self.0.fields.borrow_mut().push(field);
    }
}
impl<'a, U> NodeAtPath<'a, U> for Oneof<'a, U> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            Some(Node::Oneof(self.clone()))
        } else {
            None
        }
    }
}

impl<'a, U> Downgrade for Oneof<'a, U> {
    type Target = WeakOneof<'a, U>;
    fn downgrade(&self) -> Self::Target {
        WeakOneof(Rc::downgrade(&self.0))
    }
}

impl<'a, U> FullyQualified for Oneof<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}

pub(crate) struct WeakOneof<'a, U>(Weak<OneofDetail<'a, U>>);

impl<'a, U> WeakOneof<'a, U> {
    pub(crate) fn upgrade(&self) -> Oneof<'a, U> {
        Oneof(self.0.upgrade().unwrap())
    }
}
impl<'a, U> Upgrade for WeakOneof<'a, U> {
    type Target = Oneof<'a, U>;
    fn upgrade(&self) -> Self::Target {
        Oneof(self.0.upgrade().unwrap())
    }
}
