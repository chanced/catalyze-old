use std::path::PathBuf;

pub struct Context {}

impl Context {
    /// The path where files should be generated to. This path may be relative
    /// or absolute, if it is relative, the path is based off the (unknown)
    /// output destination specified during execution of protoc. If it is
    /// absolute, the path may be outside of the target directory for protoc.
    pub fn output_path(&self) -> PathBuf {
        todo!()
    }
}
