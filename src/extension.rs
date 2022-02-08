use crate::{lang::Lang, Name};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extension<L: Lang> {
    name: Name<L>,
}
