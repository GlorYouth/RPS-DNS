#[cfg(feature = "result_error")]
use crate::dns::error::ResultAndError;
#[cfg(feature = "result_error")]
use crate::dns::error::debug_fmt;
#[cfg(feature = "result_error")]
use crate::dns::error::error_trait;
use crate::dns::types::parts::{Request, Response};
#[cfg(feature = "result_error")]
use snafu::{ResultExt, Snafu};
#[cfg(feature = "result_error")]
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket};

pub struct NetQuery {}

#[cfg(feature = "result_error")]
type Result = ResultAndError<Response, NetQueryError>;

#[cfg(feature = "result_error")]
impl error_trait::B for NetQueryError {}

#[cfg(not(feature = "result_error"))]
type Result = Option<Response>;

impl NetQuery {
    pub fn query_tcp(mut stream: TcpStream, request: Request, buf: &mut [u8; 1500]) -> Result {
        #[cfg(feature = "result_error")]
        {
            if let Err(err) =
                stream
                    .write_all(request.encode_to_tcp(buf))
                    .context(WriteTcpConnectSnafu {
                        target: debug_fmt(stream.peer_addr()),
                    })
            {
                return err.into();
            }
            if let Err(err) = stream.read(buf).context(RecvTcpPacketSnafu {
                target: debug_fmt(stream.peer_addr()),
            }) {
                return err.into();
            }
        }
        #[cfg(not(feature = "result_error"))]
        {
            stream.write_all(request.encode_to_tcp(buf)).ok()?;
            stream.read(buf).ok()?;
        }
        let len = u16::from_be_bytes([buf[0], buf[1]]);
        let response = Response::from_slice(&buf.as_slice()[2..(len + 2) as usize], &request);
        #[cfg(feature = "result_error")]
        {
            return response.into();
        }
        #[cfg(not(feature = "result_error"))]
        response
    }

    pub fn query_udp(socket: UdpSocket, request: Request, buf: &mut [u8; 1500]) -> Result {
        let arr = request.encode_to_udp(buf);
        if arr.len() > 512 {
            #[cfg(feature = "result_error")]
            return match socket.peer_addr().context(UdpNotConnectedSnafu {
                target: debug_fmt(socket.peer_addr()),
            }) {
                Ok(addr) => match TcpStream::connect(addr) {
                    Ok(stream) => Self::query_tcp(stream, request, buf),
                    Err(err) => NetQueryError::ConnectTcpAddrError {
                        target: addr.to_string(),
                        source: err,
                    }
                    .into(),
                },
                Err(err) => {
                    return err.into();
                }
            };

            #[cfg(not(feature = "result_error"))]
            {
                let stream = TcpStream::connect(socket.peer_addr().ok()?).ok()?;
                return Self::query_tcp(stream, request, buf);
            }
        }
        #[cfg(feature = "result_error")]
        {
            if let Err(err) = socket.send(arr).context(UdpPacketSendSnafu {
                target: format!("{:?}", socket.peer_addr()),
            }) {
                return err.into();
            }
            match socket.recv(buf).context(RecvUdpPacketSnafu {
                target: format!("{:?}", socket.peer_addr()),
            }) {
                Ok(number_of_bytes) => {
                    let response =
                        Response::from_slice(&buf.as_slice()[..number_of_bytes], &request);
                    return response.into();
                }
                Err(err) => err.into(),
            }
        }
        #[cfg(not(feature = "result_error"))]
        {
            socket.send(arr).ok()?;
            let number_of_bytes = socket.recv(buf).ok()?;
            Response::from_slice(&buf.as_slice()[..number_of_bytes], &request)
        }
    }
}
#[cfg(feature = "result_error")]
#[derive(Snafu, Debug)]
pub enum NetQueryError {
    #[snafu(display("ConnectTcpAddrError, target: {}, info: {}", target, source.to_string()))]
    ConnectTcpAddrError {
        target: String,
        source: std::io::Error,
    },
    #[snafu(display("UdpNotConnected, target: {}, info: {}", target, source.to_string()))]
    UdpNotConnected {
        target: String,
        source: std::io::Error,
    },
    #[snafu(display("UdpPacketSendError, target: {}, info: {}", target, source.to_string()))]
    UdpPacketSendError {
        target: String,
        source: std::io::Error,
    },
    #[snafu(display("RecvUdpPacketError, target: {}, info: {}", target, source.to_string()))]
    RecvUdpPacketError {
        target: String,
        source: std::io::Error,
    },
    #[snafu(display("RecvTcpPacketError, target: {}, info: {}", target, source.to_string()))]
    RecvTcpPacketError {
        target: String,
        source: std::io::Error,
    },
    #[snafu(display("WriteTcpConnectError, target: {}, info: {}", target, source.to_string()))]
    WriteTcpConnectError {
        target: String,
        source: std::io::Error,
    },
}
