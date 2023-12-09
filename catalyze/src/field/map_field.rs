#![allow(clippy::new_ret_no_self)]
use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, bail};

use super::FieldDetail;
use crate::{
    proto::{FieldDescriptor, Type},
    proto::{Scalar, Syntax},
    Comments, Enum, Field, File, FileRefs, JsType, Message, Name, Node, Package,
    UninterpretedOptions, WeakEnum, WeakMessage, WellKnownEnum, WellKnownMessage, WellKnownType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Int64 = 3,
    Uint64 = 4,
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    String = 9,
    Uint32 = 13,
    Sfixed32 = 15,
    Sfixed64 = 16,
    Sint32 = 17,
    Sint64 = 18,
}
impl Key {
    pub fn is_int64(self) -> bool {
        self == Key::Int64
    }
    pub fn is_uint64(self) -> bool {
        self == Key::Uint64
    }
    pub fn is_int32(self) -> bool {
        self == Key::Int32
    }
    pub fn is_fixed64(self) -> bool {
        self == Key::Fixed64
    }
    pub fn is_fixed32(self) -> bool {
        self == Key::Fixed32
    }
    pub fn is_string(self) -> bool {
        self == Key::String
    }
    pub fn is_uint32(self) -> bool {
        self == Key::Uint32
    }
    pub fn is_sfixed32(self) -> bool {
        self == Key::Sfixed32
    }
    pub fn is_sfixed64(self) -> bool {
        self == Key::Sfixed64
    }
    pub fn is_sint32(self) -> bool {
        self == Key::Sint32
    }
    pub fn is_sint64(self) -> bool {
        self == Key::Sint64
    }
}

impl<'a> TryFrom<Type<'a>> for Key {
    type Error = anyhow::Error;

    fn try_from(t: Type<'a>) -> Result<Self, Self::Error> {
        match t {
            Type::Scalar(s) => match s {
                Scalar::Int64 => Ok(Key::Int64),
                Scalar::Uint64 => Ok(Key::Uint64),
                Scalar::Int32 => Ok(Key::Int32),
                Scalar::Fixed64 => Ok(Key::Fixed64),
                Scalar::Fixed32 => Ok(Key::Fixed32),
                Scalar::String => Ok(Key::String),
                Scalar::Uint32 => Ok(Key::Uint32),
                Scalar::Sfixed32 => Ok(Key::Sfixed32),
                Scalar::Sfixed64 => Ok(Key::Sfixed64),
                Scalar::Sint32 => Ok(Key::Sint32),
                Scalar::Sint64 => Ok(Key::Sint64),
                _ => bail!("invalid map key type: {}", s),
            },
            _ => bail!("invalid map key type: {:?}", t),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MapField<'a> {
    Scalar(MappedScalarField<'a>),
    Enum(MappedEnumField<'a>),
    Embed(MappedEmbedField<'a>),
}

impl<'a> MapField<'a> {
    pub fn name(&self) -> &Name {
        match self {
            MapField::Scalar(f) => f.name(),
            MapField::Enum(f) => f.name(),
            MapField::Embed(f) => f.name(),
        }
    }

    pub fn key(&self) -> Key {
        match self {
            MapField::Scalar(f) => f.key(),
            MapField::Enum(f) => f.key(),
            MapField::Embed(f) => f.key(),
        }
    }

    pub fn fully_qualified_name(&self) -> String {
        match self {
            MapField::Scalar(f) => f.fully_qualified_name(),
            MapField::Enum(f) => f.fully_qualified_name(),
            MapField::Embed(f) => f.fully_qualified_name(),
        }
    }
    pub fn embed(&self) -> Option<Message<'a>> {
        match self {
            MapField::Embed(m) => m.embed().into(),
            _ => None,
        }
    }
    /// alias for `enum_`
    pub fn enumeration(&self) -> Option<Enum<'a>> {
        self.enum_()
    }

    pub fn enum_(&self) -> Option<Enum<'a>> {
        match self {
            MapField::Enum(e) => e.enum_().into(),
            _ => None,
        }
    }
    pub fn comments(&self) -> Comments<'a> {
        match self {
            MapField::Scalar(f) => f.comments(),
            MapField::Enum(f) => f.comments(),
            MapField::Embed(f) => f.comments(),
        }
    }

    pub fn file(&self) -> File<'a> {
        match self {
            MapField::Scalar(f) => f.file(),
            MapField::Enum(f) => f.file(),
            MapField::Embed(f) => f.file(),
        }
    }

