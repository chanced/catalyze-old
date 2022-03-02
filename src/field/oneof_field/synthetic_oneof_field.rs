use crate::{FullyQualified, Message, Name, OneofEnumField, OneofMessageField, OneofScalarField};

#[derive(Debug)]
pub enum SyntheticOneofField<'a, U> {
    Scalar(OneofScalarField<'a, U>),
    Enum(OneofEnumField<'a, U>),
    Message(OneofMessageField<'a, U>),
}

impl<'a, U> SyntheticOneofField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            SyntheticOneofField::Scalar(f) => f.name(),
            SyntheticOneofField::Enum(f) => f.name(),
            SyntheticOneofField::Message(f) => f.name(),
        }
    }
    pub fn fully_qualified_name(&self) -> String {
        match self {
            SyntheticOneofField::Scalar(f) => f.fully_qualified_name(),
            SyntheticOneofField::Enum(f) => f.fully_qualified_name(),
            SyntheticOneofField::Message(f) => f.fully_qualified_name(),
        }
    }
    pub fn message(&self) -> Message<'a, U> {
        match self {
            SyntheticOneofField::Scalar(f) => f.message(),
            SyntheticOneofField::Enum(f) => f.message(),
            SyntheticOneofField::Message(f) => f.message(),
        }
    }
}
impl<'a, U> Clone for SyntheticOneofField<'a, U> {
    fn clone(&self) -> Self {
        match self {
            Self::Scalar(f) => Self::Scalar(f.clone()),
            Self::Enum(f) => Self::Enum(f.clone()),
            Self::Message(f) => Self::Message(f.clone()),
        }
    }
}
impl<'a, U> FullyQualified for SyntheticOneofField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            SyntheticOneofField::Scalar(f) => f.fully_qualified_name(),
            SyntheticOneofField::Enum(f) => f.fully_qualified_name(),
            SyntheticOneofField::Message(f) => f.fully_qualified_name(),
        }
    }
}
