use super::{MethodDescriptor, UninterpretedOption};

// Describes a service.
#[derive(Clone, PartialEq, Debug)]
pub struct ServiceDescriptor {
    name: Option<String>,
    methods: Vec<MethodDescriptor>,
    options: Option<ServiceOptions>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ServiceOptions {
    // Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
    //   framework.  We apologize for hoarding these numbers to ourselves, but
    //   we were already using them long before we decided to release Protocol
    //   Buffers.
    /// Is this service deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the service, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating services.
    deprecated: Option<bool>,

    /// The parser stores options it doesn't recognize here. See above.
    uninterpreted_options: Vec<UninterpretedOption>,
}
