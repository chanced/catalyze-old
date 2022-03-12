use std::{cell::RefCell, rc::Rc};

use anyhow::bail;

use crate::{
    proto::FieldDescriptor, proto::Syntax, Comments, Field, File, Files, FullyQualified, Message,
    Name, Node, Package, Type, WeakMessage, WellKnownMessage, WellKnownType,
};

use super::FieldDetail;

#[derive(Debug)]
pub(crate) struct EmbedFieldDetail<'a, U> {
    pub detail: FieldDetail<'a, U>,
    pub embed: RefCell<WeakMessage<'a, U>>,
}
impl<'a, U> EmbedFieldDetail<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.detail.name()
    }
    pub fn embed(&self) -> Message<'a, U> {
        self.embed.borrow().clone().into()
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

    pub fn syntax(&self) -> Syntax {
        self.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.detail.descriptor()
    }

    pub fn imports(&self) -> Files<'a, U> {
        if self.embed.borrow().file() != self.detail.file() {
            Files::from(self.embed.borrow().weak_file())
        } else {
            Files::empty()
        }
    }
    pub fn build_target(&self) -> bool {
        self.detail.build_target()
    }
    pub fn is_well_known_type(&self) -> bool {
        self.embed.borrow().is_well_known_type()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.embed().well_known_type()
    }
    pub fn well_known_message(&self) -> Option<WellKnownMessage> {
        self.embed().well_known_message()
    }
    pub fn has_import(&self) -> bool {
        self.embed.borrow().file() != self.detail.file()
    }
    pub(crate) fn set_value(&self, value: Node<'a, U>) -> Result<(), anyhow::Error> {
        match value {
            Node::Message(m) => {
                self.embed.replace(m.into());
                Ok(())
            }
            _ => bail!("expected Message, received {}", value),
        }
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

impl<'a, U> EmbedField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name.clone()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn embed(&self) -> Message<'a, U> {
        self.0.embed()
    }
    pub fn build_target(&self) -> bool {
        self.0.build_target()
    }
    pub fn file(&self) -> File<'a, U> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a, U> {
        self.0.detail.package()
    }

    /// Indicates whether or not the field is labeled as a required field. This
    /// will only be `true` if the syntax is proto2.
    pub fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }
    pub fn is_marked_optional(&self) -> bool {
        self.0.detail.is_marked_optional()
    }

    pub fn has_presence(&self) -> bool {
        true
    }

    pub fn has_import(&self) -> bool {
        self.0.has_import()
    }

    pub fn imports(&self) -> Files<'a, U> {
        self.0.imports()
    }

    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.descriptor()
    }

    pub fn syntax(&self) -> Syntax {
        self.0.syntax()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.0.is_well_known_type()
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.0.well_known_type()
    }

    pub fn util(&self) -> Rc<U> {
        self.0.util()
    }

    pub(crate) fn set_value(&self, value: Node<'a, U>) -> Result<(), anyhow::Error> {
        self.0.set_value(value)
    }

    pub fn value_type(&self) -> Type<'a> {
        self.0.descriptor().proto_type()
    }

    pub(crate) fn new(detail: FieldDetail<'a, U>) -> Result<Field<'a, U>, anyhow::Error> {
        let detail = Rc::new(EmbedFieldDetail {
            detail,
            embed: RefCell::new(WeakMessage::empty()),
        });
        Ok(Field::Embed(EmbedField(detail)))
    }
}
impl<'a, U> Clone for EmbedField<'a, U> {
    fn clone(&self) -> Self {
        EmbedField(self.0.clone())
    }
}

impl<'a, U> FullyQualified for EmbedField<'a, U> {
    fn fully_qualified_name(&self) -> String {
        self.0.detail.fqn.clone()
    }
}
