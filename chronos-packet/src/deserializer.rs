use chronos_buffer::buffer::ByteBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeserializerError {
    #[error("Invalid packet ID")]
    InvalidPacketID,
    #[error("Invalid packet format")]
    InvalidPacketFormat,
    #[error("Invalid packet data")]
    Unknown
}

pub type DeserializerResult<T> = Result<T, DeserializerError>;

pub trait Deserializer<T> {
    fn deserialize(buf: &mut ByteBuf) -> DeserializerResult<T>;
}