    pub fn build_target(&self) -> bool {
        match self {
            MapField::Scalar(f) => f.build_target(),
            MapField::Enum(f) => f.build_target(),
            MapField::Embed(f) => f.build_target(),
        }
    }

    pub fn package(&self) -> Package<'a> {
        match self {
            MapField::Scalar(f) => f.package(),
            MapField::Enum(f) => f.package(),
            MapField::Embed(f) => f.package(),
        }
    }
    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        match self {
            MapField::Scalar(f) => f.set_comments(comments),
            MapField::Enum(f) => f.set_comments(comments),
            MapField::Embed(f) => f.set_comments(comments),
        }
    }

    pub fn has_import(&self) -> bool {
        match self {
            MapField::Scalar(_) => false,
            MapField::Enum(f) => f.has_import(),
            MapField::Embed(f) => f.has_import(),
        }
    }
    pub fn imports(&self) -> FileRefs<'a> {
        match self {
            MapField::Scalar(_) => FileRefs::empty(),
            MapField::Enum(f) => f.imports(),
            MapField::Embed(f) => f.imports(),
        }
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        match self {
            MapField::Scalar(f) => f.descriptor(),
            MapField::Enum(f) => f.descriptor(),
            MapField::Embed(f) => f.descriptor(),
        }
    }

    pub fn syntax(&self) -> Syntax {
        match self {
            MapField::Scalar(f) => f.syntax(),
            MapField::Enum(f) => f.syntax(),
            MapField::Embed(f) => f.syntax(),
        }
    }

    pub fn scalar(&self) -> Option<Scalar> {
        match self {
            MapField::Scalar(f) => Some(f.scalar()),
            _ => None,
        }
    }

    pub fn is_marked_required(&self) -> bool {
        match self {
            MapField::Scalar(f) => f.is_marked_required(),
            MapField::Enum(f) => f.is_marked_required(),
            MapField::Embed(f) => f.is_marked_required(),
        }
    }

    pub fn is_embed(&self) -> bool {
        matches!(self, MapField::Embed(_))
    }

    pub fn is_well_known_type(&self) -> bool {
        match self {
            MapField::Enum(f) => f.is_well_known_type(),
            MapField::Embed(f) => f.is_well_known_type(),
            MapField::Scalar(_) => false,
        }
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        match self {
            MapField::Enum(f) => f.well_known_type(),
            MapField::Embed(f) => f.well_known_type(),
            MapField::Scalar(_) => None,
        }
    }

    pub fn is_scalar(&self) -> bool {
        matches!(self, MapField::Scalar(_))
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, MapField::Enum(_))
    }

    pub(crate) fn has_presence(&self) -> bool {
        match self {
            MapField::Scalar(f) => f.has_presence(),
            MapField::Enum(f) => f.has_presence(),
            MapField::Embed(f) => f.has_presence(),
        }
    }

    pub(crate) fn set_value(&self, value: Node<'a>) -> Result<(), anyhow::Error> {
        match self {
            MapField::Enum(f) => f.set_value(value),
            MapField::Embed(f) => f.set_value(value),
            _ => panic!("set_value called on non-enum"),
        }
    }

    pub fn value_type(&self) -> Type<'a> {
        match self {
            MapField::Scalar(s) => s.value_type(),
            MapField::Enum(e) => e.value_type(),
            MapField::Embed(e) => e.value_type(),
        }
    }

    pub(crate) fn new(detail: FieldDetail<'a>) -> Result<Self, anyhow::Error> {
        let map_entry = detail.map_entry()?;
        let fields = map_entry.fields();

        let key = fields.get(0).ok_or_else(|| {
            anyhow!(
                "map entry {} key is missing a key field",
                map_entry.fully_qualified_name()
            )
        })?;

        let value = fields.get(1).ok_or_else(|| {
            anyhow!(
                "map entry {} is missing a value field",
                map_entry.fully_qualified_name()
            )
        })?;
        let key = key.value_type().try_into()?;
        let fd = MapFieldDetail { key, detail };
        let map_field = match value.value_type() {
            Type::Scalar(s) => MapField::Scalar(MappedScalarField::new(fd, s)),
            Type::Enum(_) => MapField::Enum(MappedEnumField::new(fd, value.enumeration())?),
            Type::Message(_) => MapField::Embed(MappedEmbedField::new(fd, value.embed())?),
            Type::Group => bail!("group is not supported"),
        };
        Ok(map_field)
    }

    pub fn as_field(self) -> Field<'a> {
        Field::Map(self)
    }

    /// The jstype option determines the JavaScript type used for values of the
    /// field.  The option is permitted only for 64 bit integral and fixed types
    /// (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
    /// is represented as JavaScript string, which avoids loss of precision that
    /// can happen when a large value is converted to a floating point JavaScript.
    /// Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
    /// use the JavaScript "number" type.  The behavior of the default option
    /// JS_NORMAL is implementation dependent.
    ///
    /// This option is an enum to permit additional types to be added, e.g.
    /// goog.math.Integer.
    pub fn jstype(&self) -> JsType {
        match self {
            MapField::Scalar(f) => f.jstype(),
            MapField::Enum(f) => f.jstype(),
            MapField::Embed(f) => f.jstype(),
        }
    }

    /// Should this field be parsed lazily?  Lazy applies only to message-type
    /// fields.  It means that when the outer message is initially parsed, the
    /// inner message's contents will not be parsed but instead stored in encoded
    /// form.  The inner message will actually be parsed when it is first accessed.
    ///
    /// This is only a hint.  Implementations are free to choose whether to use
    /// eager or lazy parsing regardless of the value of this option.  However,
    /// setting this option true suggests that the protocol author believes that
    /// using lazy parsing on this field is worth the additional bookkeeping
    /// overhead typically needed to implement it.
    ///
    /// This option does not affect the public interface of any generated code;
    /// all method signatures remain the same.  Furthermore, thread-safety of the
    /// interface is not affected by this option; const methods remain safe to
    /// call from multiple threads concurrently, while non-const methods continue
    /// to require exclusive access.
    ///
    ///
    /// Note that implementations may choose not to check required fields within
    /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
    /// may return true even if the inner message has missing required fields.
    /// This is necessary because otherwise the inner message would have to be
    /// parsed in order to perform the check, defeating the purpose of lazy
    /// parsing.  An implementation which chooses not to check required fields
    /// must be consistent about it.  That is, for any particular sub-message, the
    /// implementation must either *always* check its required fields, or *never*
    /// check its required fields, regardless of whether or not the message has
    /// been parsed.
    pub fn is_lazy(&self) -> bool {
        match self {
            MapField::Scalar(f) => f.is_lazy(),
            MapField::Enum(f) => f.is_lazy(),
            MapField::Embed(f) => f.is_lazy(),
        }
    }

    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub fn is_deprecated(&self) -> bool {
        match self {
            MapField::Scalar(f) => f.is_deprecated(),
            MapField::Enum(f) => f.is_deprecated(),
            MapField::Embed(f) => f.is_deprecated(),
        }
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        match self {
            MapField::Scalar(f) => f.uninterpreted_options(),
            MapField::Enum(f) => f.uninterpreted_options(),
            MapField::Embed(f) => f.uninterpreted_options(),
        }
    }
    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub fn is_packed(&self) -> bool {
        match self {
            MapField::Scalar(f) => f.is_packed(),
            MapField::Enum(f) => f.is_packed(),
            MapField::Embed(f) => f.is_packed(),
        }
    }

    pub fn message(&self) -> Message<'a> {
        match self {
            MapField::Scalar(f) => f.message(),
            MapField::Enum(f) => f.message(),
            MapField::Embed(f) => f.message(),
        }
    }

    pub fn number(&self) -> i32 {
        match self {
            MapField::Scalar(f) => f.number(),
            MapField::Enum(f) => f.number(),
            MapField::Embed(f) => f.number(),
        }
    }
}

