mod generic;
mod lang;
mod rust;
mod typescript;
pub use lang::{Keyword, Lang};

pub use typescript::TypeScript;

pub use rust::Rust;

pub use generic::Generic;
