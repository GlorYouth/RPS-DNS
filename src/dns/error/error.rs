use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum NetError {
    ConnectTcpAddrError(String),
    UdpNotConnected(String),
    SendUdpPacketError(String),
    RecvUdpPacketError(String),
    RecvTcpPacketError(String),
    WriteTcpConnectError(String),
    ConnectUdpAddrError(String),
    BindUdpAddrError(String)
}

impl Display for NetError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            NetError::ConnectTcpAddrError(str) => {
                write!(f, "ConnectTcpAddrError({})", str)
            }
            NetError::UdpNotConnected(str) => {
                write!(f, "UdpNotConnected({})", str)
            }
            NetError::SendUdpPacketError(str) => {
                write!(f, "SendUdpPacketError({})", str)
            }
            NetError::RecvUdpPacketError(str) => {
                write!(f, "RecvUdpPacketError({})", str)
            }
            NetError::RecvTcpPacketError(str) => {
                write!(f, "RecvTcpPacketError({})", str)
            }
            NetError::WriteTcpConnectError(str) => {
                write!(f, "WriteTcpConnectError({})", str)
            }
            NetError::ConnectUdpAddrError(str) => {
                write!(f, "ConnectUdpAddrError({})", str)
            }
            NetError::BindUdpAddrError(str) => {
                write!(f, "BindUdpAddrError({})", str)
            }
        }
    }
}