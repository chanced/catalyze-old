use crate::language::Language;
use crate::{Name, Visitor};
use std::rc::{Rc, Weak};

trait Node<'a, L: Language> {
    fn name(&self) -> Name<L>;
    fn visit<V: Visitor<L, Error = E>, E>(&self, visitor: V) -> Result<(), E>;
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Enum {}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct EnumValue {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extension {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OneOf {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Service {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Method {}
