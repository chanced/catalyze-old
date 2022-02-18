use crate::Name;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extension<U> {
    name: Name<U>,
}
