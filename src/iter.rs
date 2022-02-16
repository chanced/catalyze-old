use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    rc::{Rc, Weak},
};

use crate::{file::WeakFileList, Enum, EnumList, File, Lang, Message, MessageList, Name};

pub struct UpgradeIter<T> {
    nodes: Rc<RefCell<Vec<Weak<T>>>>,
    idx: usize,
    size: usize,
}
impl<T> UpgradeIter<T> {
    pub(crate) fn new(nodes: Rc<RefCell<Vec<Weak<T>>>>) -> Self {
        let size = nodes.borrow().len();
        Self {
            nodes,
            idx: 0,
            size,
        }
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
pub struct Iter<T> {
    nodes: Rc<RefCell<Vec<Rc<T>>>>,
    idx: usize,
}

impl<T> Iter<T> {
    pub(crate) fn new(nodes: Rc<RefCell<Vec<Rc<T>>>>) -> Self {
        Self { nodes, idx: 0 }
    }
}

impl<T> Iterator for Iter<T> {
    type Item = Rc<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let nodes = self.nodes.borrow();
        if self.idx < nodes.len() {
            self.idx += 1;
            nodes.get(self.idx - 1).cloned()
        } else {
            None
        }
    }
}

pub struct AllMessages<L> {
    q: VecDeque<Rc<Message<L>>>,
}

impl<L> AllMessages<L> {
    pub(crate) fn new(msgs: MessageList<L>) -> Self {
        Self {
            q: VecDeque::from_iter(msgs.borrow().iter().cloned()),
        }
    }
}

impl<L> Iterator for AllMessages<L> {
    type Item = Rc<Message<L>>;
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
pub struct AllEnums<L> {
    msgs: VecDeque<Rc<Message<L>>>,
    enums: VecDeque<Rc<Enum<L>>>,
}

impl<L> AllEnums<L> {
    pub(crate) fn new(enums: EnumList<L>, msgs: MessageList<L>) -> Self {
        Self {
            msgs: VecDeque::from_iter(msgs.borrow().iter().cloned()),
            enums: VecDeque::from_iter(enums.borrow().iter().cloned()),
        }
    }
}

impl<L> Iterator for AllEnums<L> {
    type Item = Rc<Enum<L>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(enum_) = self.enums.pop_front() {
            Some(enum_)
        } else {
            while let Some(msg) = self.msgs.pop_front() {
                for v in msg.messages() {
                    self.msgs.push_back(v);
                }
                for v in msg.enums() {
                    self.enums.push_back(v);
                }
                if let Some(enum_) = self.enums.pop_front() {
                    return Some(enum_);
                }
            }
            None
        }
    }
}

pub struct TransitiveImports<L> {
    queue: VecDeque<Rc<File<L>>>,
    processed: HashSet<Name<L>>,
}

impl<L> TransitiveImports<L> {
    pub(crate) fn new(files: WeakFileList<L>) -> Self {
        Self {
            queue: VecDeque::from_iter(files.borrow().iter().map(|f| f.upgrade().unwrap())),
            processed: HashSet::new(),
        }
    }
}

impl<L: Clone> Iterator for TransitiveImports<L> {
    type Item = Rc<File<L>>;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{container::Container, lang::Unspecified, File, Name};
    use std::collections::HashMap;

    type MsgTable = HashMap<String, Rc<Message<Unspecified>>>;
    fn init() -> (Rc<File<Unspecified>>, MsgTable) {
        let mut table = HashMap::new();
        let f = Rc::new(File {
            name: Name::new("test.rs", Unspecified),
            ..File::<Unspecified>::default()
        });
        let mut create_msg =
            |name: &str, container: Container<Unspecified>| -> Rc<Message<Unspecified>> {
                let m = Rc::new(Message {
                    name: Name::new(name, Unspecified),
                    container: container.downgrade(),
                    ..Message::<Unspecified>::default()
                });
                container.add_message(m.clone());
                table.insert(name.to_string(), m.clone());
                m
            };
        let create_enum =
            |name: &str, container: Container<Unspecified>| -> Rc<Enum<Unspecified>> {
                let e = Rc::new(Enum {
                    name: Name::new(name, Unspecified),
                    ..Enum::<Unspecified>::default()
                });
                container.add_enum(e.clone());
                e
            };
        let r1 = create_msg("r1", Container::from(f.clone()));
        let r2 = create_msg("r2", Container::from(f.clone()));

        let s1 = create_msg("s1", Container::from(r1.clone()));
        let s2 = create_msg("s2", Container::from(r1.clone()));
        let s3 = create_msg("s3", Container::from(r1.clone()));
        let s1s1 = create_msg("s1s1", Container::from(s1.clone()));
        let s1s2 = create_msg("s1s2", Container::from(s1.clone()));
        let _s2s1 = create_msg("s1s3", Container::from(s1.clone()));
        let s2s1 = create_msg("s2s1", Container::from(s2.clone()));
        let _s2s2 = create_msg("s2s2", Container::from(s2.clone()));
        let _s3s1 = create_msg("s2s3", Container::from(s2.clone()));
        let s3s1 = create_msg("s3s1", Container::from(s3.clone()));
        create_msg("s3s2", Container::from(s3.clone()));
        create_msg("s3s3", Container::from(s3.clone()));

        create_enum("e1", Container::from(f.clone()));
        create_enum("e2", Container::from(f.clone()));
        create_enum("r1e1", Container::from(r1));
        create_enum("r1e2", Container::from(r2));
        create_enum("s1e1", Container::from(s1.clone()));
        create_enum("s1e2", Container::from(s1));
        create_enum("s2e1", Container::from(s2.clone()));
        create_enum("s2e2", Container::from(s2));
        create_enum("s3e1", Container::from(s3));
        create_enum("s1s1e1", Container::from(s1s1));
        create_enum("s1s2e1", Container::from(s1s2));
        create_enum("s2s1e1", Container::from(s2s1));
        create_enum("s3s1e1", Container::from(s3s1));
        (f, table)
    }

    #[test]
    fn test_all_messages() {
        let (f, _t) = init();

        assert_eq!(
            f.all_messages()
                .map(|msg| msg.name.to_string())
                .collect::<Vec<String>>(),
            vec![
                "r1".to_string(),
                "r2".to_string(),
                "s1".to_string(),
                "s2".to_string(),
                "s3".to_string(),
                "s1s1".to_string(),
                "s1s2".to_string(),
                "s1s3".to_string(),
                "s2s1".to_string(),
                "s2s2".to_string(),
                "s2s3".to_string(),
                "s3s1".to_string(),
                "s3s2".to_string(),
                "s3s3".to_string(),
            ]
        );
    }

    #[test]
    fn test_all_enums() {
        let (f, _t) = init();
        assert_eq!(
            f.all_enums()
                .map(|e| e.name.to_string())
                .collect::<Vec<String>>(),
            vec![
                "e1".to_string(),
                "e2".to_string(),
                "r1e1".to_string(),
                "r1e2".to_string(),
                "s1e1".to_string(),
                "s1e2".to_string(),
                "s2e1".to_string(),
                "s2e2".to_string(),
                "s3e1".to_string(),
                "s1s1e1".to_string(),
                "s1s2e1".to_string(),
                "s2s1e1".to_string(),
                "s3s1e1".to_string(),
            ],
        );
    }

    #[test]
    fn test_transitive_imports() {}
}
