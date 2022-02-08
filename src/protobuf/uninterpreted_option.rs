/// The name of the uninterpreted option.  Each string represents a segment in
/// a dot-separated name.  is_extension is true iff a segment represents an
/// extension (denoted with parentheses in options specs in .proto files).
/// E.g.,{ ["foo", false], ["bar.baz", true], ["qux", false] } represents
/// "foo.(bar.baz).qux".
#[derive(Clone, PartialEq, Eq, Debug)]
struct NamePart {
    name_part: String,
    is_extension: bool,
}
impl NamePart {
    pub(crate) fn new(np: &prost_types::uninterpreted_option::NamePart) -> Self {
        NamePart {
            name_part: np.name_part.clone(),
            is_extension: np.is_extension.clone(),
        }
    }
    pub fn part(&self) -> String {
        self.name_part.clone()
    }
    pub fn is_extension(&self) -> bool {
        self.is_extension.clone()
    }
}

/// A message representing a option the parser does not recognize. This only
/// appears in options protos created by the compiler::Parser class.
/// DescriptorPool resolves these when building Descriptor objects. Therefore,
/// options protos in descriptor objects (e.g. returned by Descriptor::options(),
/// or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
/// in them.
#[derive(Clone, PartialEq, Debug)]
pub struct UninterpretedOption {
    /// tag: 2
    name: Vec<NamePart>,
    /// tag: 3
    /// The value of the uninterpreted option, in whatever type the tokenizer
    /// identified it as during parsing. Exactly one of these should be set.
    identifier_value: Option<String>,
    ///tag: 4
    positive_int_value: Option<u64>,
    ///tag: 5
    negative_int_value: Option<i64>,
    ///tag: 6
    double_value: Option<f64>,
    ///tag: 7
    string_value: Option<Vec<u8>>,
    ///tag: 8
    aggregate_value: Option<String>,
}

impl UninterpretedOption {
    pub(crate) fn new(uo: &prost_types::UninterpretedOption) -> Self {
        UninterpretedOption {
            name: uo.name.iter().map(|np| NamePart::new(np)).collect(),
            identifier_value: uo.identifier_value.clone(),
            positive_int_value: uo.positive_int_value.clone(),
            negative_int_value: uo.negative_int_value.clone(),
            double_value: uo.double_value.clone(),
            string_value: uo.string_value.clone(),
            aggregate_value: uo.aggregate_value.clone(),
        }
    }
    pub fn name(&self) -> Vec<NamePart> {
        self.name.iter().map(|p| p.clone()).collect()
    }
    pub fn identifier_value(&self) -> Option<String> {
        self.identifier_value.clone()
    }
    pub fn positive_int_value(&self) -> Option<u64> {
        self.positive_int_value.clone()
    }
    pub fn negative_int_value(&self) -> Option<i64> {
        self.negative_int_value.clone()
    }
    pub fn double_value(&self) -> Option<f64> {
        self.double_value.clone()
    }
    pub fn string_value(&self) -> Option<Vec<u8>> {
        self.string_value.clone()
    }
    pub fn aggregate_value(&self) -> Option<String> {
        self.aggregate_value.clone()
    }
}
