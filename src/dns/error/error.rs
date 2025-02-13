use std::fmt::{Debug, Display, Formatter};

#[inline]
pub fn debug_fmt<T: Debug>(v: T) -> String {
    format!("{:?}", v)
}

pub struct TraceErrorFormat {
    pub(crate) info: String,
    pub(crate) trace: String,
}

impl TraceErrorFormat {
    pub fn get_info(&self) -> String {
        self.info.clone()
    }

    pub fn get_trace(&self) -> String {
        self.trace.clone()
    }

    pub fn add_trace(&mut self, s: &str) {
        self.trace = format!("{}\n{}", s, self.trace);
    }
}

impl From<Vec<TraceErrorFormat>> for TraceErrorFormat {
    fn from(vec: Vec<TraceErrorFormat>) -> Self {
        let mut info = String::with_capacity(40);
        for e in vec {
            info.push_str(format!("{}\ntrace:{}\n\n", e.info, e.trace).as_str());
        }
        info.push('\n');
        TraceErrorFormat {
            info,
            trace: String::new(),
        }
    }
}

#[cfg(feature = "result_error")]
pub enum NetError {
    ConnectTcpAddrError(TraceErrorFormat),
    UdpNotConnected(TraceErrorFormat),
    SendUdpPacketError(TraceErrorFormat),
    RecvUdpPacketError(TraceErrorFormat),
    RecvTcpPacketError(TraceErrorFormat),
    WriteTcpConnectError(TraceErrorFormat),
    ConnectUdpAddrError(TraceErrorFormat),
    BindUdpAddrError(TraceErrorFormat),
}

#[cfg(feature = "result_error")]
impl Display for NetError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            NetError::ConnectTcpAddrError(err) => {
                write!(f, "ConnectTcpAddrError {}", err.info)
            }
            NetError::UdpNotConnected(err) => {
                write!(f, "UdpNotConnected {}", err.info)
            }
            NetError::SendUdpPacketError(err) => {
                write!(f, "SendUdpPacketError {}", err.info)
            }
            NetError::RecvUdpPacketError(err) => {
                write!(f, "RecvUdpPacketError {}", err.info)
            }
            NetError::RecvTcpPacketError(err) => {
                write!(f, "RecvTcpPacketError {}", err.info)
            }
            NetError::WriteTcpConnectError(err) => {
                write!(f, "WriteTcpConnectError {}", err.info)
            }
            NetError::ConnectUdpAddrError(err) => {
                write!(f, "ConnectUdpAddrError {}", err.info)
            }
            NetError::BindUdpAddrError(err) => {
                write!(f, "BindUdpAddrError {}", err.info)
            }
        }
    }
}

#[cfg(feature = "result_error")]
impl Debug for NetError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            NetError::ConnectTcpAddrError(err) => {
                write!(
                    f,
                    "NetError::ConnectTcpAddrError {}\ntrace:\n{}",
                    err.info, err.trace
                )
            }
            NetError::UdpNotConnected(err) => {
                write!(
                    f,
                    "NetError::UdpNotConnected {}\ntrace:\n{}",
                    err.info, err.trace
                )
            }
            NetError::SendUdpPacketError(err) => {
                write!(
                    f,
                    "NetError::SendUdpPacketError {}\ntrace:\n{}",
                    err.info, err.trace
                )
            }
            NetError::RecvUdpPacketError(err) => {
                write!(
                    f,
                    "NetError::RecvUdpPacketError {}\ntrace:\n{}",
                    err.info, err.trace
                )
            }
            NetError::RecvTcpPacketError(err) => {
                write!(
                    f,
                    "NetError::RecvTcpPacketError {}\ntrace:\n{}",
                    err.info, err.trace
                )
            }
            NetError::WriteTcpConnectError(err) => {
                write!(
                    f,
                    "NetError::WriteTcpConnectError {}\ntrace:\n{}",
                    err.info, err.trace
                )
            }
            NetError::ConnectUdpAddrError(err) => {
                write!(
                    f,
                    "NetError::ConnectUdpAddrError {}\ntrace:\n{}",
                    err.info, err.trace
                )
            }
            NetError::BindUdpAddrError(err) => {
                write!(
                    f,
                    "NetError::BindUdpAddrError {}\ntrace:\n{}",
                    err.info, err.trace
                )
            }
        }
    }
}
