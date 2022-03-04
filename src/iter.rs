use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    marker::PhantomData,
    rc::Rc,
};

use crate::{Enum, EnumList, File, Message, MessageList, Name, WeakFile};

#[derive(Debug)]
pub struct Iter<T> {
    nodes: Rc<RefCell<Vec<T>>>,
    idx: usize,
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

#[derive(Debug)]
pub struct TransitiveImports<'a, U> {
    queue: VecDeque<File<'a, U>>,
    processed: HashSet<Name<U>>,
    phantom: PhantomData<&'a U>,
}
impl<'a, U> TransitiveImports<'a, U> {
    pub(crate) fn new(files: Rc<RefCell<Vec<WeakFile<'a, U>>>>) -> Self {
        Self {
            queue: VecDeque::from_iter(files.borrow().iter().map(|f| f.into())),
            processed: HashSet::new(),
            phantom: PhantomData,
        }
    }
}
impl<'a, U> Iterator for TransitiveImports<'a, U> {
    type Item = File<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(file) = self.queue.pop_front() {
            if !self.processed.contains(&file.name()) {
                self.processed.insert(file.name());
                for d in file.imports() {
                    self.queue.push_back(d);
                }
                return Some(file);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct AllMessages<'a, U> {
    q: VecDeque<Message<'a, U>>,
    phantom: PhantomData<&'a U>,
}
impl<'a, U> AllMessages<'a, U> {
    pub(crate) fn new(msgs: MessageList<'a, U>) -> Self {
        Self {
            q: VecDeque::from_iter(msgs.borrow().iter().cloned()),
            phantom: PhantomData,
        }
    }
}
impl<'a, U> Iterator for AllMessages<'a, U> {
    type Item = Message<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(msg) = self.q.pop_front() {
            for v in msg.messages() {
                self.q.push_back(v);
            }
            Some(msg)
        } else {
            None
        }
    }
}

pub struct AllEnums<'a, U> {
    msgs: VecDeque<Message<'a, U>>,
    enums: VecDeque<Enum<'a, U>>,
}
impl<'a, U> AllEnums<'a, U> {
    pub(crate) fn new(enums: EnumList<'a, U>, msgs: MessageList<'a, U>) -> Self {
        Self {
            msgs: msgs.borrow().iter().cloned().collect(),
            enums: enums.borrow().iter().cloned().collect(),
        }
    }
}
impl<'a, U> Iterator for AllEnums<'a, U> {
    type Item = Enum<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.enums.pop_front() {
            Some(e)
        } else {
            while let Some(msg) = self.msgs.pop_front() {
                for v in msg.messages() {
                    self.msgs.push_back(v);
                }
                for v in msg.enums() {
                    self.enums.push_back(v);
                }
                if let Some(e) = self.enums.pop_front() {
                    return Some(e);
                }
            }
            None
        }
    }
}

/// FileRefIter is an iterator that upgrades weak references to `File`s.
pub struct FileRefIter<'a, U> {
    files: Rc<RefCell<Vec<WeakFile<'a, U>>>>,
    index: usize,
}
impl<'a, U> From<&Rc<RefCell<Vec<WeakFile<'a, U>>>>> for FileRefIter<'a, U> {
    fn from(files: &Rc<RefCell<Vec<WeakFile<'a, U>>>>) -> Self {
        FileRefIter {
            files: files.clone(),
            index: 0,
        }
    }
}
impl<'a, U> From<Rc<RefCell<Vec<WeakFile<'a, U>>>>> for FileRefIter<'a, U> {
    fn from(files: Rc<RefCell<Vec<WeakFile<'a, U>>>>) -> Self {
        FileRefIter {
            files: files.clone(),
            index: 0,
        }
    }
}

impl<'a, U> Iterator for FileRefIter<'a, U> {
    type Item = File<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        let files = self.files.borrow();
        if let Some(file) = files.get(self.index) {
            self.index += 1;
            return Some(file.into());
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
