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
    is_real: bool,
}
#[derive(Debug, Clone)]
pub enum Oneof<'a, U> {
    Real(RealOneof<'a, U>),
    Synthetic(SyntheticOneof<'a, U>),
}

impl<'a, U> Oneof<'a, U> {
    pub fn fields(&self) -> Iter<Field<'a, U>> {
        match self {
            Oneof::Real(o) => o.fields(),
            Oneof::Synthetic(o) => o.fields(),
        }
    }
    pub(crate) fn add_field(&self, field: Field<'a, U>) {
        match self {
            Oneof::Real(o) => o.add_field(field),
            Oneof::Synthetic(o) => o.add_field(field),
            //self.0.fields.borrow_mut().push(field);
        }
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
#[derive(Debug)]
pub struct RealOneof<'a, U>(Rc<OneofDetail<'a, U>>);
impl<'a, U> RealOneof<'a, U> {
    pub fn fields(&self) -> Iter<Field<'a, U>> {
        Iter::from(&self.0.fields)
    }
    pub(crate) fn add_field(&self, field: Field<'a, U>) {
        self.0.fields.borrow_mut().push(field);
    }
}

impl<'a, U> Clone for RealOneof<'a, U> {
    fn clone(&self) -> Self {
        RealOneof(self.0.clone())
    }
}

#[derive(Debug)]
pub struct SyntheticOneof<'a, U>(Rc<OneofDetail<'a, U>>);
impl<'a, U> Clone for SyntheticOneof<'a, U> {
    fn clone(&self) -> Self {
        SyntheticOneof(self.0.clone())
    }
}

impl<'a, U> Downgrade for Oneof<'a, U> {
    type Output = WeakOneof<'a, U>;
    fn downgrade(self) -> Self::Output {
        WeakOneof(Rc::downgrade(&self.0))
    }
}

impl<'a, U> FullyQualified for Oneof<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
    }
}
#[derive(Debug)]
pub(crate) struct WeakOneof<'a, U>(Weak<OneofDetail<'a, U>>);
impl<'a, U> Clone for WeakOneof<'a, U> {
    fn clone(&self) -> Self {
        WeakOneof(self.0.clone())
    }
}
impl<'a, U> WeakOneof<'a, U> {
    pub(crate) fn upgrade(&self) -> Oneof<'a, U> {
        let detail = self.0.upgrade().unwrap();
        if detail.is_real() {
            Oneof::Real(RealOneof(detail))
        } else {
            Oneof::Synthetic(SyntheticOneof(detail))
        }
    }
}
impl<'a, U> Upgrade for WeakOneof<'a, U> {
    type Output = Oneof<'a, U>;
    fn upgrade(self) -> Self::Output {
        Oneof(self.0.upgrade().unwrap())
    }
}
