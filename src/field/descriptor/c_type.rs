#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum CType {
    /// Default mode.
    String = 0,
    Cord = 1,
    StringPiece = 2,
}

impl From<prost_types::field_options::CType> for CType {
    fn from(c_type: prost_types::field_options::CType) -> Self {
        match c_type {
            prost_types::field_options::CType::String => CType::String,
            prost_types::field_options::CType::Cord => CType::Cord,
            prost_types::field_options::CType::StringPiece => CType::StringPiece,
        }
    }
}
