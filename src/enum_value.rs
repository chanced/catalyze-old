use crate::{util::Lang, Name};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValue<U> {
    pub name: Name<U>,
}
