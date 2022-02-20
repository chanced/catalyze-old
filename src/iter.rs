use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    rc::{Rc, Weak},
};

use crate::{File, Name};

pub struct Iter<T> {
    nodes: Rc<RefCell<Vec<Rc<T>>>>,
    idx: usize,
}
impl<T> Iterator for Iter<T> {
    type Item = Rc<T>;
    fn next(&mut self) -> Option<Rc<T>> {
        self.idx += 1;
        self.nodes.borrow().get(self.idx - 1).cloned()
    }
}
impl<T> From<&Rc<RefCell<Vec<Rc<T>>>>> for Iter<T> {
    fn from(nodes: &Rc<RefCell<Vec<Rc<T>>>>) -> Self {
        Iter {
            nodes: nodes.clone(),
            idx: 0,
        }
    }
}

pub struct UpgradeIter<T> {
    nodes: Rc<RefCell<Vec<Weak<T>>>>,
    idx: usize,
}
impl<T> UpgradeIter<T> {
    pub(crate) fn new(nodes: Rc<RefCell<Vec<Weak<T>>>>) -> Self {
        Self { nodes, idx: 0 }
    }
}
impl<T> Iterator for UpgradeIter<T> {
    type Item = Rc<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let nodes = self.nodes.borrow();
        if self.idx < nodes.len() {
            self.idx += 1;
            nodes
                .get(self.idx - 1)
                .cloned()
                .map(|n| n.upgrade().unwrap())
        } else {
            None
        }
    }
}

pub struct TransitiveImports<U> {
    queue: VecDeque<Rc<File<U>>>,
    processed: HashSet<Name<U>>,
}

impl<U> TransitiveImports<U> {
    pub(crate) fn new(files: Rc<RefCell<Vec<Weak<File<U>>>>>) -> Self {
        Self {
            queue: VecDeque::from_iter(files.borrow().iter().map(|f| f.upgrade().unwrap())),
            processed: HashSet::new(),
        }
    }
}

