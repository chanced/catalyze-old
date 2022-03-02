use std::rc::Rc;
mod repeated_embed_field;
mod repeated_enum_field;
mod repeated_scalar_field;
pub use repeated_embed_field::*;
pub use repeated_enum_field::*;
pub use repeated_scalar_field::*;

use crate::{proto::Syntax, FullyQualified, Message, Name};

use super::descriptor::FieldDescriptor;

/// Represents a field marked as `repeated`. The field can hold
/// a scalar value, an enum, or a message.
#[derive(Debug)]
pub enum RepeatedField<'a, U> {
    Scalar(RepeatedScalarField<'a, U>),
    Enum(RepeatedEnumField<'a, U>),
    Message(RepeatedEmbedField<'a, U>),
}

impl<'a, U> RepeatedField<'a, U> {
    pub fn name(&self) -> Name<U> {
        match self {
            RepeatedField::Scalar(f) => f.name(),
            RepeatedField::Enum(f) => f.name(),
            RepeatedField::Message(f) => f.name(),
        }
    }
    fn fully_qualified_name(&self) -> String {
        match self {
            RepeatedField::Scalar(f) => f.fully_qualified_name(),
            RepeatedField::Enum(f) => f.fully_qualified_name(),
            RepeatedField::Message(f) => f.fully_qualified_name(),
        }
    }
    /// Returns the Message containing this RepeatedField
    pub fn message(&self) -> Message<'a, U> {
        match self {
            RepeatedField::Scalar(f) => f.message(),
            RepeatedField::Enum(f) => f.message(),
            RepeatedField::Message(f) => f.message(),
        }
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        match self {
            RepeatedField::Scalar(f) => f.util(),
            RepeatedField::Enum(f) => f.util(),
            RepeatedField::Message(f) => f.util(),
        }
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        match self {
            RepeatedField::Scalar(s) => s.replace_util(util),
            RepeatedField::Enum(e) => e.replace_util(util),
            RepeatedField::Message(m) => m.replace_util(util),
        }
    }
    pub fn syntax(&self) -> Syntax {
        match self {
            RepeatedField::Scalar(f) => f.syntax(),
            RepeatedField::Enum(f) => f.syntax(),
            RepeatedField::Message(f) => f.syntax(),
        }
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        match self {
            RepeatedField::Scalar(f) => f.descriptor(),
            RepeatedField::Enum(f) => f.descriptor(),
            RepeatedField::Message(f) => f.descriptor(),
        }
    }

    pub fn is_map(&self) -> bool {
        false
    }

    pub fn is_repeated(&self) -> bool {
        true
    }

    pub fn is_scalar(&self) -> bool {
        match self {
            RepeatedField::Scalar(_) => true,
            _ => false,
        }
    }

    pub fn is_marked_optional(&self) -> bool {
        false
    }

    pub fn has_presence(&self) -> bool {
        if let RepeatedField::Message(_) = self {
            true
        } else {
            false
        }
    }
}
impl<'a, U> Clone for RepeatedField<'a, U> {
    fn clone(&self) -> Self {
        match self {
            RepeatedField::Scalar(f) => RepeatedField::Scalar(f.clone()),
            RepeatedField::Enum(f) => RepeatedField::Enum(f.clone()),
            RepeatedField::Message(f) => RepeatedField::Message(f.clone()),
        }
    }
}
impl<'a, U> FullyQualified for RepeatedField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        match self {
            RepeatedField::Scalar(f) => f.fully_qualified_name(),
            RepeatedField::Enum(f) => f.fully_qualified_name(),
            RepeatedField::Message(f) => f.fully_qualified_name(),
        }
    }
}

impl<'a, U> From<RepeatedScalarField<'a, U>> for RepeatedField<'a, U> {
    fn from(f: RepeatedScalarField<'a, U>) -> Self {
        RepeatedField::Scalar(f)
    }
}

impl<'a, U> From<&RepeatedScalarField<'a, U>> for RepeatedField<'a, U> {
    fn from(f: &RepeatedScalarField<'a, U>) -> Self {
        RepeatedField::Scalar(f.clone())
    }
}

impl<'a, U> From<RepeatedEnumField<'a, U>> for RepeatedField<'a, U> {
    fn from(f: RepeatedEnumField<'a, U>) -> Self {
        RepeatedField::Enum(f)
    }
}

impl<'a, U> From<&RepeatedEnumField<'a, U>> for RepeatedField<'a, U> {
    fn from(f: &RepeatedEnumField<'a, U>) -> Self {
        RepeatedField::Enum(f.clone())
    }
}

impl<'a, U> From<RepeatedEmbedField<'a, U>> for RepeatedField<'a, U> {
    fn from(f: RepeatedEmbedField<'a, U>) -> Self {
        RepeatedField::Message(f)
    }
}

impl<'a, U> From<&RepeatedEmbedField<'a, U>> for RepeatedField<'a, U> {
    fn from(f: &RepeatedEmbedField<'a, U>) -> Self {
        RepeatedField::Message(f.clone())
    }
}

// #[derive(Debug, Clone)]
// pub(crate) enum WeakRepeatedField<'a, U> {
//     Scalar(WeakRepeatedScalarField<'a, U>),
//     Enum(WeakRepeatedEnumField<'a, U>),
//     Embed(WeakRepeatedEmbedField<'a, U>),
// }
// impl<'a, U> From<RepeatedField<'a, U>> for WeakRepeatedField<'a, U> {
//     fn from(f: RepeatedField<'a, U>) -> Self {
//         match f {
//             RepeatedField::Scalar(f) => WeakRepeatedField::Scalar(f.downgrade()),
//             RepeatedField::Enum(f) => WeakRepeatedField::Enum(f.downgrade()),
//             RepeatedField::Message(f) => WeakRepeatedField::Embed(f.downgrade()),
//         }
//     }
// }
