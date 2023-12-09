use std::path::{Path, PathBuf};

use crate::Parameters;

const OUTPUT_PATH_KEY: &str = "output_path";

// Tracks code generation relative to an output path. By default,
// BuildContext's path is relative to the output location specified when
// executing protoc (an absolute path to this location is not available within
// protoc plugins). Specifying a custom output path permits using an absolute
// path and or a different location from protoc's designated output location.
#[derive(Debug, Clone)]
pub struct Context {
    path: PathBuf,
    parent: Option<Box<Context>>,
    params: Option<Parameters>,
}

impl Context {
    pub fn new(path: PathBuf, params: Parameters) -> Self {
        Self {
            path,
            parent: None,
            params: Some(params),
        }
    }
    /// The path where files should be generated to. This path may be relative
    /// or absolute, if it is relative, the path is based off the (unknown)
    /// output destination specified during execution of protoc. If it is
    /// absolute, the path may be outside of the target directory for protoc.
    pub fn output_path(&self) -> &Path {
        &self.path
    }

    pub fn parameters(&self) -> Parameters {
        if let Some(parent) = self.parent.clone() {
            parent.parameters()
        } else {
            self.params.clone().expect("Parameters was not set")
        }
    }
}
