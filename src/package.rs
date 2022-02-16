use crate::file::{new_file_list, FileList};
use crate::util::{Lang, Unspecified};
pub use crate::File;
use crate::Name;

use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Clone)]
pub struct Package<L> {
    pub descriptor: prost_types::FileDescriptorProto,
    pub(crate) files: FileList<L>,
    pub name: Name<L>,
}

impl Default for Package<Unspecified> {
    fn default() -> Self {
        Self {
            descriptor: Default::default(),
            files: Default::default(),
            name: Default::default(),
        }
    }
}

impl<L> Package<L> {
    pub(crate) fn new(descriptor: prost_types::FileDescriptorProto, lang: L) -> Rc<Self> {
        let name = Name::new(descriptor.name(), lang);
        Rc::new(Self {
            name,
            descriptor,
            files: new_file_list(),
        })
    }

    pub(crate) fn add_file(&self, file: Rc<File<L>>) {
        self.files.borrow_mut().push(file);
    }
    pub fn file_descriptor(&self) -> &prost_types::FileDescriptorProto {
        &self.descriptor
    }
    pub fn files(&self) -> Vec<Rc<File<L>>> {
        self.files.borrow().iter().map(Rc::clone).collect()
    }
    pub fn is_well_known(&self) -> bool {
        self.name.is_well_known_package()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_files() {
//         let mut p = Package {
//             fd: prost_types::FileDescriptorProto {
//                 name: Some("".to_string()),
//                 ..Default::default()
//             },
//             files: vec![],
//             name: Name::new("".to_string(), syntax::NotSpecified),
//         };

//         p.add_file(File {
//             desc: prost_types::FileDescriptorProto {
//                 name: Some("".to_string()),
//                 ..Default::default()
//             },
//             name: Name::new("".to_string(), syntax::NotSpecified),
//             package: Rc::downgrade(&r),
//         });
//         let r = Rc::new(p);
//         for f in r.files() {
//             println!("{:?}", f);
//         }
//     }
// }
