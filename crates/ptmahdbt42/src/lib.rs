use crate::{
    error::PTMAHDBT42Error,
    request::{Request, build_request_package},
    response::{Response, parse_response_buffer},
};
use std::{
    io::{Read, Write},
    net::{IpAddr, Shutdown, SocketAddr, TcpStream},
    time::Duration,
    vec,
};

pub mod error;
pub mod request;
pub mod response;

pub(crate) const CRLF: &[u8; 2] = &[0x0D, 0x0A];
pub(crate) const LFLF: &[u8; 2] = &[0x0A, 0x0A];
// const RS232_COMMAND_ENDPOINT: &str = "/cgi-bin/MMX32_Keyvalue.cgi";
// const RS232_COMMAND_METHOD: &str = "POST";
// const RS232_BODY_POWER_ON: &str = "{CMD=>Send_H_4_4:02 50 4F 4E 03";
// const RS232_BODY_POWER_OFF: &str = "{CMD=>Send_H_4_4:02 50 4F 46 03";

pub fn execute_request(request: &Request) -> Result<Response, PTMAHDBT42Error> {
    let addr: SocketAddr = SocketAddr::new(
        IpAddr::V4(
            request
                .host()
                .parse()
                .expect("request.host is an invalid ipv4 address"),
        ),
        request.port(),
    );

    let mut stream = TcpStream::connect(addr).map_err(PTMAHDBT42Error::FailedToBindSocket)?;

    let request_buf = build_request_package(request);

    stream
        .write(&request_buf)
        .map_err(PTMAHDBT42Error::FailedToSendRequest)?;

    let mut response_buf: Vec<u8> = vec![];

    stream
        .set_read_timeout(Some(Duration::from_millis(1000)))
        .map_err(PTMAHDBT42Error::FailedToSetReadTimeout)?;

    stream
        .read_to_end(&mut response_buf)
        .map_err(PTMAHDBT42Error::FailedToReadResponse)?;

    stream
        .shutdown(Shutdown::Both)
        .map_err(PTMAHDBT42Error::FailedToShutdownStream)?;

    parse_response_buffer(response_buf)
}
