use crate::{util::Lang, Name};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValue<L> {
    pub name: Name<L>,
}
