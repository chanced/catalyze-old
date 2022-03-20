use std::path::{Path, PathBuf};

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
}

impl Context {
    /// The path where files should be generated to. This path may be relative
    /// or absolute, if it is relative, the path is based off the (unknown)
    /// output destination specified during execution of protoc. If it is
    /// absolute, the path may be outside of the target directory for protoc.
    pub fn output_path(&self) -> PathBuf {
        self.path.clone()
    }
    /// Changes the BuildContext's OutputPath to dir. If dir is relative,
    // it is applied relative to the current value of OutputPath.
    pub fn push_dir(&self, dir: impl AsRef<Path>) -> Context {
        Context {
            path: self.path.join(dir),
            parent: Some(Box::new(self.clone())),
        }
    }
    pub fn pop_dir(self) -> Context {
        if let Some(parent) = self.parent {
            *parent
        } else {
            self
        }
    }
}

// type BuildContext interface {
// 	DebuggerCommon

// 	// OutputPath is the path where files should be generated to. This path may
// 	// be relative or absolute, if it is relative, the path is based off the
// 	// (unknown) output destination specified during execution of protoc. If it
// 	// is absolute, the path may be outside of the target directory for protoc.
// 	OutputPath() string

// 	// JoinPath returns name relative to the value of OutputPath.
// 	JoinPath(name ...string) string

// 	// Push adds an arbitrary prefix to the Debugger output. The Outpath value is
// 	// unchanged.
// 	Push(prefix string) BuildContext

// 	// PushDir changes the BuildContext's OutputPath to dir. If dir is relative,
// 	// it is applied relative to the current value of OutputPath.
// 	PushDir(dir string) BuildContext

// 	// Pop returns the previous state of the BuildContext. This may or may not
// 	// change the value of OutputPath. This method will cause the plugin to fail
// 	// if the root context is popped.
// 	Pop() BuildContext

// 	// PopDir behaves like Pop but returns the last previous state of OutputPath,
// 	// skipping over any prefix changes in-between. If at the root context, this
// 	// method will always return the root context.
// 	PopDir() BuildContext

// 	// Parameters returns the command line parameters passed in from protoc,
// 	// mutated with any provided ParamMutators via InitOptions.
// 	Parameters() Parameters
// }
