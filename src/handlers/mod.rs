use std::collections::HashMap;
use std::net::SocketAddr;

use crate::header::{ MessageHeader, MessageType, MessageClass };
use crate::attributes::{ MessageAttribute, AttributeBody };

mod binding;
use binding::Binding;

pub trait MessageHandler {
    fn indication(header: &MessageHeader, body: &HashMap<MessageAttribute, AttributeBody>, origin: &SocketAddr);
    fn request(header: &MessageHeader, body: &HashMap<MessageAttribute, AttributeBody>, origin: &SocketAddr) -> Result<Option<Vec<u8>>, ()>;
}

pub fn process_message(header: &MessageHeader, body: &HashMap<MessageAttribute, AttributeBody>, origin: &SocketAddr) -> Option<Vec<u8>> {
    let mut body = match &header.mtype {
        MessageType::Binding => {
            match &header.mclass {
                MessageClass::Request => {
                    match Binding::request(header, body, origin) {
                        Ok(res) => res,
                        Err(_) => None,
                    }
                },
                MessageClass::Indication => {
                    Binding::indication(header, body, origin);
                    None
                }
                _ => None,
            }
        },
    };

    let response = if let Some(mut body) = body {
        let response_header = MessageHeader {
            mtype: header.mtype,
            mclass: header.mclass,
            length: body.len() as u16,
            id: header.id.clone(),
        };

        let mut response = response_header.serialise();
        response.append(&mut body);
        Some(response)
    } else {
        None
    };

    response
}
