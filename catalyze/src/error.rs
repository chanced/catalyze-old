use crate::{Field, File, Kind, MethodIo, Node, Syntax};
use snafu::Snafu;

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(
        display("Invalid node type for {}; expected {expected}, found {}", node.kind()),
        visibility(pub(crate))
    )]
    InvalidNode {
        expected: Kind,
        /// node may be None if the Node is unable to be constructed
        /// as a result of the mismatch
        node: Node,
    },

    #[snafu(display("Node with fully qualified name {} not found", fully_qualified_name))]
    NodeNotFound { fully_qualified_name: String },

    #[snafu(display("Group is not supported; use an embedded message instead"))]
    GroupNotSupported { fully_qualified_name: String },

    #[snafu(display("Extendee not found: {}", extendee))]
    ExtendeeNotFound { extendee: String },

    #[snafu(display("Dependency of {} not found: {}", dependee.fully_qualified_name(), dependency))]
    DependencyNotFound { dependency: String, dependee: File },

    #[snafu(display("Missing method {method_io:?} for {fully_qualified_name}"))]
    MissingMethod {
        fully_qualified_name: String,
        method_io: MethodIo,
    },
    #[snafu(display("Invalid map entry {fully_qualified_name:?}: {reason}"))]
    InvalidMapEntry {
        reason: InvalidMapEntryReason,
        fully_qualified_name: String,
        name: String,
        syntax: Syntax,
    },
    #[snafu(display("Unknown syntax: {value:?}; expected either \"proto2\" or \"proto3\""))]
    UnknownSyntax { value: String },

    #[snafu(display("Failed to execute, caused by:\n{source}"))]
    FailedToExecute {
        source: Box<dyn 'static + std::error::Error + Send + Sync>,
    },

    #[snafu(display("Invalid type: {value:?}"))]
    InvalidType { value: String },

    #[snafu(display("Invalid map key type: {:?}", field.value_type()))]
    MapKeyType { field: Field },

    #[snafu(display("Invalid scalar: {value}; expected a value between 1 and 18 (inclusive)"))]
    InvalidScalar { value: i32 },

    #[snafu(display("Invalid C type: {value}; expected a value between 0 and 2 (inclusive)"))]
    InvalidCType { value: i32 },

    #[snafu(display("Invalid label: {value}; expected a value between 1 and 3 (inclusive)"))]
    InvalidLabel { value: i32 },

    #[snafu(display(
        "Invalid optimize mode: {value}; expected a value between 1 and 3 (inclusive)"
    ))]
    InvalidOptimizeMode { value: i32 },

    #[snafu(display(
        "Invalid idempotency level: {value}; expected a value between 0 and 2 (inclusive)"
    ))]
    InvalidIdempotencyLevel { value: i32 },

    #[snafu(display("Not a well-known type: \"\""))]
    NotWellKnownTyoe { value: String },

    #[snafu(display("Unknown File Descriptor path: {path}"))]
    UnknownFileDecriptorPath { path: i32 },
}

impl Error {
    pub(crate) fn invalid_node(expected: Kind, node: Node) -> Self {
        Error::InvalidNode { expected, node }
    }
    pub(crate) fn group_not_supported(fully_qualified_name: impl AsRef<str>) -> Self {
        Error::GroupNotSupported {
            fully_qualified_name: fully_qualified_name.as_ref().to_string(),
        }
    }

    pub(crate) fn invalid_type(value: impl AsRef<str>) -> Self {
        Error::InvalidType {
            value: value.as_ref().to_string(),
        }
    }

    pub(crate) fn invalid_map_key_type(field: Field) -> Self {
        Error::MapKeyType { field }
    }
    pub(crate) fn invalid_scalar(value: i32) -> Self {
        Error::InvalidScalar { value }
    }
    pub(crate) fn invalid_c_type(value: i32) -> Self {
        Error::InvalidCType { value }
    }
    pub(crate) fn invalid_label(value: i32) -> Self {
        Error::InvalidLabel { value }
    }
    pub(crate) fn invalid_optimize_mode(value: i32) -> Self {
        Error::InvalidOptimizeMode { value }
    }
    pub(crate) fn invalid_idempotency_level(value: i32) -> Self {
        Error::InvalidIdempotencyLevel { value }
    }
    pub(crate) fn not_well_known_type(value: impl AsRef<str>) -> Self {
        Error::NotAWellKnownType {
            value: value.as_ref().to_string(),
        }
    }
    pub(crate) fn invalid_map_entry(
        reason: InvalidMapEntryReason,
        fully_qualified_name: impl AsRef<str>,
        name: impl AsRef<str>,
        syntax: Syntax,
    ) -> Self {
        Error::InvalidMapEntry {
            reason,
            fully_qualified_name: fully_qualified_name.as_ref().to_string(),
            name: name.as_ref().to_string(),
            syntax,
        }
    }
    pub(crate) fn invalid_syntax(value: String) -> Self {
        Error::UnknownSyntax { value }
    }

    pub(crate) fn map_entry_missing_value(
        fully_qualified_name: impl AsRef<str>,
        name: impl AsRef<str>,
        syntax: Syntax,
    ) -> Self {
        Self::invalid_map_entry(
            InvalidMapEntryReason::MissingValue,
            fully_qualified_name,
            name,
            syntax,
        )
    }
    pub(crate) fn embed_not_map(
        fully_qualified_name: impl AsRef<str>,
        name: impl AsRef<str>,
        syntax: Syntax,
    ) -> Self {
        Self::invalid_map_entry(
            InvalidMapEntryReason::EmbedNotMap,
            fully_qualified_name,
            name,
            syntax,
        )
    }
    pub(crate) fn map_entry_missing_key(
        fully_qualified_name: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> Self {
        Self::invalid_map_entry(
            InvalidMapEntryReason::MissingKey,
            fully_qualified_name,
            name,
            Syntax::Proto3,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InvalidMapEntryReason {
    MissingValue,
    MissingKey,
    FieldNotEmbed,
    EmbedNotMap,
}
impl std::fmt::Display for InvalidMapEntryReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidMapEntryReason::MissingValue => "missing map value field".fmt(f),
            InvalidMapEntryReason::MissingKey => "missing map key field".fmt(f),
            InvalidMapEntryReason::FieldNotEmbed => "field does not of the appropriate type".fmt(f),
            InvalidMapEntryReason::EmbedNotMap => "embedded message is not a map".fmt(f),
        }
    }
}

#[derive(Debug, Snafu)]
pub(crate) enum InputErrorInner {
    Standalone { source: protobuf::Error },
}

#[derive(Debug, Snafu)]
#[snafu(display("Failed to parse input: {}", source))]
pub struct InputError {
    source: InputErrorInner,
    backtrace: snafu::Backtrace,
}
