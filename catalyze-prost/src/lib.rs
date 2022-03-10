use catalyze::codec::Codec;

impl Codec for prost::Message {
    type DecodeError = &'static str;

    fn decode<T>(bytes: &[u8]) -> Result<T, Self::DecodeError> {
        println!("hmmm");
        Err("?")
    }
}
