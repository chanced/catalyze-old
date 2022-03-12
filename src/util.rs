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
    // Optionally initializes this `Util`
    fn init<'a>(&self, _ast: Ast<'a, Self>) {}
}
