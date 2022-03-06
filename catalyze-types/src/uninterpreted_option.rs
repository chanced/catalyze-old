use crate::Impl;
use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

/// A message representing a option the parser does not recognize. This only
/// appears in options protos created by the compiler::Parser class.
/// DescriptorPool resolves these when building Descriptor objects. Therefore,
/// options protos in descriptor objects (e.g. returned by Descriptor::options(),
/// or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
/// in them.
pub trait UninterpretedOption<'a, I: Impl<'a>>:
    core::fmt::Debug + Copy + Clone + IntoIterator<Item = I::NamePart<'a>, IntoIter = I::NamePartIter>
{
    fn name_parts(&self) -> dyn NameParts<'a>;
    fn identifier_value(&self) -> &'a str;
    fn positive_int_value(&self) -> u64;
    fn negative_int_value(&self) -> i64;
    fn double_value(&self) -> f64;
    fn string_value(&self) -> &'a [u8];
    fn aggregate_value(&self) -> &'a str;
}

/// The name of the uninterpreted option.  Each string represents a segment in
/// a dot-separated name. `is_extension` is `true` if a segment represents an
/// extension (denoted with parentheses in options specs in .proto files).
///
/// E.g.,
/// ```norun
/// "foo.(bar.baz).qux" => [ ("foo", false), ("bar.baz", true), ("qux", false) ]
/// ```
pub trait NameParts<'a>: Copy
    + Clone
    + IntoIterator<Item = dyn NamePart<'a>, IntoIter = dyn ExactSizeIterator<Item = dyn NamePart<'a>>>
{
    fn len(&self) -> usize {
        self.into_iter().len()
    }
    fn is_empty(&self) -> bool {
        self.into_iter().is_empty()
    }
    fn formatted_value(&self) -> String {
        self.into_iter()
            .map(|x| x.formatted_value())
            .collect::<Vec<String>>()
            .join(".")
    }
}
impl<'a, T: NameParts<'a>> ToString for T {
    fn to_string(&self) -> String {
        self.value()
    }
}
impl<'a, T: NameParts<'a>> Display for T {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.formatted_value())
    }
}
impl<'a, T: NameParts<'a>> Debug for T {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.formatted_value())
    }
}

pub trait NamePart<'a, I: Impl<'a>> {
    /// the value of the part
    /// E.g. `"foo"`, `"bar.baz"`, or `"qux"` of:
    /// ```no_run
    /// "foo.(bar.baz).qux" => [ ("foo", false), ("bar.baz", true), ("qux", false) ]
    /// ```
    fn value(&self) -> &'a str;
    fn is_extension(&self) -> bool;
    fn as_str(&self) -> &'a str {
        self.value()
    }
    fn formatted_value(&self) -> String {
        if self.is_extension() {
            format!("({})", self.value())
        } else {
            self.value().to_string()
        }
    }
}

impl<'a, T: NamePart<'a>> core::fmt::Debug for T {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.value())
    }
}
impl<'a, T: NamePart<'a>> ToString for T {
    fn to_string(&self) -> String {
        self.value().to_string()
    }
}
impl<'a, T: NamePart<'a>> Display for T {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.part.is_extension {
            write!(f, "({})", self.value())
        } else {
            write!(f, "{}", self.value())
        }
    }
}

impl<'a, T: NamePart<'a>> Eq for T {}

impl<'a, T: NamePart<'a>> PartialEq<T> for T {
    fn eq(&self, other: &Self) -> bool {
        self.part == other.part
    }
}

impl<'a, T: NamePart<'a>> Debug for T {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NamePart")
            .field("value", &self.value())
            .field("is_extension", &self.is_extension())
            .finish()
    }
}

impl<'a, T: NamePart<'a>> PartialEq<String> for T {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}
impl<'a, T: NamePart<'a>> PartialEq<&str> for T {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl<'a, T: NamePart<'a>> PartialEq<T> for str {
    fn eq(&self, other: &dyn NamePart<'a>) -> bool {
        self.as_str() == *other
    }
}

impl<'a, T: NamePart<'a>> Deref for T {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}
