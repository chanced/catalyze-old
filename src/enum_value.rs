use crate::{lang::Lang, Name};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct EnumValue<L: Lang> {
    name: Name<L>,
}
