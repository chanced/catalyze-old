use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
    str::FromStr,
};

use protobuf::reflect::EnumDescriptor;

use crate::{
    container::{Container, WeakContainer},
    iter::Iter,
    Comments, Dependents, EnumDescriptorPath, EnumValue, File, Message, Node, Nodes, Package,
    WeakFile, WeakMessage, WellKnownEnum, WellKnownType,
};

#[derive(Debug, Clone)]
struct EnumDetail {
    fqn: String,
    comments: RefCell<Comments>,
    values: RefCell<Vec<EnumValue>>,
    container: WeakContainer,
    dependents: RefCell<Vec<WeakMessage>>,
    descriptor: EnumDescriptor,
    wkt: Option<WellKnownEnum>,
}

impl EnumDetail {
    pub fn comments(&self) -> Comments {
        *self.comments.borrow()
    }
    pub(crate) fn set_comments(&self, comments: Comments) {
        self.comments.replace(comments);
    }
    pub(crate) fn descriptor(&self) -> EnumDescriptor {
        self.descriptor
    }
    pub fn container(&self) -> Container {
        self.container.clone().into()
    }
    pub fn file(&self) -> File {
        self.container.file()
    }
    pub fn package(&self) -> Package {
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

#[derive(Debug, Clone)]
pub struct Enum(Rc<EnumDetail>);

impl Enum {
    pub(crate) fn new(desc: EnumDescriptor, container: Container) -> Self {
        let fully_qualified_name = format!("{}.{}", container.fully_qualified_name(), desc.name());
        let wkt = if container.package().is_well_known_type() {
            WellKnownEnum::from_str(desc.name()).ok()
        } else {
            None
        };
        let e = Enum(Rc::new(EnumDetail {
            values: Rc::new(RefCell::new(Vec::with_capacity(desc.values().len()))),
            container: container.into(),
            dependents: Rc::new(RefCell::new(Vec::default())),
            fqn: fully_qualified_name,
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
    pub fn has_value(&self, name: &str) -> bool {
        self.values().any(|v| v.name() == name)
    }
    pub fn value(&self, name: &str) -> Option<EnumValue> {
        self.values().find(|v| v.name() == name)
    }
    pub fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
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
    pub fn descriptor(&self) -> EnumDescriptor {
        self.0.descriptor()
    }
    pub fn container(&self) -> Container {
        self.0.container.clone().into()
    }
    pub fn file(&self) -> File {
        self.0.file()
    }
    pub fn name(&self) -> &str {
        &self.0.descriptor.name()
    }
    pub fn values(&self) -> Iter<EnumValue> {
        Iter::from(&self.0.values)
    }
    pub fn package(&self) -> Package {
        self.0.package()
    }
    fn downgrade(&self) -> WeakEnum {
        WeakEnum(Rc::downgrade(&self.0))
    }
    pub fn comments(&self) -> Comments {
        self.0.comments()
    }

    pub(crate) fn set_comments(&self, comments: Comments) {
        self.0.set_comments(comments);
    }

    pub(crate) fn weak_file(&self) -> WeakFile {
        self.0.container.weak_file()
    }

    pub fn nodes(&self) -> Nodes {
        Nodes::new(vec![self.values().into()])
    }
    pub fn dependents(&self) -> Dependents {
        self.0.dependents.clone().into()
    }
    pub(crate) fn add_dependent(&self, dep: Message) {
        self.0.dependents.borrow_mut().push(dep.into());
    }
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node> {
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
impl PartialEq for Enum {
    fn eq(&self, other: &Self) -> bool {
        self.fully_qualified_name() == other.fully_qualified_name()
    }
}
impl From<WeakEnum> for Enum {
    fn from(e: WeakEnum) -> Self {
        e.upgrade()
    }
}
impl From<&WeakEnum> for Enum {
    fn from(e: &WeakEnum) -> Self {
        e.upgrade()
    }
}
#[derive(Debug, Clone)]
pub(crate) struct WeakEnum(Weak<EnumDetail>);
impl WeakEnum {
    pub(crate) fn empty() -> Self {
        WeakEnum(Weak::new())
    }
    fn upgrade(&self) -> Enum {
        Enum(self.0.upgrade().expect("Failed to upgrade WeakEnum"))
    }
    pub(crate) fn weak_file(&self) -> WeakFile {
        self.upgrade().weak_file()
    }
}
impl From<Enum> for WeakEnum {
    fn from(e: Enum) -> Self {
        e.downgrade()
    }
}
impl From<&Enum> for WeakEnum {
    fn from(e: &Enum) -> Self {
        e.downgrade()
    }
}

#[derive(Debug, Clone)]
pub struct AllEnums {
    msgs: VecDeque<Message>,
    enums: VecDeque<Enum>,
}
impl AllEnums {
    pub(crate) fn new(enums: Rc<RefCell<Vec<Enum>>>, msgs: Rc<RefCell<Vec<Message>>>) -> Self {
        Self {
            msgs: msgs.borrow().iter().cloned().collect(),
            enums: enums.borrow().iter().cloned().collect(),
        }
    }
}

impl Iterator for AllEnums {
    type Item = Enum;
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
