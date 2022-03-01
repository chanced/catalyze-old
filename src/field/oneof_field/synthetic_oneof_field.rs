use crate::{
    traits::{Downgrade, Upgrade},
    FullyQualified, Message, Name, Named, OneofEnumField, OneofMessageField, OneofScalarField,
    WeakOneofEnumField, WeakOneofMessageField, WeakOneofScalarField,
};

#[derive(Debug, Clone)]
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
    pub fn fully_qualified_name(&self) -> &str {
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

impl<'a, U> FullyQualified for SyntheticOneofField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        match self {
            SyntheticOneofField::Scalar(f) => f.fully_qualified_name(),
            SyntheticOneofField::Enum(f) => f.fully_qualified_name(),
            SyntheticOneofField::Message(f) => f.fully_qualified_name(),
        }
    }
}
impl<'a, U> Named<U> for SyntheticOneofField<'a, U> {
    fn name(&self) -> Name<U> {
        match self {
            SyntheticOneofField::Scalar(f) => f.name(),
            SyntheticOneofField::Enum(f) => f.name(),
            SyntheticOneofField::Message(f) => f.name(),
        }
    }
}

impl<'a, U> Downgrade for SyntheticOneofField<'a, U> {
    type Output = WeakSyntheticOneofField<'a, U>;

    fn downgrade(self) -> Self::Output {
        match self {
            SyntheticOneofField::Scalar(f) => WeakSyntheticOneofField::Scalar(f.downgrade()),
            SyntheticOneofField::Enum(f) => WeakSyntheticOneofField::Enum(f.downgrade()),
            SyntheticOneofField::Message(f) => WeakSyntheticOneofField::Message(f.downgrade()),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum WeakSyntheticOneofField<'a, U> {
    Enum(WeakOneofEnumField<'a, U>),
    Message(WeakOneofMessageField<'a, U>),
    Scalar(WeakOneofScalarField<'a, U>),
}
impl<'a, U> Upgrade for WeakSyntheticOneofField<'a, U> {
    type Output = SyntheticOneofField<'a, U>;
    fn upgrade(self) -> Self::Output {
        match self {
            WeakSyntheticOneofField::Enum(f) => SyntheticOneofField::Enum(f.upgrade()),
            WeakSyntheticOneofField::Message(f) => SyntheticOneofField::Message(f.upgrade()),
            WeakSyntheticOneofField::Scalar(f) => SyntheticOneofField::Scalar(f.upgrade()),
        }
    }
}
