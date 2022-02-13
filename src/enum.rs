use crate::{lang::Lang, Name};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Enum<L: Lang> {
    pub name: Name<L>,
}
