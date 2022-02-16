mod ast;
pub mod container;
mod r#enum;
mod enum_value;
mod extension;
mod field;
mod file;
mod generator;
pub mod iter;
pub mod lang;
mod message;
mod method;
mod name;
mod node;
mod one_of;
mod package;
mod service;
pub mod visit;
mod well_known_type;
pub use ast::*;
pub use enum_value::*;
pub use extension::*;
pub use field::*;
pub use field::*;
pub use file::File;
pub use generator::*;
pub use lang::Lang;
pub use message::*;
pub use method::*;
pub use method::*;
pub use name::Name;
pub use node::*;
pub use one_of::*;
pub use package::Package;
pub use r#enum::*;
pub use service::*;
pub use visit::*;
pub use well_known_type::*;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
