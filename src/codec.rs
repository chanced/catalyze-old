pub trait Codec {
    type DecodeError;
    fn decode<T>(bytes: &[u8]) -> Result<T, Self::DecodeError>;
}
