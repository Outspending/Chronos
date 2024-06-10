use chronos_tcp::server::ServerConnection;

#[tokio::main]
async fn main() {
    let mut server = ServerConnection::new("127.0.0.1", 25565).await;
    server.start().await;
}
