use chronos_buffer::{buffer::ByteBuf, ConnectionState};
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::handler::PacketHandler;

#[derive(Debug)]
pub struct ClientConnection {
    stream: TcpStream,
    pub state: ConnectionState,  
}

impl ClientConnection {
    pub fn new(stream: TcpStream) -> Self {
        ClientConnection {
            stream,
            state: ConnectionState::default(),
        }
    }

    pub async fn start(&mut self) {
        loop {
            let mut buffer = [0_u8; 1024];
            let read = self.stream.read(&mut buffer).await.unwrap();
            if read == 0 {
                println!("Connection closed");
                break;
            }

            let data = &buffer[..read];
            println!("Data: {:?}", data);

            let mut buffer = ByteBuf::new(data);
            buffer.read_varint();

            let packet_id = buffer.read_varint();
            let packet = chronos_packet::macros::handle_packet(&self.state, *packet_id, &mut buffer);
            if let Some(serialized_packet) = packet {
                println!("[{:?}] Packet: {:?}", self.state, serialized_packet);
                PacketHandler::handle_packet(serialized_packet);
            }
        }
    }
}