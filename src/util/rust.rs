use crate::util::ToCase;
use crate::Name;
use heck::ToLowerCamelCase;
use heck::ToPascalCase;

use super::format::IsKeyword;
use super::KeywordCase;

impl ToCase for Rust {
    fn to_snake_case(&self, name: &Name<Self>) -> Name<Self> {
        // https://doc.rust-lang.org/reference/keywords.html.
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
                    | "async" | "await" | "try" => name.assign(&format!("r#{}",name)),
                    // the following keywords are not supported as raw identifiers and are therefore suffixed with an underscore.
                    "self" | "super" | "extern" | "crate" => name.assign(&format!("r#{}_",name)),
                    _ => name.clone(),
                }
    }

    fn to_pascal_case(&self, name: &Name<Self>) -> Name<Self> {
        // the language needs to operate on the string value so not to cause a recursive loop.
        let mut val = name.as_str().to_pascal_case();
        // Suffix an underscore for the `Self` Rust keyword as it is not allowed as raw identifier.
        if val == "Self" {
            val += "_";
        }
        Name::new(&val, name.util.clone())
    }
}

#[derive(Clone)]
pub struct Rust {}

impl IsKeyword for Rust {
    fn is_keyword(&self, s: &str) -> Option<Vec<KeywordCase>> {
        let mut cap = 0;
        let lcc = match s.to_lower_camel_case().as_str() {
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
            | "self" | "super" | "extern" | "crate" => {
                cap += 1;
                Some(KeywordCase::Camel)
            },
            _ => None,
        };
        let pc = match s.to_pascal_case().as_str() {
            "Self" => {
                cap += 1;
                Some(KeywordCase::Pascal)
            }
            _ => None,
        };
        match cap {
            0 => None,
            1 => Some(vec![lcc.unwrap_or_else(|| pc.unwrap())]),
            2 => Some(vec![lcc.unwrap(), pc.unwrap()]),
            _ => unreachable!(),
        }
    }
}
