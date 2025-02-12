#[cfg(feature = "result_error")]
use crate::dns::error::error_trait;
#[cfg(feature = "result_error")]
use crate::dns::error::ResultAndError;
use crate::dns::types::parts::{Request, Response};
#[cfg(feature = "result_error")]
use std::fmt::{Debug};
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
            if stream.write_all(request.encode_to_tcp(buf)).is_err() {
                return NetQueryError::WriteTcpConnectError("WriteTcpConnectError 写入TcpConnect失败".to_string()).into();
            }
            if stream.read(buf).is_err() {
                return NetQueryError::RecvTcpPacketError(format!("RecvUdpPacketError Udp包接受失败,目标地址: {:?}", stream.peer_addr())).into()
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
            return if let Ok(addr) = socket.peer_addr() {
                if let Ok(stream) = TcpStream::connect(addr) {
                    Self::query_tcp(stream, request, buf)
                } else {
                    return NetQueryError::ConnectTcpAddrError(format!("ConnectTcpAddrError 连接到对应的tcp server失败 {}", addr)).into();
                }
            } else {
                return NetQueryError::UdpNotConnected("UdpNotConnected udp server未连接".to_string()).into();
            };
            #[cfg(not(feature = "result_error"))]
            {
                let stream = TcpStream::connect(socket.peer_addr().ok()?).ok()?;
                return Self::query_tcp(stream, request, buf);
            }
        }
        #[cfg(feature = "result_error")]
        {
            if socket.send(arr).is_err() {
                return NetQueryError::UdpPacketSendError(format!("UdpPacketSendError Udp包发送失败,目标地址: {:?}", socket.peer_addr())).into();
            }
            if let Ok(number_of_bytes) = socket.recv(buf) {
                let response = Response::from_slice(&buf.as_slice()[..number_of_bytes], &request);
                return response.into()
            };
            NetQueryError::RecvUdpPacketError(format!("RecvUdpPacketError Udp包接受失败,目标地址: {:?}", socket.peer_addr())).into()
            
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
#[derive(Debug)]
pub enum NetQueryError {
    ConnectTcpAddrError(String),
    UdpNotConnected(String),
    UdpPacketSendError(String),
    RecvUdpPacketError(String),
    RecvTcpPacketError(String),
    WriteTcpConnectError(String)
}