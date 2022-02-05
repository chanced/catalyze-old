use super::Language;
use anyhow::bail;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NotSpecified;

impl Language for NotSpecified {
    fn is_keyword<T: ToString>(&self, s: T) -> anyhow::Result<crate::Keyword> {
        bail!("Language not specified");
    }
}
