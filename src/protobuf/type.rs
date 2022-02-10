use anyhow::bail;

use super::{Field, Opt, SourceContext, Syntax};

pub struct Type {
    /// The fully qualified message name.
    pub name: String,
    /// The list of fields.
    pub fields: Vec<Field>,
    /// The list of types appearing in `oneof` definitions in this type.
    pub oneofs: Vec<String>,
    /// The protocol buffer options.
    pub options: Vec<Opt>,
    /// The source context.
    pub source_context: Option<SourceContext>,
    /// The source syntax.
    pub syntax: Option<Syntax>,
}

fn main() {
    let t = Type {
        name: "".to_string(),
        fields: vec![],
        oneofs: vec![],
        options: vec![],
        source_context: None,
        syntax: None,
    };
}
