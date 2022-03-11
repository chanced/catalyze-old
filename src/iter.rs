use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Iter<T> {
    nodes: Rc<RefCell<Vec<T>>>,
    idx: usize,
}

impl<T: Clone> Iter<T> {
    pub fn empty(nodes: Vec<T>) -> Self {
        Self {
            nodes: Rc::new(RefCell::new(nodes)),
            idx: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.nodes.borrow().len()
    }
    pub fn is_empty(&self) -> bool {
        self.nodes.borrow().is_empty()
    }

    pub fn get(&self, idx: usize) -> Option<T> {
        self.nodes.borrow().get(idx).cloned()
    }
}

impl<T> Iterator for Iter<T>
where
    T: Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        self.nodes.borrow().get(self.idx - 1).cloned()
    }
}

impl<T> From<&Rc<RefCell<Vec<T>>>> for Iter<T> {
    fn from(nodes: &Rc<RefCell<Vec<T>>>) -> Self {
        Iter {
            nodes: nodes.clone(),
            idx: 0,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{util::Generic, File, Name, Package};
//     fn msg(
//         name: &str,
//         messages: Vec<Rc<Message<'_, Generic>>>,
//         enums: Vec<Rc<Enum<'_, Generic>>>,
//     ) -> Rc<Message<'_, Generic>> {
//         Rc::new(Message {
//             name: Name::new(name, Rc::new(RefCell::new(Generic {}))),
//             messages,
//             enums,
//             ..Message::<'_, Generic>::default()
//         })
//     }
//     fn enums(names: Vec<&str>) -> Vec<Rc<Enum<'_, Generic>>> {
//         names
//             .iter()
//             .map(|name| {
//                 Rc::new(Enum {
//                     name: Name::new(name, Generic),
//                     ..Enum::<'_, Generic>::default()
//                 })
//             })
//             .collect()
//     }

//     fn init() -> File<'_, Generic> {
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
//             ..File::<'_, Generic>::default()
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

//         let create_file = |name: &str, dep: Option<Rc<File<'_, Generic>>>| {
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
