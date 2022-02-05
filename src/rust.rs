#[derive(Copy, PartialEq, Eq, Hash)]
pub struct Rust;

impl Display for Rust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rust")
    }
}

impl Language for Rust {
    fn to_snake_case(&self, name: Name<Self>) -> Name<Self> {
        let val = name.to_snake_case().to_string();
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
            lang: self.copy(),
            string,
        }
    }

    fn to_pascal_case(&self, name: Name<Self>) -> Name<Self> {
        // fn to_rust_upper_camel_case(&self) -> Self::Owned {
        //     let mut ident = self.to_upper_camel_case();
        //     // Suffix an underscore for the `Self` Rust keyword as it is not allowed as raw identifier.
        //     if ident == "Self" {
        //         ident += "_";
        //     }
        //     ident
        // }

        todo!()
    }
}
