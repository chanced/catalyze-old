use crate::util::{Keyword, Lang};
use anyhow::bail;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Generic;

impl Lang for Generic {
    type Error = anyhow::Error;
    fn is_keyword<T: ToString>(&self, _s: T) -> Result<Keyword, Self::Error> {
        bail!("Language not specified");
    }

    fn name() -> &'static str {
        "not_specified"
    }
}
