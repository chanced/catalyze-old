mod language;
mod rust;
mod typescript;
mod not_specified;
pub use language::{Keyword, Language};

pub use typescript::TypeScript;

pub use rust::Rust;

pub use not_specified::NotSpecified;