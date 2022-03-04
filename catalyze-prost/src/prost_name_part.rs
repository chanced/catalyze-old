use std::{
    fmt::{self, Debug, Display},
    marker::PhantomData,
    ops::Deref,
    vec,
};

/// The name of the uninterpreted option.  Each string represents a segment in
/// a dot-separated name. `is_extension` is `true` if a segment represents an
/// extension (denoted with parentheses in options specs in .proto files).
///
/// E.g.,
/// ```norun
/// "foo.(bar.baz).qux" => [ ("foo", false), ("bar.baz", true), ("qux", false) ]
/// ```
#[derive(Copy)]
pub struct NamePart<'a, U> {
    part: &'a prost_types::uninterpreted_option::NamePart,
    u: PhantomData<U>,
}
impl<'a, U> Eq for NamePart<'a, U> {}
impl<'a, U> PartialEq<NamePart<'a, U>> for NamePart<'a, U> {
    fn eq(&self, other: &Self) -> bool {
        self.part == other.part
    }
}

impl<'a, U> Debug for NamePart<'a, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NamePart")
            .field("value", &self.part.name_part)
            .field("is_extension", &self.part.is_extension)
            .finish()
    }
}

impl<'a, U> PartialEq<String> for NamePart<'a, U> {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}
impl<'a, U> Display for NamePart<'a, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl<'a, U> PartialEq<&str> for NamePart<'a, U> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<'a, U> NamePart<'a, U> {
    /// alias for value
    /// the value of the part `NamePart`
    pub fn name_part(&self) -> &'a str {
        &self.part.name_part
    }
    /// the value of the part
    /// E.g. `"foo"`, `"bar.baz"`, or `"qux"` of:
    /// ```no_run
    /// "foo.(bar.baz).qux" => [ ("foo", false), ("bar.baz", true), ("qux", false) ]
    /// ```
    pub fn value(&self) -> &'a str {
        self.name_part()
    }
    /// is_extension is true if the segment represents an extension (denoted
    /// with parentheses in options specs in .proto files).
    pub fn is_extension(&self) -> bool {
        self.part.is_extension
    }
    pub fn formatted_value(&self) -> String {
        if self.part.is_extension {
            format!("({})", self.part.name_part)
        } else {
            self.part.name_part.to_string()
        }
    }
}
impl<'a, U> Clone for NamePart<'a, U> {
    fn clone(&self) -> Self {
        Self {
            u: PhantomData,
            part: self.part,
        }
    }
}

impl<'a, U> NamePart<'a, U> {
    pub fn as_str(&self) -> &str {
        &self.part.name_part
    }
}

impl<'a, U> Deref for NamePart<'a, U> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.part.name_part
    }
}

impl<'a, U> From<&'a prost_types::uninterpreted_option::NamePart> for NamePart<'a, U> {
    fn from(part: &'a prost_types::uninterpreted_option::NamePart) -> Self {
        Self {
            u: PhantomData,
            part,
        }
    }
}

#[derive(Debug, Copy)]
pub struct NameParts<'a, U> {
    parts: &'a [prost_types::uninterpreted_option::NamePart],
    u: PhantomData<U>,
}

impl<'a, U> ToString for NameParts<'a, U> {
    fn to_string(&self) -> String {
        self.formatted_value()
    }
}

// impl<'a, U> From<&'a [prost_types::uninterpreted_option::NamePart]> for NameParts<'a, U> {
//     fn from(prost_parts: &'a [prost_types::uninterpreted_option::NamePart]) -> Self {
//     }
// }

impl<'a, U> From<&'a std::vec::Vec<prost_types::uninterpreted_option::NamePart>>
    for NameParts<'a, U>
{
    fn from(prost_parts: &'a std::vec::Vec<prost_types::uninterpreted_option::NamePart>) -> Self {
        Self {
            parts: prost_parts,
            u: PhantomData,
        }
    }
}

impl<'a, U> std::iter::IntoIterator for &NameParts<'a, U> {
    type Item = NamePart<'a, U>;
    type IntoIter = vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.parts
            .iter()
            .map(NamePart::from)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<'a, U> NameParts<'a, U> {
    pub fn iter(&self) -> NamePartIter<'a, U> {
        self.into()
    }
    pub fn get(&self, idx: usize) -> Option<NamePart<'a, U>> {
        self.parts.get(idx).map(NamePart::from)
    }

    pub fn len(&self) -> usize {
        self.parts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
    pub fn contains(&self, part: &str) -> bool {
        self.parts.iter().any(|p| p.name_part == part)
    }
    pub fn formatted_value(&self) -> String {
        self.iter()
            .map(|part| part.formatted_value())
            .collect::<Vec<_>>()
            .join(".")
    }
}

impl<'a, U> Clone for NameParts<'a, U> {
    fn clone(&self) -> Self {
        Self {
            parts: self.parts,
            u: PhantomData,
        }
    }
}

pub struct NamePartIter<'a, U> {
    iter: std::slice::Iter<'a, prost_types::uninterpreted_option::NamePart>,
    u: PhantomData<U>,
}
impl<'a, U> Iterator for NamePartIter<'a, U> {
    type Item = NamePart<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(NamePart::from)
    }
}

impl<'a, U> From<&NameParts<'a, U>> for NamePartIter<'a, U> {
    fn from(parts: &NameParts<'a, U>) -> Self {
        Self {
            iter: parts.parts.iter(),
            u: PhantomData,
        }
    }
}
impl<'a, U> From<&'a [prost_types::uninterpreted_option::NamePart]> for NamePartIter<'a, U> {
    fn from(parts: &'a [prost_types::uninterpreted_option::NamePart]) -> Self {
        Self {
            iter: parts.iter(),
            u: PhantomData,
        }
    }
}
impl<'a, U> From<&'a Vec<prost_types::uninterpreted_option::NamePart>> for NamePartIter<'a, U> {
    fn from(parts: &'a Vec<prost_types::uninterpreted_option::NamePart>) -> Self {
        Self {
            iter: parts.iter(),
            u: PhantomData,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::util::Generic;

    use super::*;

    #[test]
    fn test_to_string() {
        let p1 = prost_types::uninterpreted_option::NamePart {
            name_part: "foo".to_string(),
            is_extension: false,
        };
        let p2 = prost_types::uninterpreted_option::NamePart {
            name_part: "bar".to_string(),
            is_extension: true,
        };
        let p3 = prost_types::uninterpreted_option::NamePart {
            name_part: "baz".to_string(),
            is_extension: false,
        };
        let parts = vec![p1, p2, p3];
        let name_parts: NameParts<'_, Generic> = NameParts::from(&parts);
        assert_eq!(name_parts.to_string(), "foo.(bar).baz");
        assert_eq!(name_parts.get(0).unwrap(), "foo")
    }
}
