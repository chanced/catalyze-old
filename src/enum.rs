use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
};

use prost_types::EnumDescriptorProto;

use crate::{
    container::{Container, WeakContainer},
    util::Generic,
    EnumValue, EnumValueList, Message, MessageList, Name, Node, Package,
};

pub(crate) type EnumList<'a, U> = Rc<RefCell<Vec<Rc<Enum<'a, U>>>>>;

#[derive(Debug, Clone)]
pub struct Enum<'a, U> {
    pub name: Name<U>,
    pub fully_qualified_name: String,
    pub(crate) values: EnumValueList<'a, U>,
    pub(crate) container: WeakContainer<'a, U>,
    pub(crate) dependents: Rc<RefCell<Vec<Weak<Message<'a, U>>>>>,
}

impl<'a, U> Enum<'a, U> {
    pub fn new(
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
            fully_qualified_name,
        });
        {
            let mut values = e.values.borrow_mut();
            for v in &desc.value {
                values.push(EnumValue::new(v, e.clone(), util.clone()));
            }
        }

        e
    }

    pub fn package(&self) -> Option<Rc<Package<'a, U>>> {
        self.container.package()
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
        todo!()
    }
}

impl Default for Enum<'_, Generic> {
    fn default() -> Self {
        Self {
            name: Name::default(),
            fully_qualified_name: String::default(),
            container: WeakContainer::File(Weak::new()),
            dependents: Rc::new(RefCell::new(Vec::default())),
            values: Rc::new(RefCell::new(Vec::default())),
        }
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

pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}
