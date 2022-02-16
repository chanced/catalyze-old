use crate::util::{Lang, Unspecified};
use crate::WELL_KNNOWN_TYPE_PACKAGE;
pub use heck::{
    AsLowerCamelCase, ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase,
    ToShoutySnakeCase, ToSnakeCase, ToTitleCase, ToUpperCamelCase,
};
use std::hash::{Hash, Hasher};
use std::{fmt, ops::Add};

#[derive(Clone)]
pub struct Name<L> {
    val: String,
    lang: L,
}

impl<L> Hash for Name<L> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}
impl<L> PartialEq for Name<L> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<L> Eq for Name<L> {}

impl Default for Name<Unspecified> {
    fn default() -> Self {
        Self {
            val: Default::default(),
            lang: Unspecified {},
        }
    }
}

impl<L> Name<L> {
    pub fn new(s: &str, lang: L) -> Self {
        Self {
            val: s.to_string(),
            lang,
        }
    }
}
impl<L: Clone> Name<L> {
    /// lang is the specified programming language targeted by the current generator.
    pub fn lang(&self) -> L {
        self.lang.clone()
    }
}

impl<L> Name<L> {
    pub(crate) fn is_well_known_package(&self) -> bool {
        self.val.starts_with(WELL_KNNOWN_TYPE_PACKAGE)
    }
    pub fn as_str(&self) -> &str {
        self.val.as_str()
    }
}

impl<L: Lang> Add<Self> for Name<L> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            val: format!("{}{}", self.val, other.val),
            lang: self.lang.clone(),
        }
    }
}
impl<L: Lang> Add<&str> for Name<L> {
    type Output = Self;

    fn add(self, rhs: &str) -> Self::Output {
        Name::new(&(self.val + rhs), self.lang.clone())
    }
}

impl<L: Clone> Add<String> for Name<L> {
    type Output = Self;
    fn add(self, rhs: String) -> Self::Output {
        Name::new(&(self.val + rhs.as_str()), self.lang.clone())
    }
}

pub trait ToScreamingSnakeCase: ToOwned {
    fn to_screaming_snake_case(&self) -> Self::Owned;
}

pub trait ToScreamingKebabCase: ToOwned {
    fn to_screaming_kebab_case(&self) -> Self::Owned;
}

impl<T: ToShoutySnakeCase> ToScreamingSnakeCase for T {
    fn to_screaming_snake_case(&self) -> Self::Owned {
        self.to_shouty_snake_case()
    }
}
impl<T: ToShoutyKebabCase> ToScreamingKebabCase for T {
    fn to_screaming_kebab_case(&self) -> Self::Owned {
        self.to_shouty_kebab_case()
    }
}

impl<T: ToLowerCamelCase> ToCamelCase for T {
    fn to_camel_case(&self) -> Self::Owned {
        self.to_lower_camel_case()
    }
}

pub trait ToCamelCase: ToOwned {
    fn to_camel_case(&self) -> Self::Owned;
}
impl ToCamelCase for str {
    fn to_camel_case(&self) -> String {
        AsLowerCamelCase(self).to_string()
    }
}

impl<L: Lang> ToKebabCase for Name<L> {
    fn to_kebab_case(&self) -> Self {
        self.lang.to_kebab_case(self)
    }
}
impl<L: Lang> ToSnakeCase for Name<L> {
    fn to_snake_case(&self) -> Self {
        self.lang.to_snake_case(self)
    }
}

impl<L: Lang> ToPascalCase for Name<L> {
    fn to_pascal_case(&self) -> Self {
        self.lang.to_pascal_case(self)
    }
}

impl<L: Lang> ToScreamingSnakeCase for Name<L> {
    fn to_screaming_snake_case(&self) -> Self {
        self.lang.to_screaming_snake_case(self)
    }
}

impl<L> fmt::Debug for Name<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<L: Lang> fmt::Display for Name<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

// impl<L: Language> ToKebabCase for Name<L> {
//     fn to_kebab_case(&self) -> Name<L> {
//         self.lang.to_kebab_case(self.clone())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    #[test]
    fn test_to_kebab() {
        let n = Name::new("hello_world", util::Rust);
        assert_eq!(n.to_kebab_case().to_string(), "hello-world".to_string());
    }
    #[test]
    fn test_to_pascal() {
        let n = Name::new("hello_world", util::Rust);
        assert_eq!(n.to_pascal_case().to_string(), "HelloWorld".to_string());
    }
}
