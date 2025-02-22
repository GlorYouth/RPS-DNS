#[cfg(feature = "result_error")]
use crate::dns::error::ErrorFormat;
use crate::dns::types::parts::{Request, Response};
use crate::error::ResultAndError;
#[cfg(feature = "result_error")]
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket};

pub struct NetQuery {}

#[cfg(feature = "result_error")]
type Result = ResultAndError<Option<Response>, NetQueryError>;

#[cfg(not(feature = "result_error"))]
type Result = Option<Response>;

impl NetQuery {
    pub fn query_tcp(mut stream: TcpStream, request: Request, buf: &mut [u8; 1500]) -> Result {
        #[cfg(feature = "result_error")]
        {
            if let Err(err) = stream.write_all(request.encode_to_tcp(buf)) {
                return ResultAndError::from_error(NetQueryError::WriteTcpConnectError(
                    ErrorFormat::new(
                        format!(
                            "WriteTcpConnectError, target {:?}, {}",
                            stream.peer_addr(),
                            err
                        ),
                        "NetQuery::query_tcp()",
                    ),
                ));
            }
            if let Err(err) = stream.read(buf) {
                return ResultAndError::from_error(NetQueryError::RecvTcpPacketError(
                    ErrorFormat::new(
                        format!(
                            "RecvTcpPacketError, target {:?}, {}",
                            stream.peer_addr(),
                            err
                        ),
                        "NetQuery::query_tcp()",
                    ),
                ));
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
            return ResultAndError::from_result(response);
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
                    Err(err) => ResultAndError::from_error(NetQueryError::ConnectTcpAddrError(
                        ErrorFormat::new(
                            format!("ConnectTcpAddrError, target {}, {}", addr, err),
                            "NetQuery::query_udp() arr.len() > 512",
                        ),
                    )),
                },
                Err(err) => {
                    return ResultAndError::from_error(NetQueryError::UdpNotConnected(
                        ErrorFormat::new(
                            format!("UdpNotConnected, {}", err),
                            "NetQuery::query_udp() arr.len() > 512",
                        ),
                    ));
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
                return ResultAndError::from_error(NetQueryError::SendUdpPacketError(
                    ErrorFormat::new(
                        format!(
                            "UdpPacketSendError, target: {:?}, {}",
                            socket.peer_addr(),
                            err
                        ),
                        "NetQuery::query_udp()",
                    ),
                ));
            }
            match socket.recv(buf) {
                Ok(number_of_bytes) => {
                    let response =
                        Response::from_slice(&buf.as_slice()[..number_of_bytes], &request);
                    return ResultAndError::from_result(response);
                }
                Err(err) => {
                    ResultAndError::from_error(NetQueryError::RecvUdpPacketError(ErrorFormat::new(
                        format!(
                            "RecvUdpPacketError, target: {:?}, {}",
                            socket.peer_addr(),
                            err
                        ),
                        "NetQuery::query_udp()",
                    )))
                }
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
#[cfg(feature = "result_error")]
impl Display for NetQueryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectTcpAddrError(e)
            | Self::UdpNotConnected(e)
            | Self::SendUdpPacketError(e)
            | Self::RecvTcpPacketError(e)
            | Self::RecvUdpPacketError(e)
            | Self::WriteTcpConnectError(e) => Display::fmt(e, f),
        }
    }
}
#[cfg(feature = "result_error")]
impl Debug for NetQueryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectTcpAddrError(e)
            | Self::UdpNotConnected(e)
            | Self::SendUdpPacketError(e)
            | Self::RecvTcpPacketError(e)
            | Self::RecvUdpPacketError(e)
            | Self::WriteTcpConnectError(e) => Debug::fmt(e, f),
        }
    }
}
