use chronos_packet::Packet;

pub struct PacketHandler;

impl PacketHandler {
    pub fn handle_packet(packet: Box<dyn Packet>) {
        match packet {
            HandshakePacket => {
                println!("Handshake packet")
            },
            _ => ()
        }
    }
}