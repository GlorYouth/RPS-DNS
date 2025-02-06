use std::net::{AddrParseError, SocketAddr};
use std::str::FromStr;
use stringzilla::sz::rfind_char_from;

pub enum ServerType {
    Tcp(SocketAddr),
    Udp(SocketAddr),
}

impl ServerType {
    pub fn from_string(s: &mut String) -> Result<ServerType, AddrParseError> {
        Self::check_colon(s);
        if s.starts_with("tcp://") {
            Ok(ServerType::Tcp(SocketAddr::from_str(&s[6..])?))
        } else if s.starts_with("udp://") {
            Ok(ServerType::Udp(SocketAddr::from_str(&s[6..])?))
        } else {
            Ok(ServerType::Udp(SocketAddr::from_str(&s[..])?))
        }
    }

    fn check_colon(s: &mut String) {
        if rfind_char_from(&s, [0x4c_u8]).is_none() {
            s.push_str(":53");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dns::utils::server_type::ServerType;

    #[test]
    fn test_check_colon() {
        let server = &mut "223.5.5.5".to_string();
        ServerType::check_colon(server);
        assert_eq!(server, "223.5.5.5:53");

        let server = &mut "tcp://223.5.5.5".to_string();
        ServerType::check_colon(server);
        assert_eq!(server, "tcp://223.5.5.5:53");

        let server = &mut "udp://223.5.5.5".to_string();
        ServerType::check_colon(server);
        assert_eq!(server, "udp://223.5.5.5:53");
    }

    #[test]
    fn test_server_type() {
        let server = &mut "223.5.5.5".to_string();
        let server_type = ServerType::from_string(server).unwrap();
        match server_type {
            ServerType::Tcp(_) => {
                panic!();
            }
            ServerType::Udp(addr) => {
                assert_eq!(addr.to_string(), "223.5.5.5:53");
            }
        }

        let server = &mut "udp://223.5.5.5".to_string();
        let server_type = ServerType::from_string(server).unwrap();
        match server_type {
            ServerType::Tcp(_) => {
                panic!();
            }
            ServerType::Udp(addr) => {
                assert_eq!(addr.to_string(), "223.5.5.5:53");
            }
        }

        let server = &mut "tcp://223.5.5.5".to_string();
        let server_type = ServerType::from_string(server).unwrap();
        match server_type {
            ServerType::Tcp(addr) => {
                assert_eq!(addr.to_string(), "223.5.5.5:53");
            }
            ServerType::Udp(_) => {
                panic!();
            }
        }
    }
}
