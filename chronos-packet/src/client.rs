use chronos_buffer::ConnectionState;

#[derive(Debug, Default)]
pub struct ClientInformation {
    pub state: ConnectionState,
}
