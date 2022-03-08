pub mod format;
mod generic;

mod rust;
pub mod typescript;

use std::error::Error;

pub use format::*;
pub use generic::*;
pub use rust::*;

#[cfg(feature = "message-graph")]
pub mod message_graph;

use crate::Ast;

pub trait Util: Sized {
    type Error: Error + Send + Sync + 'static;
    fn init<'a>(&self, ast: &Ast<'a, Self>) -> Result<Self, Self::Error>;
}
