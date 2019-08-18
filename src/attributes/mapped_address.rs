use crate::attributes::Attribute;
use crate::header::MessageHeader;
use std::net::{ IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr };

pub struct MappedAddress {
    address: Option<SocketAddr>,
}
impl MappedAddress {
    pub fn with_address(address:SocketAddr) -> MappedAddress {
        MappedAddress {
            address: Some(address),
        }
    }
}
impl Attribute for MappedAddress {
    fn new() -> MappedAddress {
        MappedAddress {
            address: None,
        }
    }

    fn serialise(&self) -> Option<Vec<u8>>{
        let address = match self.address {
            Some(address) => address,
            None => return None,
        };

        let family = if address.is_ipv4() {
            1
        } else if address.is_ipv6() {
            2
        } else {
            return None;
        };

        let p1 = (address.port() / 256) as u8;
        let p2 = (address.port() % 256) as u8;

        let address = match address.ip(){
            IpAddr::V4(address) => address.octets().to_vec(),
            IpAddr::V6(address) => address.octets().to_vec(),
        };

        let buf = vec![0, family, p1, p2].iter()
                                         .chain(&address)
                                         .map(|b| *b)
                                         .collect::<Vec<u8>>();
        Some(buf)
    }

    fn deserialise(&mut self, body:&[u8], _header:&MessageHeader) -> Result<(), ()> {
        if body.len() < 4 {
            return Err(());
        }
        let port = u16::from_be_bytes([body[2], body[3]]);
        let address = match (body[1], body.len()) {
            (1, 8) => {
                if body.len() < 8 {
                    return Err(());
                }
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(body[4], body[5], body[6], body[7])), port)
            },
            (2, 20) => {
                if body.len() < 20 {
                    return Err(());
                }
                let address_bits = body.chunks(2)
                                       .skip(2)
                                       .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                                       .collect::<Vec<u16>>();
                if address_bits.len() < 8 {
                    return Err(());
                }
                SocketAddr::new(IpAddr::V6(Ipv6Addr::new(address_bits[0], address_bits[1], address_bits[2], address_bits[3], address_bits[4], address_bits[5], address_bits[6], address_bits[7])), port)
            },
            _ => return Err(()),
        };
        self.address = Some(address);
        Ok(())
    }
}
