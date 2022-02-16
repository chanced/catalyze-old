use crate::{util::Lang, Name};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValue<L> {
    name: Name<L>,
}
