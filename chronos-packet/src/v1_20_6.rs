use chronos_buffer::{buffer::ByteBuf, types::VarInt, ConnectionState};
use uuid::Uuid;

use crate::{client::ClientInformation, deserializer::{Deserializer, DeserializerResult}, register_proto, Handleable, Packet, PacketDirection};

register_proto! {
    handle_packet,

    // Handshaking Packets
    HandshakePacket => (0x00, Handshake, Serverbound), {
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        next_state: ConnectionState
    },

    // Status Packets
    StatusRequestPacket => (0x00, Status, Serverbound),
    
    // Login Packets
    LoginStartPacket => (0x00, Login, Serverbound), {
        username: String,
        uuid: Uuid
    },
}

impl Handleable for HandshakePacket {
    fn handle(&self, info: &mut ClientInformation) {
        info.state = self.next_state;
    }
}

impl Handleable for LoginStartPacket {
    fn handle(&self, _info: &mut ClientInformation) {}
}

impl Handleable for StatusRequestPacket {
    fn handle(&self, _info: &mut ClientInformation) {}
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

impl Deserializer<LoginStartPacket> for LoginStartPacket {
    fn deserialize(buf: &mut ByteBuf) -> DeserializerResult<LoginStartPacket> {
        Ok(LoginStartPacket {
            username: buf.read_string(),
            uuid: buf.read_uuid(),
        })
    }
}

impl Deserializer<StatusRequestPacket> for StatusRequestPacket {
    fn deserialize(buf: &mut ByteBuf) -> DeserializerResult<StatusRequestPacket> {
        Ok(StatusRequestPacket {})
    }
}