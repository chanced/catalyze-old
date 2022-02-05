use heck::{ToLowerCamelCase, ToPascalCase};
use tonic::codegen::ok;

use super::Language;
use crate::{Keyword, Name};
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]

pub struct Rust;

impl Language for Rust {
    fn is_keyword<T: ToString>(&self, s: T) -> anyhow::Result<Keyword> {
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
        let val = name.val.to_string();
        // Use a raw identifier if the identifier matches a Rust keyword:
        // https://doc.rust-lang.org/reference/keywords.html.
        let string = match val.as_str() {
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
            | "async" | "await" | "try" => format!("#r{}",val),
            // the following keywords are not supported as raw identifiers and are therefore suffixed with an underscore.
            "self" | "super" | "extern" | "crate" => format!("{}_", val),
            _ => val,
        };
        Name {
            lang: self.clone(),
            val: string,
        }
    }

    fn to_pascal_case(&self, name: &Name<Self>) -> Name<Self> {
        let mut val = name.val.to_pascal_case();
        // Suffix an underscore for the `Self` Rust keyword as it is not allowed as raw identifier.
        if val == "Self" {
            val += "_";
        }
        Name {
            lang: self.clone(),
            val,
        }
    }
}
