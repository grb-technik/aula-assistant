#[derive(Debug)]
pub enum ArtNetError {
    FailedToBindSocket(std::io::Error),
    FailedToSendData(std::io::Error),
}

impl std::fmt::Display for ArtNetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtNetError::FailedToBindSocket(e) => write!(f, "failed to bind socket: {}", e),
            ArtNetError::FailedToSendData(e) => write!(f, "failed to send data: {}", e),
        }
    }
}

impl std::error::Error for ArtNetError {}
