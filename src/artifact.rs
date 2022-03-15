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
    /// Custom files will always be written to disk rather than being sent to `protoc`.
    ///
    /// The path should be relative and must not start with `"."`, `".."`, or `"/"`.
    Custom {
        path: String,
        contents: String,
        tags: Vec<String>,
        op: Op,
    },
}
