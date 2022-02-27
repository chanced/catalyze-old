pub mod format;
mod generic;
mod rust;
pub mod typescript;
use std::{cell::RefCell, rc::Rc};

pub use format::*;
pub use generic::*;
pub use rust::*;

pub type Util<T> = Rc<RefCell<T>>;