impl<'a> From<MappedEnumField<'a>> for MapField<'a> {
    fn from(f: MappedEnumField<'a>) -> Self {
        MapField::Enum(f)
    }
}
impl<'a> From<&MappedEnumField<'a>> for MapField<'a> {
    fn from(f: &MappedEnumField<'a>) -> Self {
        MapField::Enum(f.clone())
    }
}

impl<'a> From<MappedScalarField<'a>> for MapField<'a> {
    fn from(f: MappedScalarField<'a>) -> Self {
        MapField::Scalar(f)
    }
}
impl<'a> From<&MappedScalarField<'a>> for MapField<'a> {
    fn from(f: &MappedScalarField<'a>) -> Self {
        MapField::Scalar(f.clone())
    }
}

impl<'a> From<MappedEmbedField<'a>> for MapField<'a> {
    fn from(f: MappedEmbedField<'a>) -> Self {
        MapField::Embed(f)
    }
}
impl<'a> From<&MappedEmbedField<'a>> for MapField<'a> {
    fn from(f: &MappedEmbedField<'a>) -> Self {
        MapField::Embed(f.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MapFieldDetail<'a> {
    key: Key,
    detail: FieldDetail<'a>,
}

impl<'a> MapFieldDetail<'a> {
    fn value_type(&self) -> Type<'a> {
        self.value_field()
            .expect("value_field is None")
            .value_type()
    }
    fn value_field(&self) -> Result<Field<'a>, anyhow::Error> {
        let map_entry = self.detail.map_entry()?;
        map_entry
            .fields()
            .get(1)
            .ok_or_else(|| anyhow!("map entry {} is missing value field", &map_entry.name()))
    }

    pub fn name(&self) -> &Name {
        self.detail.name()
    }

    pub fn key(&self) -> Key {
        self.key
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
    pub fn message(&self) -> Message<'a> {
        self.detail.message()
    }
    pub fn syntax(&self) -> Syntax {
        self.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.detail.descriptor()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.detail.comments()
    }
    pub fn set_comments(&self, comments: Comments<'a>) {
        self.detail.comments.replace(comments);
    }

    pub fn file(&self) -> File<'a> {
        self.detail.file()
    }
    pub fn package(&self) -> Package<'a> {
        self.detail.package()
    }

    pub fn build_target(&self) -> bool {
        self.detail.build_target()
    }

    pub fn is_marked_required(&self) -> bool {
        self.detail.is_marked_required()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MappedScalarFieldDetail<'a> {
    detail: MapFieldDetail<'a>,
    scalar: Scalar,
}

