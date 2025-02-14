use std::fmt::{Debug, Display, Formatter};

pub struct ErrorFormat {
    info: String,
    debug: String,
}

impl ErrorFormat {
    pub fn new(mut info: String, trace: &str) -> Self {
        info.push('\n');
        let info_new = info.clone();
        info.push_str(format!("trace:\n{}\n", trace).as_str());
        Self {
            info: info_new,
            debug: info,
        }
    }

    pub fn from_vec<T: AsRef<ErrorFormat>>(errs: Vec<T>) -> Self {
        let len = errs.len();
        let mut info = String::with_capacity(len * 10);
        info.push_str("Errors:\n");
        let mut debug = String::with_capacity(len * 40);
        debug.push_str("Errors:\n[\n\t");
        let vec = errs.into_iter().fold((info, debug, 0), |mut b, t| {
            let t = t.as_ref();
            b.0.push_str(format!("{}: {}", b.2, t.info).as_str());
            b.1.push_str(t.debug.replace("\n", "\n\t").as_str());
            b.2 += 1;
            if b.2 < len {
                b.1.push_str("\n\t");
            } else {
                b.1.pop();
                b.1.push_str("]\ntrace:\n");
            }
            b
        });

        Self {
            info: vec.0,
            debug: vec.1,
        }
    }

    pub fn add_trace(&mut self, t: &str) {
        self.debug.push_str(t);
        self.debug.push_str("\n");
    }

    pub fn add_trace_into(mut self, t: &str) -> Self {
        self.debug.push_str(t);
        self.debug.push_str("\n");
        self
    }
}

impl Display for ErrorFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info)
    }
}

impl Debug for ErrorFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug)
    }
}

impl AsRef<ErrorFormat> for ErrorFormat {
    fn as_ref(&self) -> &ErrorFormat {
        self
    }
}

#[cfg(feature = "result_error")]
pub enum NetError {
    ConnectTcpAddrError(ErrorFormat),
    UdpNotConnected(ErrorFormat),
    SendUdpPacketError(ErrorFormat),
    RecvUdpPacketError(ErrorFormat),
    RecvTcpPacketError(ErrorFormat),
    WriteTcpConnectError(ErrorFormat),
    ConnectUdpAddrError(ErrorFormat),
    BindUdpAddrError(ErrorFormat),
}

#[cfg(feature = "result_error")]
impl Display for NetError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            NetError::ConnectTcpAddrError(err)
            | NetError::UdpNotConnected(err)
            | NetError::SendUdpPacketError(err)
            | NetError::RecvUdpPacketError(err)
            | NetError::RecvTcpPacketError(err)
            | NetError::WriteTcpConnectError(err)
            | NetError::ConnectUdpAddrError(err)
            | NetError::BindUdpAddrError(err) => Display::fmt(err, f),
        }
    }
}

#[cfg(feature = "result_error")]
impl Debug for NetError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            NetError::ConnectTcpAddrError(err)
            | NetError::UdpNotConnected(err)
            | NetError::SendUdpPacketError(err)
            | NetError::RecvUdpPacketError(err)
            | NetError::RecvTcpPacketError(err)
            | NetError::WriteTcpConnectError(err)
            | NetError::ConnectUdpAddrError(err)
            | NetError::BindUdpAddrError(err) => Debug::fmt(err, f),
        }
    }
}

impl Into<ErrorFormat> for NetError {
    fn into(self) -> ErrorFormat {
        match self {
            NetError::ConnectTcpAddrError(err)
            | NetError::UdpNotConnected(err)
            | NetError::SendUdpPacketError(err)
            | NetError::RecvUdpPacketError(err)
            | NetError::RecvTcpPacketError(err)
            | NetError::WriteTcpConnectError(err)
            | NetError::ConnectUdpAddrError(err)
            | NetError::BindUdpAddrError(err) => err,
        }
    }
}

impl NetError {
    pub fn get_index(&self) -> &ErrorFormat {
        match self {
            NetError::ConnectTcpAddrError(err)
            | NetError::UdpNotConnected(err)
            | NetError::SendUdpPacketError(err)
            | NetError::RecvUdpPacketError(err)
            | NetError::RecvTcpPacketError(err)
            | NetError::WriteTcpConnectError(err)
            | NetError::ConnectUdpAddrError(err)
            | NetError::BindUdpAddrError(err) => err,
        }
    }
}
