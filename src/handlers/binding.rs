use std::collections::HashMap;
use std::net::{ IpAddr, SocketAddr };

use crate::attributes::{ self, MessageAttribute, AttributeBody };
use crate::attributes::xor_mapped_address::XorMappedAddress;
use crate::handlers::MessageHandler;
use crate::header::MessageHeader;

pub struct Binding {
}
impl MessageHandler for Binding {
    fn indication(_header: &MessageHeader, _body:&HashMap<MessageAttribute, AttributeBody>, _origin: &SocketAddr){
    }
    fn request(header: &MessageHeader, _body:&HashMap<MessageAttribute, AttributeBody>, origin: &SocketAddr) -> Result<Option<Vec<u8>>, ()> {
        let origin = origin.clone();
        let key = match origin.ip() {
            IpAddr::V4(_) => vec![0x21, 0x12, 0xA4, 0x42],
            IpAddr::V6(_) => vec![0x21, 0x12, 0xA4, 0x42].iter().chain(&header.id).map(|b| *b).collect::<Vec<u8>>(),
        };
        let xor_mapped_address = match XorMappedAddress::with_address(origin, key) {
            Ok(xor_mapped_address) => xor_mapped_address,
            Err(_) => return Err(()),
        };
        let body = attributes::serialise_attribute(&AttributeBody::XorMappedAddress(xor_mapped_address));
        Ok(Some(body))
    }
}