#[derive(Debug, Clone)]
pub struct MappedScalarField<'a>(Rc<MappedScalarFieldDetail<'a>>);

impl<'a> MappedScalarField<'a> {
    pub(crate) fn new(detail: MapFieldDetail<'a>, scalar: Scalar) -> Self {
        MappedScalarField(Rc::new(MappedScalarFieldDetail { detail, scalar }))
    }
    pub(crate) fn value_type(&self) -> Type<'a> {
        self.0.detail.value_type()
    }
    pub fn name(&self) -> &Name {
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
    pub fn message(&self) -> Message<'a> {
        self.0.detail.message()
    }

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }

    pub fn file(&self) -> File<'a> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a> {
        self.0.detail.package()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }

    pub fn build_target(&self) -> bool {
        self.0.detail.build_target()
    }

    pub fn scalar(&self) -> Scalar {
        self.0.scalar
    }

    pub fn key(&self) -> Key {
        self.0.detail.key()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }

    pub fn has_presence(&self) -> bool {
        false
    }
    /// The jstype option determines the JavaScript type used for values of the
    /// field.  The option is permitted only for 64 bit integral and fixed types
    /// (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
    /// is represented as JavaScript string, which avoids loss of precision that
    /// can happen when a large value is converted to a floating point JavaScript.
    /// Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
    /// use the JavaScript "number" type.  The behavior of the default option
    /// JS_NORMAL is implementation dependent.
    ///
    /// This option is an enum to permit additional types to be added, e.g.
    /// goog.math.Integer.
    pub fn jstype(&self) -> JsType {
        self.descriptor().options().jstype()
    }
    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub fn is_packed(&self) -> bool {
        self.descriptor().options().packed()
    }

    /// Should this field be parsed lazily?  Lazy applies only to message-type
    /// fields.  It means that when the outer message is initially parsed, the
    /// inner message's contents will not be parsed but instead stored in encoded
    /// form.  The inner message will actually be parsed when it is first accessed.
    ///
    /// This is only a hint.  Implementations are free to choose whether to use
    /// eager or lazy parsing regardless of the value of this option.  However,
    /// setting this option true suggests that the protocol author believes that
    /// using lazy parsing on this field is worth the additional bookkeeping
    /// overhead typically needed to implement it.
    ///
    /// This option does not affect the public interface of any generated code;
    /// all method signatures remain the same.  Furthermore, thread-safety of the
    /// interface is not affected by this option; const methods remain safe to
    /// call from multiple threads concurrently, while non-const methods continue
    /// to require exclusive access.
    ///
    ///
    /// Note that implementations may choose not to check required fields within
    /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
    /// may return true even if the inner message has missing required fields.
    /// This is necessary because otherwise the inner message would have to be
    /// parsed in order to perform the check, defeating the purpose of lazy
    /// parsing.  An implementation which chooses not to check required fields
    /// must be consistent about it.  That is, for any particular sub-message, the
    /// implementation must either *always* check its required fields, or *never*
    /// check its required fields, regardless of whether or not the message has
    /// been parsed.
    pub fn is_lazy(&self) -> bool {
        self.descriptor().options().is_lazy()
    }

    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub fn is_deprecated(&self) -> bool {
        self.descriptor().options().is_deprecated()
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        self.descriptor().options().uninterpreted_options()
    }

    pub fn number(&self) -> i32 {
        self.descriptor().number()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MappedEmbedFieldDetail<'a> {
    embed: RefCell<WeakMessage<'a>>,
    detail: MapFieldDetail<'a>,
}
impl<'a> MappedEmbedFieldDetail<'a> {
    pub(crate) fn embed(&self) -> WeakMessage<'a> {
        self.embed.borrow().clone()
    }
    pub(crate) fn value_type(&self) -> Type<'a> {
        self.detail.value_type()
    }
}

