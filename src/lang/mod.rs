mod lang;
mod unspecified;
mod rust;
mod typescript;
pub use lang::{Keyword, Lang};

pub use typescript::TypeScript;

pub use rust::Rust;

pub use unspecified::Unspecified;
