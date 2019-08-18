use crate::attributes::Attribute;
use crate::header::MessageHeader;
use std::net::{ IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr };

pub struct XorMappedAddress {
    address: Option<SocketAddr>,
    address_key: Vec<u8>,
}
impl XorMappedAddress {  
    pub fn with_address(address:SocketAddr, key:Vec<u8>) -> Result<XorMappedAddress, ()> {
        if address.is_ipv4() && key.len() != 4 {
            Err(())
        } else if address.is_ipv6() && key.len() != 16 {
            Err(())
        } else {
            Ok(XorMappedAddress {
                address: Some(address),
                address_key: key,
            })
        }
    }
}
impl Attribute for XorMappedAddress {
    fn new() -> XorMappedAddress {
        XorMappedAddress {
            address: None,
            address_key: vec![],
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

        let p1 = ((address.port() / 256) as u8) ^ 0x21;
        let p2 = ((address.port() % 256) as u8) ^ 0x12;

        let address = match address.ip(){
            IpAddr::V4(address) => address.octets().to_vec(),
            IpAddr::V6(address) => address.octets().to_vec(),
        };
        let address = address.iter()
                             .zip(self.address_key.iter())
                             .map(|(octet, key)| octet ^ key)
                             .collect::<Vec<u8>>();

        let buf = vec![0, family, p1, p2].iter()
                                         .chain(&address)
                                         .map(|b| *b)
                                         .collect::<Vec<u8>>();
        Some(buf)
    }

    fn deserialise(&mut self, body:&[u8], header:&MessageHeader) -> Result<(), ()> {
        let port = u16::from_be_bytes([body[2] ^ 0x21, body[3] ^ 0x12]);
        let address_key = match body[1] {
            1 => vec![0x21, 0x12, 0xA4, 0x42],
            2 => vec![0x21, 0x12, 0xA4, 0x42].iter().chain(&header.id).map(|b| *b).collect::<Vec<u8>>(),
            _ => vec![],
        };
        let address = match (body[1], body.len()) {
            (1, 8) => {
                if body.len() < 8 {
                    return Err(());
                }
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(body[4] ^ 0x21, body[5] ^ 0x12, body[6] ^ 0xA4, body[7] ^ 0x42)), port)
            },
            (2, 20) => {
                if body.len() < 20 {
                    return Err(());
                }
                let address_bits = body.chunks(2)
                                       .skip(2)
                                       .zip(address_key.chunks(2))
                                       .map(|(address_chunk, key_chunk)| {
                                            u16::from_be_bytes([address_chunk[0] ^ key_chunk[0], address_chunk[1] ^ key_chunk[1]])
                                       })
                                       .collect::<Vec<u16>>();
                SocketAddr::new(IpAddr::V6(Ipv6Addr::new(address_bits[0], address_bits[1], address_bits[2], address_bits[3], address_bits[4], address_bits[5], address_bits[6], address_bits[7])), port)
            },
            _ => return Err(()),
        };
        self.address = Some(address);
        self.address_key = address_key;
        Ok(())
    }
}
