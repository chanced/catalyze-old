use super::Any;

/// A protocol buffer option, which can be attached to a message, field,
/// enumeration, etc.
///
#[derive(Clone, PartialEq, Debug)]
pub struct Opt {
    name: String,
    value: Option<Any>,
}

impl Opt {
    pub(crate) fn new(opt: &prost_types::Option) -> Self {
        Opt {
            name: opt.name.clone(),
            value: Any::try_from(opt.value).ok(),
        }
    }
    /// The option's name. For protobuf built-in options (options defined in
    /// descriptor.proto), this is the short name. For example, `"map_entry"`.
    /// For custom options, it should be the fully-qualified name. For example,
    /// `"google.api.http"`.
    pub fn name(&self) -> String {
        self.name.clone()
    }
    /// The option's value packed in an Any message. If the value is a primitive,
    /// the corresponding wrapper type defined in google/protobuf/wrappers.proto
    /// should be used. If the value is an enum, it should be stored as an int32
    /// value using the google.protobuf.Int32Value type.
    pub fn value(&self) -> Option<Any> {
        self.value.clone()
    }
}
