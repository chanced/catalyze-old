use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
    str::FromStr,
};

use crate::{
    container::{Container, WeakContainer},
    iter::Iter,
    proto::{EnumDescriptor, EnumDescriptorPath},
    Comments, Dependents, EnumValue, File, FullyQualified, Message, Name, Node, Nodes, Package,
    WeakFile, WeakMessage, WellKnownEnum, WellKnownType,
};

#[derive(Debug, Clone)]
struct EnumDetail<'a, U> {
    name: Name<U>,
    fqn: String,
    comments: RefCell<Comments<'a>>,
    values: Rc<RefCell<Vec<EnumValue<'a, U>>>>,
    container: WeakContainer<'a, U>,
    dependents: Rc<RefCell<Vec<WeakMessage<'a, U>>>>,
    util: Rc<U>,
    descriptor: EnumDescriptor<'a>,
    wkt: Option<WellKnownEnum>,
}

impl<'a, U> EnumDetail<'a, U> {
    pub fn comments(&self) -> Comments<'a> {
        *self.comments.borrow()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.comments.replace(comments);
    }
    pub(crate) fn descriptor(&self) -> EnumDescriptor<'a> {
        self.descriptor
    }
    pub fn container(&self) -> Container<'a, U> {
        self.container.clone().into()
    }
    pub fn file(&self) -> File<'a, U> {
        self.container.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.container().package()
    }
    pub fn is_well_known_type(&self) -> bool {
        self.package().is_well_known_type()
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.wkt.map(Into::into)
    }
    pub fn well_known_enum(&self) -> Option<WellKnownEnum> {
        self.wkt
    }
}

#[derive(Debug)]
pub struct Enum<'a, U>(Rc<EnumDetail<'a, U>>);

impl<'a, U> Enum<'a, U> {
    pub(crate) fn new(desc: EnumDescriptor<'a>, container: Container<'a, U>) -> Self {
        let util = container.util();
        let fully_qualified_name = format!("{}.{}", container.fully_qualified_name(), desc.name());
        let wkt = if container.package().is_well_known_type() {
            WellKnownEnum::from_str(desc.name()).ok()
        } else {
            None
        };
        let e = Enum(Rc::new(EnumDetail {
            name: Name::new(desc.name(), util.clone()),
            values: Rc::new(RefCell::new(Vec::with_capacity(desc.values().len()))),
            container: container.into(),
            dependents: Rc::new(RefCell::new(Vec::default())),
            fqn: fully_qualified_name,
            util,
            descriptor: desc,
            comments: RefCell::new(Comments::default()),
            wkt,
        }));

        {
            let mut values = e.0.values.borrow_mut();
            for v in desc.values() {
                values.push(EnumValue::new(v, e.clone()));
            }
        }
        e
    }

    pub fn well_known_enum(&self) -> Option<WellKnownEnum> {
        self.0.well_known_enum()
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.0.well_known_type()
    }
    pub fn is_well_known_type(&self) -> bool {
        self.0.is_well_known_type()
    }
    pub fn descriptor(&self) -> EnumDescriptor<'a> {
        self.0.descriptor()
    }
    pub fn container(&self) -> Container<'a, U> {
        self.0.container.clone().into()
    }
    pub fn file(&self) -> File<'a, U> {
        self.0.file()
    }
    pub fn name(&self) -> Name<U> {
        self.0.name.clone()
    }
    pub fn util(&self) -> Rc<U> {
        self.0.util.clone()
    }

    pub fn values(&self) -> Iter<EnumValue<'a, U>> {
        Iter::from(&self.0.values)
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.package()
    }
    fn downgrade(&self) -> WeakEnum<'a, U> {
        WeakEnum(Rc::downgrade(&self.0))
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.comments()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.set_comments(comments);
    }

    pub(crate) fn weak_file(&self) -> WeakFile<'a, U> {
        self.0.container.weak_file()
    }

    pub fn nodes(&self) -> Nodes<'a, U> {
        Nodes::new(vec![self.values().into()])
    }
    pub fn dependents(&self) -> Dependents<'a, U> {
        self.0.dependents.clone().into()
    }
    pub(crate) fn add_dependent(&self, dep: Message<'a, U>) {
        self.0.dependents.borrow_mut().push(dep.into());
    }
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node<'a, U>> {
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
                EnumDescriptorPath::Value => self.0.values.borrow().get(next).map(Into::into),
                // _ => None,
            })
    }
}

impl<'a, U> Clone for Enum<'a, U> {
    fn clone(&self) -> Self {
        Enum(self.0.clone())
    }
}

impl<'a, U> FullyQualified for Enum<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.fqn.clone()
    }
}
impl<'a, U> From<WeakEnum<'a, U>> for Enum<'a, U> {
    fn from(e: WeakEnum<'a, U>) -> Self {
        e.upgrade()
    }
}
impl<'a, U> From<&WeakEnum<'a, U>> for Enum<'a, U> {
    fn from(e: &WeakEnum<'a, U>) -> Self {
        e.upgrade()
    }
}
#[derive(Debug)]
pub(crate) struct WeakEnum<'a, U>(Weak<EnumDetail<'a, U>>);
impl<'a, U> WeakEnum<'a, U> {
    pub(crate) fn empty() -> Self {
        WeakEnum(Weak::new())
    }
    fn upgrade(&self) -> Enum<'a, U> {
        Enum(self.0.upgrade().expect("Failed to upgrade WeakEnum"))
    }
    pub(crate) fn weak_file(&self) -> WeakFile<'a, U> {
        self.upgrade().weak_file()
    }
}
impl<'a, U> Clone for WeakEnum<'a, U> {
    fn clone(&self) -> Self {
        WeakEnum(self.0.clone())
    }
}
impl<'a, U> From<Enum<'a, U>> for WeakEnum<'a, U> {
    fn from(e: Enum<'a, U>) -> Self {
        e.downgrade()
    }
}
impl<'a, U> From<&Enum<'a, U>> for WeakEnum<'a, U> {
    fn from(e: &Enum<'a, U>) -> Self {
        e.downgrade()
    }
}

#[derive(Debug)]
pub struct AllEnums<'a, U> {
    msgs: VecDeque<Message<'a, U>>,
    enums: VecDeque<Enum<'a, U>>,
}
impl<'a, U> AllEnums<'a, U> {
    pub(crate) fn new(
        enums: Rc<RefCell<Vec<Enum<'a, U>>>>,
        msgs: Rc<RefCell<Vec<Message<'a, U>>>>,
    ) -> Self {
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
