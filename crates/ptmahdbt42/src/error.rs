#[derive(Debug)]
pub enum PTMAHDBT42Error {
    FailedToBindSocket(std::io::Error),
    FailedToSendRequest(std::io::Error),
    FailedToReadResponse(std::io::Error),
    FailedToShutdownStream(std::io::Error),
    FailedToSetReadTimeout(std::io::Error),
    FailedToConvertResponseToUtf8(std::string::FromUtf8Error),
    InvalidResponse,
}

impl std::fmt::Display for PTMAHDBT42Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PTMAHDBT42Error::FailedToBindSocket(e) => write!(f, "failed to bind socket: {}", e),
            PTMAHDBT42Error::FailedToSendRequest(e) => write!(f, "failed to send request: {}", e),
            PTMAHDBT42Error::FailedToReadResponse(e) => write!(f, "failed to read response: {}", e),
            PTMAHDBT42Error::FailedToShutdownStream(e) => {
                write!(f, "failed to shutdown stream: {}", e)
            }
            PTMAHDBT42Error::FailedToSetReadTimeout(e) => {
                write!(f, "failed to set read timeout: {}", e)
            }
            PTMAHDBT42Error::FailedToConvertResponseToUtf8(e) => {
                write!(f, "failed to convert response to UTF-8: {}", e)
            }
            PTMAHDBT42Error::InvalidResponse => write!(f, "invalid response"),
        }
    }
}

impl std::error::Error for PTMAHDBT42Error {}
