use anyhow::bail;

use super::UninterpretedOption;

#[derive(Clone, PartialEq, Debug)]
pub struct MessageOptions {
    /// Set true to use the old proto1 MessageSet wire format for extensions.
    /// This is provided for backwards-compatibility with the MessageSet wire
    /// format.  You should not use this for any other reason:  It's less
    /// efficient, has fewer features, and is more complicated.
    ///
    /// The message must be defined exactly as follows:
    ///   message Foo {
    ///     option message_set_wire_format = true;
    ///     extensions 4 to max;
    ///   }
    /// Note that the message cannot have any defined fields; MessageSets only
    /// have extensions.
    ///
    /// All extensions of your type must be singular messages; e.g. they cannot
    /// be int32s, enums, or repeated messages.
    ///
    /// Because this is an option, the above two restrictions are not enforced by
    /// the protocol compiler.
    ///
    /// default: `false`
    pub message_set_wire_format: Option<bool>,
    /// Disables the generation of the standard "descriptor()" accessor, which can
    /// conflict with a field of the same name.  This is meant to make migration
    /// from proto1 easier; new code should avoid fields named "descriptor".
    ///
    /// default: `false`
    pub no_standard_descriptor_accessor: Option<bool>,
    /// Is this message deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the message, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating messages.
    ///
    /// default: `false`
    pub deprecated: Option<bool>,
    /// Whether the message is an automatically generated map entry type for the
    /// maps field.
    ///
    /// For maps fields:
    ///     map<KeyType, ValueType> map_field = 1;
    /// The parsed descriptor looks like:
    ///     message MapFieldEntry {
    ///         option map_entry = true;
    ///         optional KeyType key = 1;
    ///         optional ValueType value = 2;
    ///     }
    ///     repeated MapFieldEntry map_field = 1;
    ///
    /// Implementations may choose not to generate the map_entry=true message, but
    /// use a native map in the target language to hold the keys and values.
    /// The reflection APIs in such implementations still need to work as
    /// if the field is a repeated message field.
    ///
    /// NOTE: Do not set the option in .proto files. Always use the maps syntax
    /// instead. The option should only be implicitly set by the proto compiler
    /// parser.
    pub map_entry: Option<bool>,
    /// The parser stores options it doesn't recognize here.
    pub uninterpreted_options: Vec<UninterpretedOption>,
}

impl MessageOptions {
    pub(crate) fn new(mo: &prost_types::MessageOptions) -> Self {
        MessageOptions {
            message_set_wire_format: mo.message_set_wire_format.clone(),
            no_standard_descriptor_accessor: mo.no_standard_descriptor_accessor.clone(),
            deprecated: mo.deprecated.clone(),
            map_entry: mo.map_entry.clone(),
            uninterpreted_options: mo
                .uninterpreted_option
                .iter()
                .map(UninterpretedOption::new)
                .collect(),
        }
    }
}
impl TryFrom<Option<prost_types::MessageOptions>> for MessageOptions {
    type Error = anyhow::Error;
    fn try_from(mo: Option<prost_types::MessageOptions>) -> Result<Self, Self::Error> {
        match mo {
            Some(mo) => Ok(MessageOptions::new(&mo)),
            None => bail!("MessageOptions is not set"),
        }
    }
}
