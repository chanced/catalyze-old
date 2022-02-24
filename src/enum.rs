use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
};

use prost_types::EnumDescriptorProto;

use crate::{
    container::{Container, WeakContainer},
    iter::Iter,
    path::EnumDescriptorPath,
    EnumValue, EnumValueList, FullyQualified, Message, MessageList, Name, Node, NodeAtPath,
    Package,
};

pub(crate) type EnumList<'a, U> = Rc<RefCell<Vec<Rc<Enum<'a, U>>>>>;

#[derive(Debug, Clone)]
pub struct Enum<'a, U> {
    pub name: Name<U>,
    fqn: String,
    values: EnumValueList<'a, U>,
    container: WeakContainer<'a, U>,
    dependents: Rc<RefCell<Vec<Weak<Message<'a, U>>>>>,
}

impl<'a, U> Enum<'a, U> {
    pub(crate) fn new(
        desc: &'a EnumDescriptorProto,
        container: Container<'a, U>,
        util: Rc<RefCell<U>>,
    ) -> Rc<Self> {
        let fully_qualified_name = format!("{}.{}", container.fully_qualified_name(), desc.name());

        let e = Rc::new(Enum {
            name: Name::new(desc.name(), util.clone()),
            values: Rc::new(RefCell::new(Vec::with_capacity(desc.value.len()))),
            container: container.downgrade(),
            dependents: Rc::new(RefCell::new(Vec::default())),
            fqn: fully_qualified_name,
        });
        {
            let mut values = e.values.borrow_mut();
            for v in &desc.value {
                values.push(EnumValue::new(v, e.clone(), util.clone()));
            }
        }

        e
    }
    pub fn name(&self) -> Name<U> {
        self.name.clone()
    }

    pub fn values(&self) -> Iter<EnumValue<'a, U>> {
        Iter::from(&self.values)
    }
    pub fn package(&self) -> Option<Rc<Package<'a, U>>> {
        self.container.package()
    }
}

impl<'a, U> NodeAtPath<'a, U> for Rc<Enum<'a, U>> {
    fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        if path.is_empty() {
            return Some(Node::Enum(self.clone()));
        }
        if path.len() != 2 {
            return None;
        }
        let next = path[1] as usize;
        EnumDescriptorPath::try_from(path[0])
            .ok()
            .and_then(|p| match p {
                EnumDescriptorPath::Value => {
                    self.values.borrow().get(next).cloned().map(Node::EnumValue)
                }
                _ => None,
            })
    }
}

pub struct AllEnums<'a, U> {
    msgs: VecDeque<Rc<Message<'a, U>>>,
    enums: VecDeque<Rc<Enum<'a, U>>>,
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
    type Item = Rc<Enum<'a, U>>;
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

impl<'a, U> FullyQualified for Enum<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.fqn.clone()
    }
}
