use crate::dns::{Request, Response, ResponseCheck};
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket};

pub struct NetQuery {}

impl NetQuery {
    pub fn query_tcp(
        mut stream: TcpStream,
        request: Request,
        mut buf: [u8; 1500],
    ) -> Result<Option<Response>, NetQueryError> {
        stream
            .write_all(request.encode_to_tcp(&mut buf))
            .map_err(|_| NetQueryError::WriteTcpConnectError)?;
        stream
            .read(&mut buf)
            .map_err(|_| NetQueryError::ConnectTcpAddrError)?;
        let len = u16::from_be_bytes([buf[0], buf[1]]);
        let response = ResponseCheck::new(&request)
            .check_into_response(&buf.as_slice()[2..(len + 2) as usize]);
        Ok(response)
    }

    pub fn query_udp(
        socket: UdpSocket,
        request: Request,
        mut buf: [u8; 1500],
    ) -> Result<Option<Response>, NetQueryError> {
        let arr = request.encode_to_udp(&mut buf);
        if arr.len() > 512 {
            let stream = TcpStream::connect(
                socket
                    .peer_addr()
                    .map_err(|_| NetQueryError::UdpNotConnected)?,
            )
            .map_err(|_| NetQueryError::ConnectTcpAddrError)?;
            return Ok(Self::query_tcp(stream, request, buf)?);
        }
        socket
            .send(arr)
            .map_err(|_| NetQueryError::SendUdpConnectError)?;
        let number_of_bytes = socket
            .recv(&mut buf)
            .map_err(|_| NetQueryError::RecvUdpConnectError)?;
        let response =
            ResponseCheck::new(&request).check_into_response(&buf.as_slice()[..number_of_bytes]);
        Ok(response)
    }
}

pub enum NetQueryError {
    BindUdpAddrError,
    ConnectUdpAddrError,
    ConnectTcpAddrError,
    UdpNotConnected,
    RecvUdpConnectError,
    SendUdpConnectError,
    WriteTcpConnectError,
}

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
