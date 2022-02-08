use anyhow::bail;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Label {
    Optional = 1,
    Required = 2,
    Repeated = 3,
}
impl TryFrom<&i32> for Label {
    type Error = anyhow::Error;
    fn try_from(value: &i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Label::Optional),
            2 => Ok(Label::Required),
            3 => Ok(Label::Repeated),
            _ => bail!("invalid value for Label: {}", value),
        }
    }
}
impl TryFrom<i32> for Label {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}
impl TryFrom<&Option<i32>> for Label {
    type Error = anyhow::Error;
    fn try_from(value: &Option<i32>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => Self::try_from(v),
            None => bail!("value is none"),
        }
    }
}
impl TryFrom<Option<i32>> for Label {
    type Error = anyhow::Error;
    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}
