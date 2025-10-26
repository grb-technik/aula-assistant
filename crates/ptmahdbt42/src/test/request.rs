use crate::request::{Body, Method, Request, build_request_package};
use std::collections::HashMap;

#[test]
fn test_build_request_package() {
    let request = Request::new(
        "127.0.0.1".to_string(),
        80,
        Method::POST,
        "/cgi-bin/MMX32_Keyvalue.cgi".to_string(),
        HashMap::new(),
        Some(Body::String("{CMD=>Send_H_4_4:02 50 4F 4E 03".to_string())),
    );

    let buf = build_request_package(&request);
    let hex_string = buf.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    let expected_hex = "504f5354202f6367692d62696e2f4d4d5833325f4b657976616c75652e63676920485454502f312e300d0a486f73743a203132372e302e302e310d0a436f6e74656e742d4c656e6774683a2033310d0a0d0a7b434d443d3e53656e645f485f345f343a3032203530203446203445203033";
    assert_eq!(hex_string, expected_hex);
}
