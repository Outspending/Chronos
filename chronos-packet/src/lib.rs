use std::fmt::Debug;

use client::ClientInformation;

pub mod client;
pub mod deserializer;
pub mod macros;
pub mod serializer;
pub mod v1_20_6;

#[derive(Debug, PartialEq, Eq)]
pub enum PacketDirection {
    Clientbound,
    Serverbound,
}

impl PacketDirection {
    pub fn opposite(&self) -> PacketDirection {
        match self {
            PacketDirection::Clientbound => PacketDirection::Serverbound,
            PacketDirection::Serverbound => PacketDirection::Clientbound,
        }
    }

    pub fn get(name: &str) -> PacketDirection {
        match name.to_lowercase().as_str() {
            "serverbound" => PacketDirection::Serverbound,
            _ => PacketDirection::Clientbound,
        }
    }
}

pub trait Packet: Handleable + Debug {
    fn id(&self) -> i32;
    fn direction(&self) -> PacketDirection;
}

pub trait Handleable {
    fn handle(&self, info: &mut ClientInformation);
}
