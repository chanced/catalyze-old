use std::rc::{Rc, Weak};

use crate::{
    descriptor::FieldDescriptor, proto::Syntax, FullyQualified, Message, Name, ScalarFieldDetail,
};

#[derive(Debug, Clone)]
pub(crate) struct RepeatedScalarFieldDetail<'a, U> {
    detail: ScalarFieldDetail<'a, U>,
}

#[derive(Debug)]
pub struct RepeatedScalarField<'a, U>(Rc<RepeatedScalarFieldDetail<'a, U>>);

impl<'a, U> RepeatedScalarField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn is_repeated(&self) -> bool {
        self.0.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.0.detail.is_map()
    }
    pub fn message(&self) -> Message<'a, U> {
        self.0.detail.message()
    }
    /// Returns `Rc<U>`
    pub fn util(&self) -> Rc<U> {
        self.0.detail.util()
    }
    pub(crate) fn replace_util(&self, util: Rc<U>) {
        self.0.detail.replace_util(util);
    }
    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a, U> {
        self.0.detail.descriptor()
    }
}

impl<'a, U> FullyQualified for RepeatedScalarField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
}

impl<'a, U> Clone for RepeatedScalarField<'a, U> {
    fn clone(&self) -> Self {
        RepeatedScalarField(self.0.clone())
    }
}

// impl<'a, U> Downgrade for RepeatedScalarField<'a, U> {
//     type Output = WeakRepeatedScalarField<'a, U>;
//     fn downgrade(&self) -> Self::Output {
//         WeakRepeatedScalarField(Rc::downgrade(&self.0))
//     }
// }

// #[derive(Debug)]
// pub(crate) struct WeakRepeatedScalarField<'a, U>(Weak<RepeatedScalarFieldDetail<'a, U>>);
// impl<'a, U> Clone for WeakRepeatedScalarField<'a, U> {
//     fn clone(&self) -> Self {
//         WeakRepeatedScalarField(self.0.clone())
//     }
// }

// impl<'a, U> Upgrade for WeakRepeatedScalarField<'a, U> {
//     type Output = RepeatedScalarField<'a, U>;

//     fn upgrade(&self) -> Self::Output {
//         RepeatedScalarField(
//             self.0
//                 .upgrade()
//                 .expect("Failed to upgrade WeakRepeatedScalarField"),
//         )
//     }
// }
