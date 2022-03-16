use std::path::PathBuf;

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

pub enum Artifact {
    Rust {
        path: String,
        contents: TokenStream,
        tags: Vec<String>,
        op: Op,
    },
    Generic {
        path: String,
        tags: Vec<String>,
        contents: String,
        op: Op,
    },
    /// Custom Artifacts are files generated directly against the file system,
    /// and do not use protoc for the generation. These artifacts should be used
    /// over Generic or Rust Artifacts when custom permissions need to be set
    /// (such as executable scripts or read-only configs) or when the file needs
    /// to be created outside of the protoc-plugin's generation output
    /// directory.
    Custom {
        path: String,
        contents: String,
        tags: Vec<String>,
        op: Op,
    },
}