impl<U> Iterator for TransitiveImports<U> {
    type Item = Rc<File<U>>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(file) = self.queue.pop_front() {
            if !self.processed.contains(&file.name) {
                self.processed.insert(file.name.clone());
                for f in file.dependencies.borrow().iter() {
                    self.queue.push_back(f.upgrade().unwrap());
                }
                return Some(file);
            }
        }
        None
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{util::Generic, File, Name, Package};
//     fn msg(
//         name: &str,
//         messages: Vec<Rc<Message<Generic>>>,
//         enums: Vec<Rc<Enum<Generic>>>,
//     ) -> Rc<Message<Generic>> {
//         Rc::new(Message {
//             name: Name::new(name, Rc::new(RefCell::new(Generic {}))),
//             messages,
//             enums,
//             ..Message::<Generic>::default()
//         })
//     }
//     fn enums(names: Vec<&str>) -> Vec<Rc<Enum<Generic>>> {
//         names
//             .iter()
//             .map(|name| {
//                 Rc::new(Enum {
//                     name: Name::new(name, Generic),
//                     ..Enum::<Generic>::default()
//                 })
//             })
//             .collect()
//     }

//     fn init() -> File<Generic> {
//         File {
//             name: Name::new("test.rs", Generic),
//             messages: vec![
//                 msg(
//                     "r1",
//                     vec![
//                         msg(
//                             "s1",
//                             vec![
//                                 msg(
//                                     "s1s1",
//                                     vec![msg("s1s1s1", vec![], enums(vec!["s1s1s1e1"]))],
//                                     enums(vec!["s1s1e1"]),
//                                 ),
//                                 msg("s1s2", vec![], enums(vec!["s1s2e1", "s1s2e2"])),
//                                 msg("s1s3", vec![], enums(vec!["s1s3e1", "s1s3e2"])),
//                             ],
//                             enums(vec!["s1e1", "s1e2"]),
//                         ),
//                         msg(
//                             "s2",
//                             vec![
//                                 msg("s2s1", vec![], enums(vec!["s2s1e1"])),
//                                 msg("s2s2", vec![], enums(vec!["s2s2e1"])),
//                                 msg("s2s3", vec![], vec![]),
//                             ],
//                             enums(vec!["s2e1", "s2e2"]),
//                         ),
//                         msg(
//                             "s3",
//                             vec![
//                                 msg("s3s1", vec![], vec![]),
//                                 msg("s3s2", vec![], vec![]),
//                                 msg("s3s3", vec![], vec![]),
//                             ],
//                             enums(vec!["s3e1", "s3e2"]),
//                         ),
//                     ],
//                     enums(vec!["r1e1", "r1e2"]),
//                 ),
//                 msg("r2", vec![], enums(vec!["r2e1"])),
//             ],
//             enums: enums(vec!["e1", "e2"]),
//             ..File::<Generic>::default()
//         }
//     }

//     #[test]
//     fn test_all_messages() {
//         let f = init();

//         assert_eq!(
//             f.all_messages()
//                 .map(|msg| msg.name.to_string())
//                 .collect::<Vec<String>>(),
//             vec![
//                 "r1", "r2", "s1", "s2", "s3", "s1s1", "s1s2", "s1s3", "s2s1", "s2s2", "s2s3",
//                 "s3s1", "s3s2", "s3s3", "s1s1s1"
//             ]
//         );
//     }

//     #[test]
//     fn test_all_enums() {
//         let f = init();
//         assert_eq!(
//             f.all_enums()
//                 .map(|e| e.name.to_string())
//                 .collect::<Vec<_>>(),
//             vec![
//                 "e1", "e2", "r1e1", "r1e2", "r2e1", "s1e1", "s1e2", "s2e1", "s2e2", "s3e1", "s3e2",
//                 "s1s1e1", "s1s2e1", "s1s2e2", "s1s3e1", "s1s3e2", "s2s1e1", "s2s2e1", "s1s1s1e1"
//             ],
//         );
//     }

//     #[test]
//     fn test_transitive_imports() {
//         let pkg = Rc::new(Package::default());

//         let create_file = |name: &str, dep: Option<Rc<File<Generic>>>| {
//             let f = Rc::new(File {
//                 name: Name::new(name, Generic),
//                 ..File::default()
//             });
//             pkg.add_file(f.clone());
//             if let Some(dep) = dep {
//                 dep.add_dependency(f.clone());
//             }
//             f
//         };
//         let dep1 = create_file("dep1", None);
//         create_file("dep1_d1", Some(dep1.clone()));
//         let dep2 = create_file("dep2", None);
//         create_file("dep2_d1", Some(dep2.clone()));

//         let f1 = create_file("f1", None);
//         f1.add_dependency(dep1.clone());
//         f1.add_dependency(dep2.clone());

//         let f1_d1 = create_file("f1_d1", Some(f1.clone()));
//         let f1_d2 = create_file("f1_d2", Some(f1.clone()));
//         let f1_d1_d1 = create_file("f1_d1_d1", Some(f1_d1.clone()));
//         let f1_d1_d2 = create_file("f1_d1_d2", Some(f1_d1.clone()));
//         let f1_d1_d1_d1 = create_file("f1_d1_d1_d1", Some(f1_d1_d1.clone()));
//         f1_d1_d1.add_dependency(f1_d1_d1_d1);

//         f1_d1.add_dependency(dep1.clone());
//         f1_d1.add_dependency(dep2.clone());
//         f1_d2.add_dependency(dep1.clone());
//         f1_d2.add_dependency(dep2.clone());
//         f1_d1_d2.add_dependency(dep1);
//         f1_d1_d2.add_dependency(dep2);
//         let _f2 = create_file("f2", None);
//         let _f3 = create_file("f3", None);

//         assert_eq!(
//             f1.transitive_imports()
//                 .map(|i| i.name.to_string())
//                 .collect::<Vec<_>>(),
//             vec![
//                 "dep1",
//                 "dep2",
//                 "f1_d1",
//                 "f1_d2",
//                 "dep1_d1",
//                 "dep2_d1",
//                 "f1_d1_d1",
//                 "f1_d1_d2",
//                 "f1_d1_d1_d1",
//             ]
//         );
//     }
// }
