use std::rc::Rc;

use crate::{
    descriptor::FieldDescriptor, proto::Syntax, FullyQualified, Message, Name, WeakMessage,
};

use super::FieldDetail;

#[derive(Debug)]
pub(crate) struct EmbedFieldDetail<'a, U> {
    pub detail: FieldDetail<'a, U>,
    pub embed: WeakMessage<'a, U>,
}
impl<'a, U> EmbedFieldDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.detail.fully_qualified_name()
    }
    pub fn is_repeated(&self) -> bool {
        self.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.detail.is_map()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.detail.message()
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.detail.util()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.detail.util.replace(util);
    }
    pub fn syntax(&self) -> Syntax {
        self.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.detail.descriptor()
    }
    pub fn embed(&self) -> Message<'a, U> {
        self.embed.clone().into()
    }
}

impl<'a, U> Clone for EmbedFieldDetail<'a, U> {
    fn clone(&self) -> Self {
        Self {
            detail: self.detail.clone(),
            embed: self.embed.clone(),
        }
    }
}

#[derive(Debug)]
pub struct EmbedField<'a, U>(Rc<EmbedFieldDetail<'a, U>>);

impl<'a, U> Clone for EmbedField<'a, U> {
    fn clone(&self) -> Self {
        EmbedField(self.0.clone())
    }
}

impl<'a, U> EmbedField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name.clone()
    }
}
impl<'a, U> FullyQualified for EmbedField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fqn.clone()
    }
}

// impl<'a, U> From<WeakEmbedField<'a, U>> for EmbedField<'a, U> {
//     fn from(weak: WeakEmbedField<'a, U>) -> Self {
//         weak.upgrade()
//     }
// }
// impl<'a, U> From<&WeakEmbedField<'a, U>> for EmbedField<'a, U> {
//     fn from(weak: &WeakEmbedField<'a, U>) -> Self {
//         weak.upgrade()
//     }
// }

// #[derive(Debug)]
// pub(crate) struct WeakEmbedField<'a, U>(Weak<EmbedFieldDetail<'a, U>>);
// impl<'a, U> Clone for WeakEmbedField<'a, U> {
//     fn clone(&self) -> Self {
//         Self(self.0.clone())
//     }
// }
// impl<'a, U> Upgrade for WeakEmbedField<'a, U> {
//     type Output = EmbedField<'a, U>;

//     fn upgrade(&self) -> Self::Output {
//         EmbedField(self.0.upgrade().expect("Failed to upgrade WeakEmbedField"))
//     }
// }
// impl<'a, U> From<EmbedField<'a, U>> for WeakEmbedField<'a, U> {
//     fn from(ef: EmbedField<'a, U>) -> Self {
//         ef.downgrade()
//     }
// }
// impl<'a, U> From<&EmbedField<'a, U>> for WeakEmbedField<'a, U> {
//     fn from(ef: &EmbedField<'a, U>) -> Self {
//         ef.downgrade()
//     }
// }
