use std::{cell::RefCell, rc::Rc};

use crate::{
    container::{Container, WeakContainer},
    fmt_fqn,
    iter::Iter,
    visit::{Accept, Visitor},
    Field, FullyQualified, Name, Node, NodeAtPath,
};

pub(crate) type OneofList<'a, U> = Rc<RefCell<Vec<Rc<Oneof<'a, U>>>>>;

#[derive(Debug, Clone)]
pub struct Oneof<'a, U> {
    pub name: Name<U>,
    pub descriptor: &'a prost_types::OneofDescriptorProto,
    fqn: String,
    fields: Rc<RefCell<Vec<Rc<Field<'a, U>>>>>,
    container: WeakContainer<'a, U>,
}

impl<'a, U> Oneof<'a, U> {
    pub(crate) fn new(
        desc: &'a prost_types::OneofDescriptorProto,
        container: Container<'a, U>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        Rc::new(Self {
            name: Name::new(desc.name(), util),
            fqn: fmt_fqn(&container, desc.name()),
            descriptor: desc,
            container: container.downgrade(),
            fields: Rc::new(RefCell::new(Vec::new())),
        })
    }

    pub fn container(&self) -> Container<'a, U> {
        self.container.upgrade()
    }

    pub fn fields(&self) -> Iter<Field<'a, U>> {
        Iter::from(&self.fields)
    }
    pub(crate) fn add_field(&self, field: Rc<Field<'a, U>>) {
        self.fields.borrow_mut().push(field);
    }
}
impl<'a, U> NodeAtPath<'a, U> for Rc<Oneof<'a, U>> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            Some(Node::Oneof(self.clone()))
        } else {
            None
        }
    }
}

impl<'a, U, V: Visitor<'a, U>> Accept<'a, U, V> for Rc<Oneof<'a, U>> {
    fn accept(&self, v: &mut V) -> Result<(), V::Error> {
        v.visit_oneof(self.clone())?;
        if v.done() {
            return Ok(());
        }
        for fld in self.fields() {
            fld.accept(v)?;
        }
        Ok(())
    }
}

impl<'a, U> FullyQualified for Oneof<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
}
