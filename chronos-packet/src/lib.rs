use std::fmt::Debug;

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

    pub fn get(name: &str) -> PacketDirection {
        match name.to_lowercase().as_str() {
            "serverbound" => PacketDirection::Serverbound,
            _ => PacketDirection::Clientbound
        }
    }
}

pub trait Packet: Debug {
    fn id(&self) -> i32;
    fn direction(&self) -> PacketDirection;
}