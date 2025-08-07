use crate::artnet::build_artnet_package;

#[test]
fn test_build_artnet_package() {
    const ARTNET_NAME: [u8; 8] = *b"Art-Net\0";
    const ARTNET_HEADER_SIZE: usize = 18;

    let data = [255; 512];
    let pgk = build_artnet_package(&0, &data);

    assert_eq!(
        pgk.len(),
        ARTNET_HEADER_SIZE + data.len(),
        "package size is not correct"
    );
    assert_eq!(
        pgk[0..8],
        ARTNET_NAME,
        "Art-Net name header is not correct"
    );
    assert_eq!(pgk[18..], data, "data is not the same");

    let mut data_length = (pgk.len() - ARTNET_HEADER_SIZE) as u16;
    data_length += data_length % 2;

    let h_len = (data_length >> 8) as u8;
    let l_len = (data_length & 0xff) as u8;
    assert_eq!(pgk[16], h_len, "high length is not correct");
    assert_eq!(pgk[17], l_len, "low length is not correct");
}
