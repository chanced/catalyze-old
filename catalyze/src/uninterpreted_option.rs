use ::std::fmt;

/// A message representing an option that parser does not recognize.
#[derive(Debug, Clone)]
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
        self.identifier_value.as_deref()
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
        self.string_value.as_deref()
    }

    pub fn aggregate_value(&self) -> Option<&str> {
        self.aggregate_value.as_deref()
    }
}

///  The name of the uninterpreted option.  Each string represents a segment in
///  a dot-separated name.
///
///  E.g.,`{ ["foo", false], ["bar.baz", true], ["qux", false] }` represents
///  `"foo.(bar.baz).qux"`.
#[derive(PartialEq, Hash, Clone, Default, Debug)]
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

    /// Returns the formatted value of the `NamePart`
    ///
    /// If `is_extension` is `true`, the formatted value will be wrapped in
    /// parentheses.
    pub fn formatted(&self) -> String {
        if self.part.is_extension() {
            format!("({})", self.part.name_part())
        } else {
            self.part.name_part().to_string()
        }
    }
    pub fn as_str(&self) -> &str {
        self.part.name_part()
    }
}

impl AsRef<str> for NamePart {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for NamePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<protobuf::descriptor::uninterpreted_option::NamePart> for NamePart {
    fn from(part: protobuf::descriptor::uninterpreted_option::NamePart) -> Self {
        Self::from(&part)
    }
}
impl From<&protobuf::descriptor::uninterpreted_option::NamePart> for NamePart {
    fn from(part: &protobuf::descriptor::uninterpreted_option::NamePart) -> Self {
        Self {
            is_extension: part.is_extension,
            name_part: part.name_part.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NameParts<'a> {
    parts: &'a [NamePart],
}

impl ToString for NameParts<'_> {
    fn to_string(&self) -> String {
        self.formatted()
    }
}

impl<'a> std::iter::IntoIterator for &'a NameParts {
    type Item = &'a NamePart;
    type IntoIter = std::slice::Iter<'a, NamePart>;
    fn into_iter(self) -> Self::IntoIter {
        self.parts
            .iter()
            .map(NamePart::from)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl NameParts {
    pub fn iter(&self) -> std::slice::Iter<'_, NamePart> {
        self.into()
    }
    pub fn get(&self, idx: usize) -> Option<NamePart> {
        self.parts.get(idx).map(NamePart::from)
    }

    pub fn len(&self) -> usize {
        self.parts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
    pub fn contains(&self, part: &str) -> bool {
        self.parts.iter().any(|p| p.name_part() == part)
    }
    pub fn formatted(&self) -> String {
        itertools::join(self.iter().map(|v| v.formatted()), '.')
    }
}
