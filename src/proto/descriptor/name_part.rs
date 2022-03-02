use std::{cell::RefCell, marker::PhantomData, ops::Index, rc::Rc, vec};

/// The name of the uninterpreted option.  Each string represents a segment in
/// a dot-separated name. `is_extension` is `true` if a segment represents an
/// extension (denoted with parentheses in options specs in .proto files).
///
/// E.g.,
/// ```
/// "foo.(bar.baz).qux" => [ ("foo", false), ("bar.baz", true), ("qux", false) ]
/// ```
#[derive(Debug, Copy)]
pub struct NamePart<'a, U> {
    part: &'a prost_types::uninterpreted_option::NamePart,
    u: PhantomData<U>,
}
impl<'a, U> Clone for NamePart<'a, U> {
    fn clone(&self) -> Self {
        Self {
            u: PhantomData,
            part: self.part,
        }
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

impl<'a, U> NamePart<'a, U> {
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
    u: PhantomData<U>,
}

impl<'a, U> From<&'a [prost_types::uninterpreted_option::NamePart]> for NameParts<'a, U> {
    fn from(prost_parts: &'a [prost_types::uninterpreted_option::NamePart]) -> Self {
        let parts = prost_parts
            .iter()
            .map(|part| NamePart::from(part))
            .collect();
        Self {
            prost_parts,
            parts,
            u: PhantomData,
        }
    }
}
impl<'a, U> From<&'a std::vec::Vec<prost_types::uninterpreted_option::NamePart>>
    for NameParts<'a, U>
{
    fn from(prost_parts: &'a std::vec::Vec<prost_types::uninterpreted_option::NamePart>) -> Self {
        let parts = prost_parts
            .iter()
            .map(|part| NamePart::from(part))
            .collect();
        Self {
            prost_parts,
            parts,
            u: PhantomData,
        }
    }
}

impl<'a, U> std::iter::IntoIterator for &NameParts<'a, U> {
    type Item = NamePart<'a, U>;
    type IntoIter = vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.parts.clone().into_iter()
    }
}

impl<'a, U> NameParts<'a, U> {
    fn iter(&self) -> std::slice::Iter<NamePart<'a, U>> {
        self.parts.iter()
    }
}

impl<'a, U> Clone for NameParts<'a, U> {
    fn clone(&self) -> Self {
        Self {
            parts: self.parts.clone(),
            prost_parts: self.prost_parts,
            u: PhantomData,
        }
    }
}

impl<'a, U> Index<usize> for NameParts<'a, U> {
    type Output = NamePart<'a, U>;
    fn index(&self, i: usize) -> &Self::Output {
        &self.parts[i]
    }
}

// #[derive(Debug, Clone)]
// pub struct NamePartsIter<'a, U> {
//     parts: &'a [NamePart<'a, U>],
//     i: usize,
// }

// impl<'a, U> Iterator for NamePartsIter<'a, U> {
//     type Item = NamePart<'a, U>;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.parts.get(self.i).map(|p| {
//             self.i += 1;
//             p.clone()
//         })
//     }
// }
