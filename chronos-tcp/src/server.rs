use tokio::net::TcpListener;

use crate::client::ClientConnection;

#[derive(Debug)]
pub struct ServerConnection {
    listener: TcpListener,
}

impl ServerConnection {
    pub async fn new(addr: &str, port: u16) -> Self {
        let listener = TcpListener::bind(format!("{}:{}", addr, port)).await.unwrap();
        ServerConnection {
            listener,
        }
    }

    pub async fn start(&mut self) {
        loop {
            let (socket, _) = self.listener.accept().await.unwrap();
            println!("New connection from: {:?}", socket.peer_addr().unwrap());

            tokio::spawn(async {
                let mut connection = ClientConnection::new(socket);
                connection.start().await;
            });
        }
    }
}