mod file;
mod generator;
mod language;
mod message;
mod name;
mod node;
mod package;
pub mod visit;
pub use file::File;
pub use generator::*;
pub use language::{Keyword, Language, Rust, TypeScript};
pub use message::Message;
pub use name::Name;
pub use node::*;
pub use package::Package;
pub use visit::*;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
