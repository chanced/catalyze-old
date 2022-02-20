use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
};

use prost_types::EnumDescriptorProto;

use crate::{
    container::{Container, InternalContainer},
    util::Generic,
    EnumValueList, Message, MessageList, Name, Node, Package,
};

pub(crate) type EnumList<U> = Rc<RefCell<Vec<Rc<Enum<U>>>>>;

#[derive(Debug, Clone)]
pub struct Enum<U> {
    pub name: Name<U>,
    pub fully_qualified_name: String,
    pub(crate) values: EnumValueList<U>,
    pub(crate) container: InternalContainer<U>,
    pub(crate) dependents: Rc<RefCell<Vec<Weak<Message<U>>>>>,
}

impl<U> Enum<U> {
    pub fn new(
        desc: EnumDescriptorProto,
        container: Container<U>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let fully_qualified_name = format!("{}.{}", container.fully_qualified_name(), desc.name());

        let e = Rc::new(Enum {
            name: Name::new(desc.name(), util.clone()),
            values: Rc::new(RefCell::new(Vec::with_capacity(desc.value.len()))),
            container: container.downgrade(),
            dependents: Rc::new(RefCell::new(Vec::default())),
            fully_qualified_name,
        });

        e
    }

    pub fn package(&self) -> Option<Rc<Package<U>>> {
        self.container.package()
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<U>> {
        todo!()
    }
}

impl Default for Enum<Generic> {
    fn default() -> Self {
        Self {
            name: Name::default(),
            fully_qualified_name: String::default(),
            container: InternalContainer::File(Weak::new()),
            dependents: Rc::new(RefCell::new(Vec::default())),
            values: Rc::new(RefCell::new(Vec::default())),
        }
    }
}

pub struct AllEnums<U> {
    msgs: VecDeque<Rc<Message<U>>>,
    enums: VecDeque<Rc<Enum<U>>>,
}

impl<U> AllEnums<U> {
    pub(crate) fn new(enums: EnumList<U>, msgs: MessageList<U>) -> Self {
        Self {
            msgs: msgs.borrow().iter().cloned().collect(),
            enums: enums.borrow().iter().cloned().collect(),
        }
    }
}

impl<U> Iterator for AllEnums<U> {
    type Item = Rc<Enum<U>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.enums.pop_front() {
            Some(e)
        } else {
            while let Some(msg) = self.msgs.pop_front() {
                for v in msg.messages.borrow().iter().cloned() {
                    self.msgs.push_back(v);
                }
                for v in msg.enums.borrow().iter().cloned() {
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
