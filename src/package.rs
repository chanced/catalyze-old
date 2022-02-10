use crate::lang::Lang;
pub use crate::File;
use crate::Name;

use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Clone)]
pub struct Package<L: Lang> {
    descriptor: prost_types::FileDescriptorProto,
    files: RefCell<Vec<Rc<File<L>>>>,
    lang: L,
}

impl<L: Lang> Package<L> {
    pub(crate) fn new(descriptor: prost_types::FileDescriptorProto, lang: L) -> Self {
        let name = Name::new(descriptor.name.clone().unwrap_or_default(), lang);
        Self {
            descriptor,
            lang,
            files: RefCell::new(Vec::new()),
        }
    }
    pub(crate) fn add_file(&self, file: File<L>) {
        self.files.borrow_mut().push(Rc::new(file));
    }
    fn name(&self) -> Name<L> {
        Name::new(self.descriptor.name(), self.lang.clone())
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
