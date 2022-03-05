pub mod format;
mod generic;

mod rust;
pub mod typescript;
pub use format::*;
pub use generic::*;
pub use rust::*;

#[cfg(feature = "message-graph")]
pub mod message_graph;

use crate::Ast;

pub trait Util: Sized {
    fn init(&self, ast: Ast<'_, Self>) -> Self;
}
