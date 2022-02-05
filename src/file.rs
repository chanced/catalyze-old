use std::rc::{Rc, Weak};

use super::Name;
use crate::language::{Language, NotSpecified};
use crate::Package;
use rc::Rc;
use std::path::PathBuf;
#[derive(Debug, Clone)]
pub struct File<L: Language = NotSpecified> {
    desc: prost_types::FileDescriptorProto,
    package: Weak<Package<L>>,
    name: Name<L>,
}

impl<L: Language> File<L> {
    fn new(
        desc: prost_types::FileDescriptorProto,
        package: Rc<Package<L>>,
        lang: L) -> Self {
        let package = Rc::downgrade(&package);
        let name = Name::new(desc.name(), lang);
        Self { desc, name, package}
    }
    pub fn package(&self) -> Rc<Package<L>> {
        // file should not exist outside of the package
        self.package.upgrade().unwrap()
    }
    fn name(&self) -> Name {
        return self.name
    }
    pub(crate)set_package(&mut self, package: Rc<Package<L>>) {
        self.package = Rc::downgrade(&package);
    }
}
