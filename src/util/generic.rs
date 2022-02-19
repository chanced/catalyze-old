use crate::util::ToCase;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Generic;
impl Default for Generic {
    fn default() -> Self {
        Self {}
    }
}
impl ToCase for Generic {}
