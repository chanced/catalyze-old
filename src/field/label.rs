#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum Label {
    Required = 1,
    Optional = 2,
    Repeated = 3,
}
impl From<prost_types::field_descriptor_proto::Label> for Label {
    fn from(label: prost_types::field_descriptor_proto::Label) -> Self {
        match label {
            prost_types::field_descriptor_proto::Label::Optional => todo!(),
            prost_types::field_descriptor_proto::Label::Required => todo!(),
            prost_types::field_descriptor_proto::Label::Repeated => todo!(),
        }
    }
}
