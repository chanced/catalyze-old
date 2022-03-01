use std::ops::Index;

use crate::util::Util;

/// The name of the uninterpreted option.  Each string represents a segment in
/// a dot-separated name. `is_extension` is `true` if a segment represents an
/// extension (denoted with parentheses in options specs in .proto files).
///
/// E.g.,
/// ```
/// "foo.(bar.baz).qux" => [ ("foo", false), ("bar.baz", true), ("qux", false) ]
/// ```
#[derive(Debug)]
pub struct NamePart<'a, U> {
    part: &'a prost_types::uninterpreted_option::NamePart,
    pub util: Util<U>,
}
impl<'a, U> Clone for NamePart<'a, U> {
    fn clone(&self) -> Self {
        Self {
            util: self.util.clone(),
            part: self.part,
        }
    }
}

impl<'a, U> NamePart<'a, U> {
    pub fn new(part: &'a prost_types::uninterpreted_option::NamePart, util: Util<U>) -> Self {
        Self { part, util }
    }
    /// alias for value
    /// the value of the part `NamePart`
    pub fn name_part(&self) -> &'a str {
        &self.part.name_part
    }
    /// the value of the part
    /// E.g. `"foo"`, `"bar.baz"`, or `"qux"` of:
    /// ```
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

    pub fn util(&self) -> Util<U> {
        return self.util.clone();
    }
}

impl<'a, U> ToString for NamePart<'a, U> {
    fn to_string(&self) -> String {
        if self.part.is_extension {
            format!("({})", self.name_part())
        } else {
            self.part.name_part.to_string()
        }
    }
}

#[derive(Debug)]
pub struct NameParts<'a, U> {
    prost_parts: &'a [prost_types::uninterpreted_option::NamePart],
    parts: Vec<NamePart<'a, U>>,
    util: Util<U>,
}
impl<'a, U> NameParts<'a, U> {
    pub fn new(
        prost_parts: &'a [prost_types::uninterpreted_option::NamePart],
        util: Util<U>,
    ) -> Self {
        let parts = prost_parts
            .iter()
            .map(|part| NamePart::new(part, util.clone()))
            .collect();
        Self {
            prost_parts,
            parts,
            util,
        }
    }
}
impl<'a, U> Clone for NameParts<'a, U> {
    fn clone(&self) -> Self {
        Self {
            parts: self.parts.clone(),
            util: self.util.clone(),
            prost_parts: self.prost_parts,
        }
    }
}

impl<'a, U> Index<usize> for NameParts<'a, U> {
    type Output = NamePart<'a, U>;
    fn index(&self, i: usize) -> &Self::Output {
        &self.parts[i]
    }
}

#[derive(Debug, Clone)]
pub struct NamePartsIter<'a, U> {
    parts: &'a [prost_types::uninterpreted_option::NamePart],
    util: Util<U>,
    i: usize,
}

impl<'a, U> Iterator for NamePartsIter<'a, U> {
    type Item = NamePart<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.parts.len() {
            let part = &self.parts[self.i];
            self.i += 1;
            Some(NamePart {
                part,
                util: self.util.clone(),
            })
        } else {
            None
        }
    }
}
