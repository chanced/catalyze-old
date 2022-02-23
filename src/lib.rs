mod ast;
pub mod container;
mod r#enum;
mod enum_value;
mod extension;
mod field;
mod file;
mod generator;
pub mod graph;
pub mod iter;
mod message;
mod method;
mod name;
mod node;
mod oneof;
mod package;
pub mod path;
pub mod proto;
mod service;
mod source;
pub mod util;
pub mod visit;

mod well_known_type;
pub use ast::*;
pub use enum_value::*;
pub use extension::*;
pub use field::*;
pub use field::*;
pub use file::File;
pub use generator::*;
pub use message::*;
pub use method::*;
pub use method::*;
pub use name::Name;
pub use node::*;
pub use oneof::*;
pub use package::Package;

pub use r#enum::*;
pub use service::*;
pub use source::*;
pub use util::ToCase;

pub use well_known_type::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
