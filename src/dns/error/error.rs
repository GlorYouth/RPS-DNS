use std::fmt::{Debug, Display, Formatter};

#[inline]
pub fn debug_fmt<T: Debug>(v: T) -> String {
    format!("{:?}", v)
}

#[cfg(feature = "result_error")]
pub enum NetError {
    ConnectTcpAddrError { info: String, path: String },
    UdpNotConnected { info: String, path: String },
    SendUdpPacketError { info: String, path: String },
    RecvUdpPacketError { info: String, path: String },
    RecvTcpPacketError { info: String, path: String },
    WriteTcpConnectError { info: String, path: String },
    ConnectUdpAddrError { info: String, path: String },
    BindUdpAddrError { info: String, path: String },
}

#[cfg(feature = "result_error")]
impl Display for NetError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            NetError::ConnectTcpAddrError { info, path: _ } => {
                write!(f, "ConnectTcpAddrError {}", info)
            }
            NetError::UdpNotConnected { info, path: _ } => {
                write!(f, "UdpNotConnected {}", info)
            }
            NetError::SendUdpPacketError { info, path: _ } => {
                write!(f, "SendUdpPacketError {}", info)
            }
            NetError::RecvUdpPacketError { info, path: _ } => {
                write!(f, "RecvUdpPacketError {}", info)
            }
            NetError::RecvTcpPacketError { info, path: _ } => {
                write!(f, "RecvTcpPacketError {}", info)
            }
            NetError::WriteTcpConnectError { info, path: _ } => {
                write!(f, "WriteTcpConnectError {}", info)
            }
            NetError::ConnectUdpAddrError { info, path: _ } => {
                write!(f, "ConnectUdpAddrError {}", info)
            }
            NetError::BindUdpAddrError { info, path: _ } => {
                write!(f, "BindUdpAddrError {}", info)
            }
        }
    }
}

#[cfg(feature = "result_error")]
impl Debug for NetError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            NetError::ConnectTcpAddrError { info, path } => {
                write!(f, "NetError::ConnectTcpAddrError {}\npath: {}", info, path)
            }
            NetError::UdpNotConnected { info, path } => {
                write!(f, "NetError::UdpNotConnected {}\n\tpath: {}", info, path)
            }
            NetError::SendUdpPacketError { info, path } => {
                write!(f, "NetError::SendUdpPacketError {}\n\tpath: {}", info, path)
            }
            NetError::RecvUdpPacketError { info, path } => {
                write!(f, "NetError::RecvUdpPacketError {}\n\tpath: {}", info, path)
            }
            NetError::RecvTcpPacketError { info, path } => {
                write!(f, "NetError::RecvTcpPacketError {}\n\tpath: {}", info, path)
            }
            NetError::WriteTcpConnectError { info, path } => {
                write!(
                    f,
                    "NetError::WriteTcpConnectError {}\n\tpath: {}",
                    info, path
                )
            }
            NetError::ConnectUdpAddrError { info, path } => {
                write!(
                    f,
                    "NetError::ConnectUdpAddrError {}\n\tpath: {}",
                    info, path
                )
            }
            NetError::BindUdpAddrError { info, path } => {
                write!(f, "NetError::BindUdpAddrError {}\n\tpath: {}", info, path)
            }
        }
    }
}
