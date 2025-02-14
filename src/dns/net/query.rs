use std::fmt::{Debug, Display, Formatter};
#[cfg(feature = "result_error")]
use crate::dns::error::ResultAndError;
#[cfg(feature = "result_error")]
use crate::dns::error::error_trait;
use crate::dns::types::parts::{Request, Response};
use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket};
use crate::dns::error::ErrorFormat;

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
            if let Err(err) = stream.write_all(request.encode_to_tcp(buf)) {
                return NetQueryError::WriteTcpConnectError(ErrorFormat::new(
                    format!("WriteTcpConnectError, target {:?}, {}", stream.peer_addr(), err),
                    "NetQuery::query_tcp()"
                ))
                    .into();
            }
            if let Err(err) = stream.read(buf) {
                return NetQueryError::RecvTcpPacketError(ErrorFormat::new(
                    format!("RecvTcpPacketError, target {:?}, {}", stream.peer_addr(), err),
                    "NetQuery::query_tcp()"
                ))
                    .into();
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
            return match socket.peer_addr() {
                Ok(addr) => match TcpStream::connect(addr) {
                    Ok(stream) => Self::query_tcp(stream, request, buf),
                    Err(err) => NetQueryError::ConnectTcpAddrError(ErrorFormat::new(
                        format!("ConnectTcpAddrError, target {}, {}", addr, err),
                        "NetQuery::query_udp() arr.len() > 512"
                    ))
                    .into(),
                },
                Err(err) => {
                    return NetQueryError::UdpNotConnected(ErrorFormat::new(
                        format!("UdpNotConnected, {}", err),
                        "NetQuery::query_udp() arr.len() > 512"
                    ))
                        .into();
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
            if let Err(err) = socket.send(arr) {
                return NetQueryError::SendUdpPacketError(ErrorFormat::new(
                    format!("UdpPacketSendError, target: {:?}, {}", socket.peer_addr(), err),
                    "NetQuery::query_udp()"
                ))
                    .into();
            }
            match socket.recv(buf) {
                Ok(number_of_bytes) => {
                    let response =
                        Response::from_slice(&buf.as_slice()[..number_of_bytes], &request);
                    return response.into();
                }
                Err(err) => NetQueryError::RecvUdpPacketError(ErrorFormat::new(
                    format!("RecvUdpPacketError, target: {:?}, {}", socket.peer_addr(), err),
                    "NetQuery::query_udp()"
                ))
                    .into(),
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
pub enum NetQueryError {
    ConnectTcpAddrError(ErrorFormat),
    UdpNotConnected(ErrorFormat),
    SendUdpPacketError(ErrorFormat),
    RecvUdpPacketError(ErrorFormat),
    RecvTcpPacketError(ErrorFormat),
    WriteTcpConnectError(ErrorFormat),
}

impl Display for NetQueryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectTcpAddrError(e) |
            Self::UdpNotConnected(e) |
            Self::SendUdpPacketError(e) |
            Self::RecvTcpPacketError(e) |
            Self::RecvUdpPacketError(e) |
            Self::WriteTcpConnectError(e) => {
                Display::fmt(e, f)
            }
        }
    }
}

impl Debug for NetQueryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectTcpAddrError(e) |
            Self::UdpNotConnected(e) |
            Self::SendUdpPacketError(e) |
            Self::RecvTcpPacketError(e) |
            Self::RecvUdpPacketError(e) |
            Self::WriteTcpConnectError(e) => {
                Debug::fmt(e, f)
            }
        }
    }
}