use deserializer::Deserializer;
use serializer::Serializer;

pub mod deserializer;
pub mod serializer;
pub mod macros;

pub enum PacketDirection {
    Clientbound,
    Serverbound
}

impl PacketDirection {
    pub fn opposite(&self) -> PacketDirection {
        match self {
            PacketDirection::Clientbound => PacketDirection::Serverbound,
            PacketDirection::Serverbound => PacketDirection::Clientbound
        }
    }
}

pub trait Packet<T> {
    fn id(&self) -> i32;
}

pub trait ClientboundPacket<T>: Packet<T> + Serializer {}
pub trait ServerboundPacket<T>: Packet<T> + Deserializer<T> {}