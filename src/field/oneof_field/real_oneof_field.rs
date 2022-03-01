use crate::{
    traits::{Downgrade, Upgrade},
    FullyQualified, Message, Name, Named, OneofEnumField, OneofMessageField, OneofScalarField,
    RealOneof, WeakOneofEnumField, WeakOneofMessageField, WeakOneofScalarField,
};

#[derive(Debug, Clone)]
pub enum RealOneofField<'a, U> {
    Scalar(OneofScalarField<'a, U>),
    Enum(OneofEnumField<'a, U>),
    Message(OneofMessageField<'a, U>),
}

impl<'a, U> RealOneofField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            RealOneofField::Scalar(f) => f.name(),
            RealOneofField::Enum(f) => f.name(),
            RealOneofField::Message(f) => f.name(),
        }
    }
    pub fn fully_qualified_name(&self) -> &str {
        match self {
            RealOneofField::Scalar(f) => f.fully_qualified_name(),
            RealOneofField::Enum(f) => f.fully_qualified_name(),
            RealOneofField::Message(f) => f.fully_qualified_name(),
        }
    }
    pub fn container(&self) -> Message<'a, U> {
        match self {
            RealOneofField::Scalar(f) => f.container(),
            RealOneofField::Enum(f) => f.container(),
            RealOneofField::Message(f) => f.container(),
        }
    }
    pub fn containing_message(&self) -> Message<'a, U> {
        self.container()
    }
}
impl<'a, U> Downgrade for RealOneofField<'a, U> {
    type Output = WeakRealOneofField<'a, U>;
    fn downgrade(self) -> Self::Output {
        match self {
            RealOneofField::Scalar(f) => WeakRealOneofField::Scalar(f.downgrade()),
            RealOneofField::Enum(f) => WeakRealOneofField::Enum(f.downgrade()),
            RealOneofField::Message(f) => WeakRealOneofField::Message(f.downgrade()),
        }
    }
}

impl<'a, U> FullyQualified for RealOneofField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        match self {
            RealOneofField::Scalar(f) => f.fully_qualified_name(),
            RealOneofField::Enum(f) => f.fully_qualified_name(),
            RealOneofField::Message(f) => f.fully_qualified_name(),
        }
    }
}
impl<'a, U> Named<U> for RealOneofField<'a, U> {
    fn name(&self) -> Name<U> {
        match self {
            RealOneofField::Scalar(f) => f.name(),
            RealOneofField::Enum(f) => f.name(),
            RealOneofField::Message(f) => f.name(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WeakRealOneofField<'a, U> {
    Scalar(WeakOneofScalarField<'a, U>),
    Enum(WeakOneofEnumField<'a, U>),
    Message(WeakOneofMessageField<'a, U>),
}

impl<'a, U> Upgrade for WeakRealOneofField<'a, U> {
    type Output = RealOneofField<'a, U>;
    fn upgrade(self) -> Self::Output {
        match self {
            WeakRealOneofField::Scalar(f) => RealOneofField::Scalar(f.upgrade()),
            WeakRealOneofField::Enum(f) => RealOneofField::Enum(f.upgrade()),
            WeakRealOneofField::Message(f) => RealOneofField::Message(f.upgrade()),
        }
    }
}
