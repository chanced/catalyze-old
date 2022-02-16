use crate::Name;

#[derive(Debug, Clone, PartialEq)]
pub struct Method<L> {
    pub name: Name<L>,
    pub desc: prost_types::MethodDescriptorProto,
}
