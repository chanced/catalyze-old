use crate::{FullyQualified, Message, Name, OneofEnumField, OneofMessageField, OneofScalarField};

#[derive(Debug)]
pub enum RealOneofField<'a, U> {
    Scalar(OneofScalarField<'a, U>),
    Enum(OneofEnumField<'a, U>),
    Message(OneofMessageField<'a, U>),
}
impl<'a, U> Clone for RealOneofField<'a, U> {
    fn clone(&self) -> Self {
        match self {
            Self::Scalar(f) => Self::Scalar(f.clone()),
            Self::Enum(f) => Self::Enum(f.clone()),
            Self::Message(f) => Self::Message(f.clone()),
        }
    }
}
impl<'a, U> RealOneofField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            RealOneofField::Scalar(f) => f.name(),
            RealOneofField::Enum(f) => f.name(),
            RealOneofField::Message(f) => f.name(),
        }
    }
    pub fn fully_qualified_name(&self) -> String {
        match self {
            RealOneofField::Scalar(f) => f.fully_qualified_name(),
            RealOneofField::Enum(f) => f.fully_qualified_name(),
            RealOneofField::Message(f) => f.fully_qualified_name(),
        }
    }
    pub fn message(&self) -> Message<'a, U> {
        match self {
            RealOneofField::Scalar(f) => f.message(),
            RealOneofField::Enum(f) => f.message(),
            RealOneofField::Message(f) => f.message(),
        }
    }
}

impl<'a, U> FullyQualified for RealOneofField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            RealOneofField::Scalar(f) => f.fully_qualified_name(),
            RealOneofField::Enum(f) => f.fully_qualified_name(),
            RealOneofField::Message(f) => f.fully_qualified_name(),
        }
    }
}
