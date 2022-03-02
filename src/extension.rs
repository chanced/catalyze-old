use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use prost_types::FieldDescriptorProto;

use crate::{
    container::{Container, WeakContainer},
    format_fqn, FullyQualified, Name, NodeAtPath,
};

pub(crate) type ExtensionList<'a, U> = Rc<RefCell<Vec<Extension<'a, U>>>>;

#[derive(Debug, Clone)]
struct ExtensionDetail<'a, U> {
    name: Name<U>,
    descriptor: &'a FieldDescriptorProto,
    fqn: String,
    container: WeakContainer<'a, U>,
}

#[derive(Debug)]
pub struct Extension<'a, U>(Rc<ExtensionDetail<'a, U>>);

impl<'a, U> Extension<'a, U> {
    pub(crate) fn new(desc: &'a FieldDescriptorProto, container: Container<'a, U>) -> Self {
        let util = container.util();
        let fqn = format_fqn(&container, desc.name());
        let ext = Self(Rc::new(ExtensionDetail {
            fqn,
            name: Name::new(desc.name(), util.clone()),
            descriptor: desc,
            container: container.into(),
        }));
        ext
    }
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}

impl<U> FullyQualified for Extension<'_, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}
impl<'a, U> Clone for Extension<'a, U> {
    fn clone(&self) -> Self {
        Extension(self.0.clone())
    }
}

#[derive(Debug)]
pub(crate) struct WeakExtension<'a, U>(Weak<ExtensionDetail<'a, U>>);

impl<'a, U> Clone for WeakExtension<'a, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
