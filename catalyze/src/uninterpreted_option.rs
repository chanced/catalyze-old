use ::std::{option::Option, string::String, vec::Vec};

/// A message representing an option that parser does not recognize.
pub struct UninterpretedOption {
    name: Vec<NamePart>,
    identifier_value: Option<String>,
    positive_int_value: Option<u64>,
    negative_int_value: Option<i64>,
    double_value: Option<f64>,
    string_value: Option<Vec<u8>>,
    aggregate_value: Option<String>,
}

impl UninterpretedOption {
    pub fn name(&self) -> &[NamePart] {
        self.name.as_ref()
    }

    pub fn identifier_value(&self) -> Option<&str> {
        self.identifier_value.as_ref()
    }

    pub fn positive_int_value(&self) -> Option<u64> {
        self.positive_int_value
    }

    pub fn negative_int_value(&self) -> Option<i64> {
        self.negative_int_value
    }

    pub fn double_value(&self) -> Option<f64> {
        self.double_value
    }

    pub fn string_value(&self) -> Option<&[u8]> {
        self.string_value.as_ref()
    }

    pub fn aggregate_value(&self) -> Option<&str> {
        self.aggregate_value.as_ref()
    }
}

///  The name of the uninterpreted option.  Each string represents a segment in
///  a dot-separated name.  
///
///  E.g.,`{ ["foo", false], ["bar.baz", true], ["qux", false] }` represents
///  `"foo.(bar.baz).qux"`.
#[derive(PartialEq, Clone, Default, Debug)]
pub struct NamePart {
    name_part: Option<String>,
    is_extension: Option<bool>,
}

impl NamePart {
    pub fn name_part(&self) -> Option<&String> {
        self.name_part.as_ref()
    }
    /// true if a segment represents an extension (denoted with parentheses in
    ///  options specs in .proto files).
    pub fn is_extension(&self) -> Option<bool> {
        self.is_extension
    }
}

// pub struct NamePart {

// }
