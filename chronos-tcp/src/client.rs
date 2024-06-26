use chronos_buffer::{buffer::ByteBuf, network::ToNetwork, types::VarInt};
use chronos_packet::{client::ClientInformation, Packet, PacketDirection};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Debug)]
pub struct ClientConnection {
    stream: TcpStream,
    pub info: ClientInformation,
}

impl ClientConnection {
    pub fn new(stream: TcpStream) -> Self {
        ClientConnection {
            stream,
            info: ClientInformation::default(),
        }
    }

    pub async fn start(&mut self) {
        loop {
            let mut buffer = [0_u8; 1024];
            let read = match self.stream.read(&mut buffer).await {
                Ok(n) if n == 0 => {
                    println!("Connection closed");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Failed to read from stream: {:?}", e);
                    break;
                }
            };

            let data = &buffer[..read];
            println!("Data: {:?}", data);

            let mut buffer = ByteBuf::new(data);
            let packet_length = buffer.read_varint();
            let packet_id = buffer.read_varint();

            let state = self.info.state;

            if let Some(serialized_packet) = chronos_packet::v1_20_6::handle_packet(
                &state,
                PacketDirection::Serverbound,
                *packet_id,
                &mut buffer,
            ) {
                println!("[{:?}] Packet: {:?}", state, serialized_packet);
                serialized_packet.handle(&mut self.info);
            } else {
                println!("[{:?}] Unknown Packet ID: {}", state, *packet_id);
            }
        }
    }

    pub async fn send_packet<T: Packet + ToNetwork<T>>(&mut self, packet: &T) {
        if packet.direction() == PacketDirection::Clientbound {
            let mut buffer = ByteBuf::new_empty();
            let id = VarInt::from(packet.id());
            buffer.write_varint(id);
            buffer.write(packet);

            let length = VarInt::from(buffer.size() as i32);
            buffer.write_varint(length);
            buffer.get_mut().rotate_right(*length as usize);

            println!("Buffer: {:?}", buffer.get_ref());
            self.stream
                .write_all(buffer.bytes().as_slice())
                .await
                .unwrap();
        }
    }
}
