use crate::language::Language;
use crate::Name;
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Message<L: Language> {
    name: Name<L>
}
