use crate::{lang::Lang, Name};

#[derive(Debug)]
pub struct Method<L: Lang> {
    pub name: Name<L>,
    pub desc: prost_types::MethodDescriptorProto,
}
