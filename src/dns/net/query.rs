use crate::dns::error::Error;
use crate::dns::{Request, Response, ResponseCheck};
use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket};

pub struct NetQuery {}

impl NetQuery {
    pub fn query_tcp(
        mut stream: TcpStream,
        request: Request,
        mut buf: [u8; 1500],
    ) -> Result<Response, Error> {
        stream
            .write_all(request.encode_to_tcp(&mut buf).unwrap())
            .unwrap();
        stream.read(&mut buf).unwrap();
        let len = u16::from_be_bytes([buf[0], buf[1]]);
        let response = ResponseCheck::new(&request)
            .check_into_response(&buf.as_slice()[2..(len + 2) as usize])
            .unwrap();
        Ok(response)
    }

    pub fn query_udp(
        socket: UdpSocket,
        request: Request,
        mut buf: [u8; 1500],
    ) -> Result<Response, Error> {
        let arr = request.encode_to_udp(&mut buf).unwrap();
        if arr.len() > 512 {
            let stream = TcpStream::connect(socket.peer_addr().unwrap()).unwrap();
            return Self::query_tcp(stream, request, buf);
        }
        socket.send(arr).unwrap();
        let number_of_bytes = socket.recv(&mut buf).expect("Didn't receive data");
        let response = ResponseCheck::new(&request)
            .check_into_response(&buf.as_slice()[..number_of_bytes])
            .unwrap();
        Ok(response)
    }
}
