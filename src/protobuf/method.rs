use super::{Opt, Syntax, UninterpretedOption};

#[derive(Clone, PartialEq, Debug)]
pub struct Method {
    /// The simple name of this method.
    pub name: String,
    /// A URL of the input message type.
    pub request_type_url: String,
    /// If true, the request is streamed.
    pub request_streaming: bool,
    /// The URL of the output message type.
    pub response_type_url: String,
    /// If true, the response is streamed.
    pub response_streaming: bool,
    /// Any metadata attached to the method.
    pub options: Vec<Opt>,
    /// The source syntax of this method.
    pub syntax: Syntax,
}

impl Method {
    pub(crate) fn new(m: &prost_types::Method) -> Self {
        Method {
            name: m.name.clone(),
            request_type_url: m.request_type_url.clone(),
            request_streaming: m.request_streaming.clone(),
            response_type_url: m.response_type_url.clone(),
            response_streaming: m.response_streaming.clone(),
            options: m.options.iter().map(Opt::new).collect(),
            syntax: Syntax::from(m.syntax),
        }
    }
}

/// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
/// or neither? HTTP based RPC implementation may choose GET verb for safe
/// methods, and PUT verb for idempotent methods instead of the default POST.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum IdempotencyLevel {
    Unknown = 0,
    /// implies idempotent
    NoSideEffects = 1,
    /// idempotent, but may have side effects
    Idempotent = 2,
}

impl From<i32> for IdempotencyLevel {
    fn from(i: i32) -> Self {
        match i {
            1 => IdempotencyLevel::NoSideEffects,
            2 => IdempotencyLevel::Idempotent,
            _ => IdempotencyLevel::Unknown,
        }
    }
}

impl From<Option<i32>> for IdempotencyLevel {
    fn from(i: Option<i32>) -> Self {
        match i {
            Some(i) => From::from(i),
            None => IdempotencyLevel::Unknown,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct MethodDescriptor {
    name: Option<String>,
    /// Input and output type names.  These are resolved in the same way as
    /// FieldDescriptorProto.type_name, but must refer to a message type.
    pub input_type: Option<String>,
    pub output_type: Option<String>,
    pub options: Option<MethodOptions>,
    pub client_streaming: Option<bool>,
    pub server_streaming: Option<bool>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MethodOptions {
    // Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
    //   framework.  We apologize for hoarding these numbers to ourselves, but
    //   we were already using them long before we decided to release Protocol
    //   Buffers.
    /// Is this method deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the method, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating methods.
    pub deprecated: Option<bool>,
    pub idempotency_level: IdempotencyLevel,
    /// The parser stores options it doesn't recognize here.
    pub uninterpreted_options: Vec<UninterpretedOption>,
}

impl MethodOptions {
    pub(crate) fn new(desc: &prost_types::MethodOptions) -> Self {
        Self {
            deprecated: desc.deprecated.clone(),
            idempotency_level: IdempotencyLevel::from(desc.idempotency_level),
            uninterpreted_options: desc
                .uninterpreted_option
                .iter()
                .map(UninterpretedOption::new)
                .collect(),
        }
    }
}
