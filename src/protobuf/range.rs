use super::UninterpretedOption;
/// Nested message and enum types in `DescriptorProto`.
#[derive(Clone, PartialEq, Debug)]
pub struct ExtensionRange {
    /// tag: 1
    /// Inclusive
    start: Option<i32>,
    /// tag: 2
    /// Exclusive
    end: Option<i32>,
    /// tag: 3
    options: Option<ExtensionRangeOptions>,
}

impl ExtensionRange {
    pub(crate) fn new(desc: &prost_types::descriptor_proto::ExtensionRange) -> Self {
        ExtensionRange {
            start: desc.start.clone(),
            end: desc.end.clone(),
            options: desc.options.map(|o| ExtensionRangeOptions::new(o)),
        }
    }
    pub fn start(&self) -> Option<i32> {
        self.start.clone()
    }
    pub fn end(&self) -> Option<i32> {
        self.end.clone()
    }
    pub fn options(&self) -> Option<ExtensionRangeOptions> {
        self.options.clone()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ExtensionRangeOptions {
    uninterpreted_options: Vec<UninterpretedOption>,
}

impl ExtensionRangeOptions {
    pub(crate) fn new(ero: prost_types::ExtensionRangeOptions) -> Self {
        ExtensionRangeOptions {
            uninterpreted_options: ero
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

/// Range of reserved tag numbers. Reserved tag numbers may not be used by
/// fields or extension ranges in the same message. Reserved ranges may
/// not overlap.
#[derive(Clone, PartialEq, Debug)]
pub struct ReservedRange {
    /// Inclusive
    start: Option<i32>,
    /// Exclusive
    end: Option<i32>,
}

impl ReservedRange {
    pub(crate) fn new(rr: &prost_types::descriptor_proto::ReservedRange) -> Self {
        ReservedRange {
            start: rr.start.clone(),
            end: rr.end.clone(),
        }
    }
    /// Inclusive
    pub fn start(&self) -> Option<i32> {
        self.start.clone()
    }
    /// Exclusive
    pub fn end(&self) -> Option<i32> {
        self.end.clone()
    }
}
