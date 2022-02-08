use crate::{lang::Lang, Name};

#[derive(Debug, Clone)]
pub struct OneOf<L: Lang> {
    name: Name<L>,
    desc: prost_types::OneofDescriptorProto
}
