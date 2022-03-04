pub mod format;
mod generic;
pub mod graph;
mod rust;
pub mod typescript;
pub use format::*;
pub use generic::*;
pub use rust::*;

use crate::Ast;

pub trait Util: Sized {
    fn init(&self, ast: Ast<'_, Self>) -> Self;
}
