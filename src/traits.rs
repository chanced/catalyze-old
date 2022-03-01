pub(crate) trait Upgrade: Sized {
    type Output;
    fn upgrade(self) -> Self::Output;
}

pub(crate) trait Downgrade: Sized {
    type Output;
    fn downgrade(self) -> Self::Output;
}
