//TODO: Fix this, impl NamePart trait

#[derive(Debug, Copy)]
pub struct NameParts<'a> {
    parts: &'a [prost_types::uninterpreted_option::NamePart],
    u: PhantomData<U>,
}

impl<'a> ToString for NameParts<'a> {
    fn to_string(&self) -> String {
        self.formatted_value()
    }
}

// impl<'a>From<&'a [prost_types::uninterpreted_option::NamePart]> for NameParts<'a>{
//     fn from(prost_parts: &'a [prost_types::uninterpreted_option::NamePart]) -> Self {
//     }
// }

impl<'a> From<&'a std::vec::Vec<prost_types::uninterpreted_option::NamePart>> for NameParts<'a, U> {
    fn from(prost_parts: &'a std::vec::Vec<prost_types::uninterpreted_option::NamePart>) -> Self {
        Self {
            parts: prost_parts,
            u: PhantomData,
        }
    }
}

impl<'a> std::iter::IntoIterator for &NameParts<'a> {
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

impl<'a> NameParts<'a> {
    pub fn iter(&self) -> NamePartIter<'a> {
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

impl<'a> Clone for NameParts<'a> {
    fn clone(&self) -> Self {
        Self {
            parts: self.parts,
            u: PhantomData,
        }
    }
}

pub struct NamePartIter<'a> {
    iter: std::slice::Iter<'a, prost_types::uninterpreted_option::NamePart>,
    u: PhantomData<U>,
}
impl<'a> Iterator for NamePartIter<'a> {
    type Item = NamePart<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(NamePart::from)
    }
}

impl<'a> From<&NameParts<'a, U>> for NamePartIter<'a> {
    fn from(parts: &NameParts<'a, U>) -> Self {
        Self {
            iter: parts.parts.iter(),
            u: PhantomData,
        }
    }
}
impl<'a> From<&'a [prost_types::uninterpreted_option::NamePart]> for NamePartIter<'a> {
    fn from(parts: &'a [prost_types::uninterpreted_option::NamePart]) -> Self {
        Self {
            iter: parts.iter(),
            u: PhantomData,
        }
    }
}
impl<'a> From<&'a Vec<prost_types::uninterpreted_option::NamePart>> for NamePartIter<'a> {
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
