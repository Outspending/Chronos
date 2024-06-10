use crate::{deserializer::{Deserializer, DeserializerResult}, Packet, PacketDirection};
use chronos_buffer::{buffer::ByteBuf, types::VarInt, ConnectionState};

macro_rules! register_proto {
    {
        $handle_name:ident,
        $(
            $packet_name:ident => ($packet_id:literal, $client_state:ident, $packet_direction:ident),
            $({
                $(
                    $field_name:ident: $field_type:ty
                ),*
            },)?
        )*
    } => {
        $(
            #[derive(Debug, Clone)]
            pub struct $packet_name {
                $($(
                    pub $field_name: $field_type
                ),*)?
            }

            impl Packet for $packet_name {
                fn id(&self) -> i32 {
                    $packet_id
                }

                fn direction(&self) -> PacketDirection {
                    PacketDirection::$packet_direction
                }
            }
        )*

        pub fn $handle_name(state: &ConnectionState, packet_id: i32, buffer: &mut ByteBuf) -> Option<Box<dyn Packet>> {
            match (state, packet_id) {
                $(
                    (ConnectionState::$client_state, $packet_id) => {
                        let serialized = $packet_name::deserialize(buffer).unwrap();
                        Some(Box::new(serialized))
                    },
                    _ => None
                ),*
            }
        }
    };
}

register_proto! {
    handle_packet,

    // Handshaking Packets
    HandshakePacket => (0x00, Handshake, Serverbound), {
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        next_state: ConnectionState
    },
}

impl Deserializer<HandshakePacket> for HandshakePacket {
    fn deserialize(buf: &mut ByteBuf) -> DeserializerResult<HandshakePacket> {
        Ok(HandshakePacket {
            protocol_version: buf.read_varint(),
            server_address: buf.read_string(),
            server_port: buf.read_short(),
            next_state: ConnectionState::from(*buf.read_varint()),
        })
    }
}