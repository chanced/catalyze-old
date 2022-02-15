use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{container::Container, Lang, Message};

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
            nodes.get(self.idx).cloned().map(|n| n.upgrade().unwrap())
        } else {
            None
        }
    }
}
pub struct Iter<T> {
    nodes: Rc<RefCell<Vec<Rc<T>>>>,
    idx: usize,
    size: usize,
}

impl<T> Iter<T> {
    pub(crate) fn new(nodes: Rc<RefCell<Vec<Rc<T>>>>) -> Self {
        let nodes = nodes.clone();
        let size = nodes.clone().borrow().len();
        Self {
            size,
            nodes,
            idx: 0,
        }
    }
}

impl<T> Iterator for Iter<T> {
    type Item = Rc<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let nodes = self.nodes.borrow();
        if self.idx < nodes.len() {
            self.idx += 1;
            nodes.get(self.idx).cloned()
        } else {
            None
        }
    }
}

pub struct MapIter<T> {
    nodes: Rc<RefCell<HashMap<String, Rc<T>>>>,
    idx: usize,
    keys: Vec<String>,
    size: usize,
}
impl<T> MapIter<T> {
    pub(crate) fn new(nodes: Rc<RefCell<HashMap<String, Rc<T>>>>) -> Self {
        let keys: Vec<String> = nodes.borrow().keys().cloned().collect();
        let size = keys.len();
        Self {
            nodes,
            size,
            keys,
            idx: 0,
        }
    }
}
impl<T> Iterator for MapIter<T> {
    type Item = (String, Rc<T>);
    fn next(&mut self) -> Option<Self::Item> {
        let nodes = self.nodes.borrow();
        if self.idx < self.keys.len() {
            let key = self.keys[self.idx].clone();
            self.idx += 1;
            nodes.get(&key).cloned().map(|v| (key, v))
        } else {
            None
        }
    }
}

pub struct AllMessages<L: Lang> {
    nodes: Rc<RefCell<Vec<Rc<Message<L>>>>>,
    cur_iter: Option<Box<AllMessages<L>>>,
    idx: usize,
}

impl<L: Lang> AllMessages<L> {
    pub(crate) fn new(nodes: Rc<RefCell<Vec<Rc<Message<L>>>>>) -> Self {
        Self {
            nodes,
            cur_iter: None,
            idx: 0,
        }
    }
}

impl<L: Lang> Iterator for AllMessages<L> {
    type Item = Rc<Message<L>>;
    fn next(&mut self) -> Option<Self::Item> {
        let msgs = self.nodes.borrow();
        loop {
            if self.cur_iter.is_none() && self.idx < msgs.len() {
                let res = Some(msgs[self.idx].clone());
                self.idx += 1;
                return res;
            } else if let Some(mut iter) = self.cur_iter.take() {
                match iter.next() {
                    Some(msg) => {
                        self.cur_iter = Some(iter);
                        return Some(msg);
                    }
                    None => {
                        self.idx += 1;
                        if self.idx < msgs.len() {
                            self.cur_iter = Some(Box::new(AllMessages::new(
                                self.nodes.borrow().get(self.idx).unwrap().messages.clone(),
                            )));
                        } else {
                            return None;
                        }
                    }
                }
            } else {
                if msgs.len() == 0 {
                    return None;
                }
                self.idx = 0;
                self.cur_iter = Some(Box::new(AllMessages::new(
                    self.nodes.borrow().get(0).unwrap().messages.clone(),
                )));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{container::InternalContainer, lang::Unspecified, File, Name};

    use super::*;
    #[test]
    fn test_all_messages() {
        let file = Rc::new(File {
            name: Name::new("test.rs", Unspecified),
            ..File::<Unspecified>::default()
        });

        let root = Rc::new(Message {
            name: Name::new("root", Unspecified),
            container: InternalContainer::from(file.clone()),
            ..Message::<Unspecified>::default()
        });
        file.add_message(root.clone());
        let s1 = Rc::new(Message {
            name: Name::new("s1", Unspecified),
            container: InternalContainer::from(file.clone()),
            ..Message::<Unspecified>::default()
        });
        root.add_message(s1.clone());
        let s2 = Rc::new(Message {
            name: Name::new("s2", Unspecified),
            container: InternalContainer::from(file.clone()),
            ..Message::<Unspecified>::default()
        });
        root.add_message(s2.clone());

        let sub3 = Rc::new(Message {
            name: Name::new("s3", Unspecified),
            container: InternalContainer::from(file.clone()),
            ..Message::<Unspecified>::default()
        });

        root.add_message(sub3);

        let s1s1 = Rc::new(Message {
            name: Name::new("s1s1", Unspecified),
            container: InternalContainer::from(s1.clone()),
            ..Message::<Unspecified>::default()
        });
        s1.add_message(s1s1);
        let s1s2 = Rc::new(Message {
            name: Name::new("s1s2", Unspecified),
            container: InternalContainer::from(file.clone()),
            ..Message::<Unspecified>::default()
        });
        s1.add_message(s1s2);

        let s1s3 = Rc::new(Message {
            name: Name::new("s1s3", Unspecified),
            container: InternalContainer::from(s1.clone()),
            ..Message::<Unspecified>::default()
        });

        s1.add_message(s1s3);
        let s2s1 = Rc::new(Message {
            name: Name::new("s1s2", Unspecified),
            container: InternalContainer::from(s2.clone()),
            ..Message::<Unspecified>::default()
        });
        s2.add_message(s2s1);
        let s2s2 = Rc::new(Message {
            name: Name::new("s2s2", Unspecified),
            container: InternalContainer::from(file.clone()),
            ..Message::<Unspecified>::default()
        });

        s2.add_message(s2s2);
        let m: Vec<String> = AllMessages::new(file.messages.clone())
            .map(|m| m.name.to_string())
            .collect();

        assert_eq!(
            m,
            vec![
                "root".to_string(),
                "s1".to_string(),
                "s2".to_string(),
                "s3".to_string(),
                "s1s1".to_string(),
                "s1s2".to_string(),
                "s1s3".to_string(),
                "s1s2".to_string(),
                "s2s2".to_string()
            ]
        );
    }
}
