pub use heck::{
    AsLowerCamelCase, ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase,
    ToShoutySnakeCase, ToSnakeCase, ToTitleCase, ToUpperCamelCase,
};

use crate::language::{Language, NotSpecified};
use std::fmt;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Name<L: Language = NotSpecified> {
    pub(crate) val: String,
    lang: L,
}
impl<L: Language> Name<L> {
    pub(crate) fn new(s: &str, lang: L) -> Self {
        Self {
            val: s.to_string(),
            lang,
        }
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

impl ToKebabCase for Name {
    fn to_kebab_case(&self) -> Self {
        self.lang.to_kebab_case(self)
    }
}
impl ToSnakeCase for Name {
    fn to_snake_case(&self) -> Self {
        self.lang.to_snake_case(self)
    }
}

impl ToScreamingSnakeCase for Name {
    fn to_screaming_snake_case(&self) -> Self {
        self.lang.to_screaming_snake_case(self)
    }
}

impl<L: Language> fmt::Debug for Name<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
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

impl<L: Language> fmt::Display for Name<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::new();
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
}
