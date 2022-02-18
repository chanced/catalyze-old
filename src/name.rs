use crate::util::{Generic, Lang};
use crate::WELL_KNNOWN_TYPE_PACKAGE;
pub use heck::{
    AsLowerCamelCase, ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase,
    ToShoutySnakeCase, ToSnakeCase, ToTitleCase, ToUpperCamelCase,
};
use std::hash::{Hash, Hasher};
use std::{fmt, ops::Add};

#[derive(Clone)]
pub struct Name<U> {
    val: String,
    lang: U,
}

impl<U> Hash for Name<U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}
impl<U> PartialEq for Name<U> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<U> Eq for Name<U> {}

impl Default for Name<Generic> {
    fn default() -> Self {
        Self {
            val: Default::default(),
            lang: Generic {},
        }
    }
}

impl<U> Name<U> {
    pub fn new(s: &str, lang: U) -> Self {
        Self {
            val: s.to_string(),
            lang,
        }
    }
}
impl<U: Clone> Name<U> {
    /// lang is the specified programming language targeted by the current generator.
    pub fn lang(&self) -> U {
        self.lang.clone()
    }
}

impl<U> Name<U> {
    pub(crate) fn is_well_known_package(&self) -> bool {
        self.val.starts_with(WELL_KNNOWN_TYPE_PACKAGE)
    }
    pub fn as_str(&self) -> &str {
        self.val.as_str()
    }
}

impl<U: Clone> Add<Self> for Name<U> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            val: format!("{}{}", self.val, other.val),
            lang: self.lang.clone(),
        }
    }
}
impl<U: Clone> Add<&str> for Name<U> {
    type Output = Self;

    fn add(self, rhs: &str) -> Self::Output {
        Name::new(&(self.val + rhs), self.lang.clone())
    }
}

impl<U: Clone> Add<String> for Name<U> {
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

impl<U: Lang> ToKebabCase for Name<U> {
    fn to_kebab_case(&self) -> Self {
        self.lang.to_kebab_case(self)
    }
}
impl<U: Lang> ToSnakeCase for Name<U> {
    fn to_snake_case(&self) -> Self {
        self.lang.to_snake_case(self)
    }
}

impl<U: Lang> ToPascalCase for Name<U> {
    fn to_pascal_case(&self) -> Self {
        self.lang.to_pascal_case(self)
    }
}

impl<U: Lang> ToScreamingSnakeCase for Name<U> {
    fn to_screaming_snake_case(&self) -> Self {
        self.lang.to_screaming_snake_case(self)
    }
}

impl<U> fmt::Debug for Name<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<U> fmt::Display for Name<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

// impl<U: Language> ToKebabCase for Name<U> {
//     fn to_kebab_case(&self) -> Name<U> {
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
