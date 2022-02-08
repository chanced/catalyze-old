use crate::lang::Lang;
use crate::name::Named;
pub use crate::File;
use crate::Name;

use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Clone)]
pub struct Package<L: Lang> {
    name: Name<L>,
    desc: prost_types::FileDescriptorProto,
    files: RefCell<Vec<Rc<File<L>>>>,
}

impl<L: Lang> Package<L> {
    pub(crate) fn new(desc: prost_types::FileDescriptorProto, lang: L) -> Self {
        let name = Name::new(desc.name.clone().unwrap_or_default(), lang);
        Self {
            name,
            desc,
            files: RefCell::new(Vec::new()),
        }
    }
    pub(crate) fn add_file(&self, file: File<L>) {
        self.files.borrow_mut().push(Rc::new(file));
    }
    pub fn file_descriptor(&self) -> &prost_types::FileDescriptorProto {
        &self.desc
    }
    pub fn files(&self) -> Vec<Rc<File<L>>> {
        self.files.borrow().iter().map(Rc::clone).collect()
    }
    pub fn is_well_known(&self) -> bool {
        self.name.is_well_known_package()
    }
}
impl<L: Lang> Named<L> for Package<L> {
    fn name(&self) -> Name<L> {
        self.name.clone()
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
