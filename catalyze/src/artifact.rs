use std::{fs::Permissions, path::PathBuf};

use proc_macro2::TokenStream;

pub enum Op {
    /// Creates a new file. If `overwrite` is `true` , any previous file content
    /// will be overwritten. If `false` and the file exists, this `Artifact`
    /// will be skipped.
    Create { overwrite: bool },
    /// Appends content to the end of the file.
    Append,
    /// Inserts the contents at the given insertion point
    Inject(String),
}

pub enum Content {
    /// A `proc_macro`
    TokenStream(TokenStream),
    /// Writes the given string to the file.
    String(String),
    /// Custom Artifacts are files generated directly against the file system,
    /// and do not use protoc for the generation. Artifacts with `Custom` should
    /// be used if custom permissions need to be set (such as executable scripts
    /// or read-only configs) or when the file needs to be created outside of
    /// the protoc-plugin's generation output directory.
    Custom(Option<Permissions>, Vec<u8>),
}

pub struct Artifact {
    pub path: PathBuf,
    pub content: Content,
    pub op: Op,
    pub tags: Vec<String>,
}
