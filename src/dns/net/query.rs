use crate::dns::types::parts::{Request, Response};
#[cfg(feature = "result_error")]
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket};

pub struct NetQuery {}

#[cfg(feature = "result_error")]
type Result = std::result::Result<Option<Response>, NetQueryError>;

#[cfg(not(feature = "result_error"))]
type Result = Option<Response>;

impl NetQuery {
    pub fn query_tcp(mut stream: TcpStream, request: Request, mut buf: [u8; 1500]) -> Result {
        #[cfg(feature = "result_error")]
        {
            stream
                .write_all(request.encode_to_tcp(&mut buf))
                .map_err(|_| NetQueryError::WriteTcpConnectError)?;
            stream
                .read(&mut buf)
                .map_err(|_| NetQueryError::ConnectTcpAddrError)?;
        }
        #[cfg(not(feature = "result_error"))]
        {
            stream.write_all(request.encode_to_tcp(&mut buf)).ok()?;
            stream.read(&mut buf).ok()?;
        }
        let len = u16::from_be_bytes([buf[0], buf[1]]);
        let response = Response::from_slice(&buf.as_slice()[2..(len + 2) as usize], &request);
        #[cfg(feature = "result_error")]
        {
            return Ok(response);
        }
        #[cfg(not(feature = "result_error"))]
        response
    }

    pub fn query_udp(socket: UdpSocket, request: Request, mut buf: [u8; 1500]) -> Result {
        let arr = request.encode_to_udp(&mut buf);
        if arr.len() > 512 {
            #[cfg(feature = "result_error")]
            let stream = TcpStream::connect(
                socket
                    .peer_addr()
                    .map_err(|_| NetQueryError::UdpNotConnected)?,
            )
            .map_err(|_| NetQueryError::ConnectTcpAddrError)?;
            #[cfg(not(feature = "result_error"))]
            let stream = TcpStream::connect(socket.peer_addr().ok()?).ok()?;
            return Self::query_tcp(stream, request, buf);
        }
        #[cfg(feature = "result_error")]
        {
            socket
                .send(arr)
                .map_err(|_| NetQueryError::SendUdpConnectError)?;
            let number_of_bytes = socket
                .recv(&mut buf)
                .map_err(|_| NetQueryError::RecvUdpConnectError)?;
            let response = Response::from_slice(&buf.as_slice()[..number_of_bytes], &request);
            Ok(response)
        }
        #[cfg(not(feature = "result_error"))]
        {
            socket.send(arr).ok()?;
            let number_of_bytes = socket.recv(&mut buf).ok()?;
            Response::from_slice(&buf.as_slice()[..number_of_bytes], &request)
        }
    }
}

#[cfg(feature = "result_error")]
pub enum NetQueryError {
    BindUdpAddrError,
    ConnectUdpAddrError,
    ConnectTcpAddrError,
    UdpNotConnected,
    RecvUdpConnectError,
    SendUdpConnectError,
    WriteTcpConnectError,
}

#[cfg(feature = "result_error")]
impl Debug for NetQueryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NetQueryError::BindUdpAddrError => f.write_str("QueryError::BindUdpAddrError"),
            NetQueryError::ConnectUdpAddrError => f.write_str("QueryError::ConnectUdpAddrError"),
            NetQueryError::ConnectTcpAddrError => f.write_str("QueryError::ConnectTcpAddrError"),
            NetQueryError::UdpNotConnected => f.write_str("QueryError::UdpNotConnected"),
            NetQueryError::RecvUdpConnectError => f.write_str("QueryError::RecvUdpConnectError"),
            NetQueryError::SendUdpConnectError => f.write_str("QueryError::SendUdpConnectError"),
            NetQueryError::WriteTcpConnectError => f.write_str("QueryError::WriteTcpConnectError"),
        }
    }
}

#[cfg(feature = "result_error")]
impl Display for NetQueryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NetQueryError::BindUdpAddrError => f.write_str("BindUdpAddrError"),
            NetQueryError::ConnectUdpAddrError => f.write_str("ConnectUdpAddrError"),
            NetQueryError::ConnectTcpAddrError => f.write_str("ConnectTcpAddrError"),
            NetQueryError::UdpNotConnected => f.write_str("UdpNotConnected"),
            NetQueryError::RecvUdpConnectError => f.write_str("RecvUdpConnectError"),
            NetQueryError::SendUdpConnectError => f.write_str("SendUdpConnectError"),
            NetQueryError::WriteTcpConnectError => f.write_str("WriteTcpConnectError"),
        }
    }
}
