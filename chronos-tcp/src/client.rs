use chronos_buffer::{buffer::ByteBuf, ConnectionState};
use tokio::{io::AsyncReadExt, net::TcpStream};

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
            println!("Packet ID: {:?}", packet_id);
        }
    }
}