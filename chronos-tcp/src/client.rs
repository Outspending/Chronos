use chronos_buffer::{buffer::ByteBuf, ConnectionState};
use chronos_packet::client::ClientInformation;
use tokio::{io::AsyncReadExt, net::TcpStream};

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
            buffer.read_varint();

            let packet_id = buffer.read_varint();

            let state = self.info.state;
            
            if let Some(serialized_packet) = chronos_packet::v1_20_6::handle_packet(&state, *packet_id, &mut buffer) {
                println!("[{:?}] Packet: {:?}", state, serialized_packet);
                serialized_packet.handle(&mut self.info);
            }
        }
    }

}