#[derive(Debug, Clone)]
pub struct MappedEmbedField<'a>(Rc<MappedEmbedFieldDetail<'a>>);

impl<'a> MappedEmbedField<'a> {
    fn new(detail: MapFieldDetail<'a>, embed: Option<Message<'a>>) -> anyhow::Result<Self> {
        embed
            .map(|e| {
                Self(Rc::new(MappedEmbedFieldDetail {
                    embed: RefCell::new(e.into()),
                    detail,
                }))
            })
            .ok_or_else(|| anyhow!("expected Embed, received None"))
    }

    pub fn name(&self) -> &Name {
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
    /// Returns the Message which contains this field.
    pub fn message(&self) -> Message<'a> {
        self.0.detail.message()
    }

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }
    /// Returns the embedded message.
    pub fn embed(&self) -> Message<'a> {
        self.0.embed().into()
    }

    pub fn has_presence(&self) -> bool {
        false
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }
    pub fn file(&self) -> File<'a> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a> {
        self.0.detail.package()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }

    pub fn has_import(&self) -> bool {
        self.file() != self.0.embed().file()
    }
    pub fn imports(&self) -> FileRefs<'a> {
        if self.has_import() {
            FileRefs::from(self.0.embed().weak_file())
        } else {
            FileRefs::empty()
        }
    }

    pub fn build_target(&self) -> bool {
        self.0.detail.build_target()
    }

    pub fn key(&self) -> Key {
        self.0.detail.key()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.0.embed().is_well_known_type()
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.0.embed().well_known_type()
    }
    pub fn well_known_message(&self) -> Option<WellKnownMessage> {
        self.0.embed().well_known_message()
    }

    /// The jstype option determines the JavaScript type used for values of the
    /// field.  The option is permitted only for 64 bit integral and fixed types
    /// (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
    /// is represented as JavaScript string, which avoids loss of precision that
    /// can happen when a large value is converted to a floating point JavaScript.
    /// Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
    /// use the JavaScript "number" type.  The behavior of the default option
    /// JS_NORMAL is implementation dependent.
    ///
    /// This option is an enum to permit additional types to be added, e.g.
    /// goog.math.Integer.
    pub fn jstype(&self) -> JsType {
        self.descriptor().options().jstype()
    }

    pub(crate) fn set_value(&self, node: Node<'a>) -> Result<(), anyhow::Error> {
        match node {
            Node::Message(m) => {
                self.0.embed.replace(m.into());
                Ok(())
            }
            _ => bail!("expected Message, received {}", node),
        }
    }

    pub(crate) fn value_type(&self) -> Type<'a> {
        self.0.value_type()
    }

    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub fn is_packed(&self) -> bool {
        self.descriptor().options().packed()
    }

    /// Should this field be parsed lazily?  Lazy applies only to message-type
    /// fields.  It means that when the outer message is initially parsed, the
    /// inner message's contents will not be parsed but instead stored in encoded
    /// form.  The inner message will actually be parsed when it is first accessed.
    ///
    /// This is only a hint.  Implementations are free to choose whether to use
    /// eager or lazy parsing regardless of the value of this option.  However,
    /// setting this option true suggests that the protocol author believes that
    /// using lazy parsing on this field is worth the additional bookkeeping
    /// overhead typically needed to implement it.
    ///
    /// This option does not affect the public interface of any generated code;
    /// all method signatures remain the same.  Furthermore, thread-safety of the
    /// interface is not affected by this option; const methods remain safe to
    /// call from multiple threads concurrently, while non-const methods continue
    /// to require exclusive access.
    ///
    ///
    /// Note that implementations may choose not to check required fields within
    /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
    /// may return true even if the inner message has missing required fields.
    /// This is necessary because otherwise the inner message would have to be
    /// parsed in order to perform the check, defeating the purpose of lazy
    /// parsing.  An implementation which chooses not to check required fields
    /// must be consistent about it.  That is, for any particular sub-message, the
    /// implementation must either *always* check its required fields, or *never*
    /// check its required fields, regardless of whether or not the message has
    /// been parsed.
    pub fn is_lazy(&self) -> bool {
        self.descriptor().options().is_lazy()
    }

    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub fn is_deprecated(&self) -> bool {
        self.descriptor().options().is_deprecated()
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        self.descriptor().options().uninterpreted_options()
    }

    pub fn number(&self) -> i32 {
        self.descriptor().number()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MappedEnumFieldDetail<'a> {
    enumeration: RefCell<WeakEnum<'a>>,
    detail: MapFieldDetail<'a>,
}

