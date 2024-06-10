use chronos_buffer::buffer::ByteBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerializerError {
    #[error("Invalid packet ID")]
    InvalidPacket,
    #[error("Invalid packet format")]
    Unknown
}

pub type SerializerResult<T> = Result<T, SerializerError>;

pub trait Serializer {
    fn serialize(&self, buf: &mut ByteBuf) -> SerializerResult<()>;
}