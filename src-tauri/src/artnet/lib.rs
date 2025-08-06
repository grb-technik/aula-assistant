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