impl<'a> MappedEnumFieldDetail<'a> {
    pub fn enumeration(&self) -> Enum<'a> {
        self.enumeration.borrow().clone().into()
    }
    pub(crate) fn value_type(&self) -> Type<'a> {
        self.detail.value_type()
    }
}

#[derive(Debug, Clone)]
pub struct MappedEnumField<'a>(Rc<MappedEnumFieldDetail<'a>>);

impl<'a> MappedEnumField<'a> {
    fn new(detail: MapFieldDetail<'a>, e: Option<Enum<'a>>) -> anyhow::Result<Self> {
        e.map(|e| {
            Self(Rc::new(MappedEnumFieldDetail {
                enumeration: RefCell::new(e.into()),
                detail,
            }))
        })
        .ok_or_else(|| anyhow!("expected Enum, received None"))
    }
    pub fn name(&self) -> &Name {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> String {
        self.0.detail.fully_qualified_name()
    }
    pub fn message(&self) -> Message<'a> {
        self.0.detail.message()
    }

    pub fn is_repeated(&self) -> bool {
        self.0.detail.is_repeated()
    }
    pub fn is_map(&self) -> bool {
        self.0.detail.is_map()
    }

    pub fn syntax(&self) -> Syntax {
        self.0.detail.syntax()
    }
    pub fn descriptor(&self) -> FieldDescriptor<'a> {
        self.0.detail.descriptor()
    }
    pub fn file(&self) -> File<'a> {
        self.0.detail.file()
    }
    pub fn package(&self) -> Package<'a> {
        self.0.detail.package()
    }

    /// Alias for `enum_()`
    pub fn enumeration(&self) -> Enum<'a> {
        self.enum_()
    }
    pub fn enum_(&self) -> Enum<'a> {
        self.0.enumeration()
    }
    pub fn comments(&self) -> Comments<'a> {
        self.0.detail.comments()
    }
    pub fn well_known_type(&self) -> Option<WellKnownType> {
        self.enumeration().well_known_type()
    }
    pub fn well_known_enum(&self) -> Option<WellKnownEnum> {
        self.enumeration().well_known_enum()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.enumeration().is_well_known_type()
    }

    pub(crate) fn set_comments(&self, comments: Comments<'a>) {
        self.0.detail.set_comments(comments);
    }

    pub fn has_import(&self) -> bool {
        return self.enumeration().file() != self.file();
    }
    pub fn imports(&self) -> FileRefs<'a> {
        if self.has_import() {
            FileRefs::from(self.enumeration().weak_file())
        } else {
            FileRefs::empty()
        }
    }

    pub fn build_target(&self) -> bool {
        self.0.detail.build_target()
    }

    pub fn key(&self) -> Key {
        self.0.detail.key()
    }

    pub fn is_marked_required(&self) -> bool {
        self.0.detail.is_marked_required()
    }

    pub fn has_presence(&self) -> bool {
        false
    }
    /// The jstype option determines the JavaScript type used for values of the
    /// field.  The option is permitted only for 64 bit integral and fixed types
    /// (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
    /// is represented as JavaScript string, which avoids loss of precision that
    /// can happen when a large value is converted to a floating point JavaScript.
    /// Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
    /// use the JavaScript "number" type.  The behavior of the default option
    /// JS_NORMAL is implementation dependent.
    ///
    /// This option is an enum to permit additional types to be added, e.g.
    /// goog.math.Integer.
    pub fn jstype(&self) -> JsType {
        self.descriptor().options().jstype()
    }
    fn set_value(&self, value: Node<'a>) -> Result<(), anyhow::Error> {
        match value {
            Node::Enum(e) => {
                self.0.enumeration.replace(e.into());
                Ok(())
            }
            _ => bail!("expected Enum, received {}", value),
        }
    }

    fn value_type(&self) -> Type<'a> {
        self.0.value_type()
    }

    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub fn is_packed(&self) -> bool {
        self.descriptor().options().packed()
    }

    /// Should this field be parsed lazily?  Lazy applies only to message-type
    /// fields.  It means that when the outer message is initially parsed, the
    /// inner message's contents will not be parsed but instead stored in encoded
    /// form.  The inner message will actually be parsed when it is first accessed.
    ///
    /// This is only a hint.  Implementations are free to choose whether to use
    /// eager or lazy parsing regardless of the value of this option.  However,
    /// setting this option true suggests that the protocol author believes that
    /// using lazy parsing on this field is worth the additional bookkeeping
    /// overhead typically needed to implement it.
    ///
    /// This option does not affect the public interface of any generated code;
    /// all method signatures remain the same.  Furthermore, thread-safety of the
    /// interface is not affected by this option; const methods remain safe to
    /// call from multiple threads concurrently, while non-const methods continue
    /// to require exclusive access.
    ///
    ///
    /// Note that implementations may choose not to check required fields within
    /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
    /// may return true even if the inner message has missing required fields.
    /// This is necessary because otherwise the inner message would have to be
    /// parsed in order to perform the check, defeating the purpose of lazy
    /// parsing.  An implementation which chooses not to check required fields
    /// must be consistent about it.  That is, for any particular sub-message, the
    /// implementation must either *always* check its required fields, or *never*
    /// check its required fields, regardless of whether or not the message has
    /// been parsed.
    pub fn is_lazy(&self) -> bool {
        self.descriptor().options().is_lazy()
    }

    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub fn is_deprecated(&self) -> bool {
        self.descriptor().options().is_deprecated()
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        self.descriptor().options().uninterpreted_options()
    }

    pub fn number(&self) -> i32 {
        self.descriptor().number()
    }
}
