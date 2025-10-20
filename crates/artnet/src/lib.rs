use crate::error::ArtNetError;
use std::net::{SocketAddr, UdpSocket};

pub mod error;

#[cfg(test)]
mod test;

pub fn create_artnet_socket(bind_addr: String, broadcast: bool) -> Result<UdpSocket, ArtNetError> {
    let socket = UdpSocket::bind(bind_addr).map_err(ArtNetError::FailedToBindSocket)?;
    socket
        .set_broadcast(broadcast)
        .map_err(ArtNetError::FailedToBindSocket)?;

    Ok(socket)
}

pub fn send_artnet_package(
    socket: &UdpSocket,
    target_addr: &SocketAddr,
    package: &[u8; 530],
) -> Result<(), ArtNetError> {
    socket
        .send_to(package, target_addr)
        .map_err(ArtNetError::FailedToSendData)?;
    Ok(())
}

/// see also https://art-net.org.uk/downloads/art-net.pdf
pub fn build_artnet_package(universe: &u16, data: &[u8; 512]) -> [u8; 530] {
    const ARTNET_NAME: &[u8; 8] = b"Art-Net\0";
    const ARTNET_VERSION: u8 = 14;
    const ARTNET_OPCODE: u8 = 80;

    let h_uni = (universe >> 8) as u8;
    let l_uni = (universe & 0xff) as u8;
    let h_len = (data.len() >> 8) as u8;
    let l_len = (data.len() & 0xff) as u8;

    let package: [u8; 530] = std::array::from_fn(|i| match i {
        0..=7 => ARTNET_NAME[i],
        8 => 0,
        9 => ARTNET_OPCODE,
        10 => 0,
        11 => ARTNET_VERSION,
        12 => 0,
        13 => 0,
        14 => l_uni,
        15 => h_uni,
        16 => h_len,
        17 => l_len,
        18..=529 => data[i - 18],
        _ => unreachable!(),
    });

    package
}
