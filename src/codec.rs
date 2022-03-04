use core::fmt::Debug;
pub trait Encoder: Debug + Send + Sync
where
    Self: Sized,
{
    type Error;
    fn encode(&self) -> Result<&[u8], Self::Error>;
    fn encoded_len(&self) -> usize;
}

pub trait Decoder: Debug + Send + Sync
where
    Self: Sized,
{
    type Error;
    fn decode(&self, buf: &[u8]) -> Result<Self, Self::Error>;
}

impl<T> Encoder for T
where
    T: prost::Message,
{
    type Error = prost::EncodeError;

    fn encode(&self) -> Result<&[u8], Self::Error> {
        todo!()
    }

    fn encoded_len(&self) -> usize {
        todo!()
    }
}
