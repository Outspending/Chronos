pub mod buffer;
pub mod network;
pub mod types;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectionState {
    #[default]
    Handshake = 0,
    Status = 1,
    Login = 2,
    Transfer = 3,
    Configuration = 4,
    Play = 5,
}

impl From<i32> for ConnectionState {
    fn from(value: i32) -> Self {
        match value {
            1 => ConnectionState::Status,
            2 => ConnectionState::Login,
            3 => ConnectionState::Transfer,
            4 => ConnectionState::Configuration,
            5 => ConnectionState::Play,
            _ => ConnectionState::Handshake,
        }
    }
}
