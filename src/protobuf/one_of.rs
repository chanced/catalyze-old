// use prost_types::OneofOptions;

use super::UninterpretedOption;

#[derive(Clone, PartialEq, Debug)]
/// Describes a oneof.
pub struct OneOfDescriptor {
    name: Option<String>,
    options: Option<OneOfOptions>,
}

impl OneOfDescriptor {
    pub(crate) fn new(o: &prost_types::OneofDescriptorProto) -> Self {
        OneOfDescriptor {
            name: o.name.clone(),
            options: o.options.map(|o| OneOfOptions::new(&o)),
        }
    }
    /// Returns the name of the oneof.
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    /// Returns the options of the oneof.
    pub fn options(&self) -> Option<OneOfOptions> {
        self.options.clone()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct OneOfOptions {
    /// The parser stores options it doesn't recognize here. See above.
    uninterpreted_options: Vec<UninterpretedOption>,
}

impl OneOfOptions {
    pub(crate) fn new(o: &prost_types::OneofOptions) -> Self {
        OneOfOptions {
            uninterpreted_options: o
                .uninterpreted_option
                .iter()
                .map(|uo| UninterpretedOption::new(uo))
                .collect(),
        }
    }
    pub fn uninterpreted_options(&self) -> Vec<UninterpretedOption> {
        self.uninterpreted_options.clone()
    }
}
