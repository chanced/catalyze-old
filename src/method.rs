use crate::Name;

#[derive(Debug, Clone, PartialEq)]
pub struct Method<U> {
    pub name: Name<U>,
    pub desc: prost_types::MethodDescriptorProto,
}
