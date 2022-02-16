use heck::ToSnakeCase;
use heck::{ToLowerCamelCase, ToPascalCase};

use crate::util::Keyword;
use crate::util::Lang;
use crate::Name;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]

pub struct Rust;

impl Lang for Rust {
    type Error = anyhow::Error;
    fn name() -> &'static str {
        "rust"
    }
    fn is_keyword<T: ToString>(&self, s: T) -> Result<Keyword, Self::Error> {
        let k = match s.to_string().to_lower_camel_case().as_str() {
            // 2015 strict keywords.
            | "as" | "break" | "const" | "continue" | "else" | "enum" | "false"
            | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut"
            | "pub" | "ref" | "return" | "static" | "struct" | "trait" | "true"
            | "type" | "unsafe" | "use" | "where" | "while"
            // 2018 strict keywords.
            | "dyn"
            // 2015 reserved keywords.
            | "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" | "typeof"
            | "unsized" | "virtual" | "yield"
            // 2018 reserved keywords.
            | "async" | "await" | "try"
            // the following keywords are not supported as raw identifiers and therefore should be suffixed with an underscore.   
            | "self" | "super" | "extern" | "crate" => Keyword::Camel,
            _ => Keyword::None,
        };

        if let Keyword::Camel = k {
            return Ok(k);
        }
        match s.to_string().to_lower_camel_case().as_str() {
            "Self" => Ok(Keyword::Pascal),
            _ => Ok(Keyword::None),
        }
    }
    fn to_snake_case(&self, name: &Name<Self>) -> Name<Self> {
        // Use a raw identifier if the identifier matches a Rust keyword:
        // https://doc.rust-lang.org/reference/keywords.html.
        let name = Name::new(&name.as_str().to_snake_case(), name.lang());
        match name.as_str() {
            // 2015 strict keywords.
            | "as" | "break" | "const" | "continue" | "else" | "enum" | "false"
            | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut"
            | "pub" | "ref" | "return" | "static" | "struct" | "trait" | "true"
            | "type" | "unsafe" | "use" | "where" | "while"
            // 2018 strict keywords.
            | "dyn"
            // 2015 reserved keywords.
            | "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" | "typeof"
            | "unsized" | "virtual" | "yield"
            // 2018 reserved keywords.
            | "async" | "await" | "try" => Name::new(&format!("r#{}",name), name.lang()),
            // the following keywords are not supported as raw identifiers and are therefore suffixed with an underscore.
            "self" | "super" | "extern" | "crate" => Name::new(&format!("{}_", name),name.lang()),
            _ => name,
        }
    }

    fn to_pascal_case(&self, name: &Name<Self>) -> Name<Self> {
        // the language needs to operate on the string value so not to cause a recursive loop.
        let mut val = name.to_string().to_pascal_case();
        // Suffix an underscore for the `Self` Rust keyword as it is not allowed as raw identifier.
        if val == "Self" {
            val += "_";
        }
        Name::new(&val, name.lang())
    }
}
