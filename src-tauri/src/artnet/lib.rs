use std::net::UdpSocket;

#[derive(Debug)]
pub(crate) enum ArtNetError {
    FailedToBindSocket(std::io::Error),
    FailedToSendData(std::io::Error),
}

impl std::fmt::Display for ArtNetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtNetError::FailedToBindSocket(e) => write!(f, "Failed to bind socket: {}", e),
            ArtNetError::FailedToSendData(e) => write!(f, "Failed to send data: {}", e),
        }
    }
}

impl std::error::Error for ArtNetError {}

/// a function to send the artnet packages
pub(crate) fn create_artnet_socket(
    bind_addr: String,
    target_addr: String,
    broadcast: bool,
) -> Result<impl FnOnce(&Vec<u8>) -> Result<(), ArtNetError>, ArtNetError> {
    let socket = UdpSocket::bind(bind_addr).map_err(ArtNetError::FailedToBindSocket)?;
    socket
        .set_broadcast(broadcast)
        .map_err(ArtNetError::FailedToBindSocket)?;

    let send = move |data: &Vec<u8>| -> Result<(), ArtNetError> {
        socket
            .send_to(data, target_addr)
            .map_err(ArtNetError::FailedToSendData)?;
        Ok(())
    };

    Ok(send)
}

/// see also https://art-net.org.uk/downloads/art-net.pdf
/// always sends 512 bytes of data
pub fn build_artnet_package(universe: &u16, data: &[u8; 512]) -> Vec<u8> {
    // would be needed if we wanted to support variable data length
    // let mut data_length = data.len();
    // data_length += data_length % 2;

    const ARTNET_NAME: &[u8; 8] = b"Art-Net\0";
    const ARTNET_VERSION: u8 = 14;
    const ARTNET_OPCODE: u8 = 80;
    const ARTNET_HEADER_SIZE: usize = 18;
    const FIXED_DATA_LENGTH: usize = 512;

    let h_uni = (universe >> 8) as u8;
    let l_uni = (universe & 0xff) as u8;
    let h_len = (FIXED_DATA_LENGTH >> 8) as u8;
    let l_len = (FIXED_DATA_LENGTH & 0xff) as u8;

    let mut package = Vec::with_capacity(ARTNET_HEADER_SIZE + FIXED_DATA_LENGTH);

    package.extend_from_slice(ARTNET_NAME);
    package.extend_from_slice(&[0, ARTNET_OPCODE]);
    package.extend_from_slice(&[0, ARTNET_VERSION]);
    package.extend_from_slice(&[0, 0]);
    package.extend_from_slice(&[l_uni, h_uni, h_len, l_len]);
    package.extend_from_slice(&data[..FIXED_DATA_LENGTH.min(data.len())]);

    package
}
