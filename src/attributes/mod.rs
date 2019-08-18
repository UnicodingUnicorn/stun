use crate::header::MessageHeader;

use std::collections::HashMap;

pub mod mapped_address;
use mapped_address::MappedAddress;
pub mod xor_mapped_address;
use xor_mapped_address::XorMappedAddress;
pub mod username;
use username::Username;
pub mod error_code;
use error_code::ErrorCode;
pub mod unknown_attributes;
use unknown_attributes::UnknownAttributes;
pub mod realm;
use realm::Realm;
pub mod nonce;
use nonce::Nonce;
pub mod message_integrity;
use message_integrity::MessageIntegrity;

#[derive(Hash, PartialEq, Eq)]
pub enum MessageAttribute {
    MappedAddress,
    Username,
    MessageIntegrity,
    ErrorCode,
    UnknownAttributes,
    Realm,
    Nonce,
    XorMappedAddress,
}

trait Attribute {
    fn new() -> Self;

    fn serialise(&self) -> Option<Vec<u8>>;
    fn deserialise(&mut self, body:&[u8], header:&MessageHeader) -> Result<(), ()>;
}

pub enum AttributeBody {
    MappedAddress(MappedAddress),
    XorMappedAddress(XorMappedAddress),
    MessageIntegrity(MessageIntegrity),
    Username(Username),
    Realm(Realm),
    Nonce(Nonce),
    ErrorCode(ErrorCode),
    UnknownAttributes(UnknownAttributes),
}

enum AttributeError {
    TooShort,
    ParsingError(usize),
}

pub fn get_attributes(body: &[u8], header:&MessageHeader) -> HashMap<MessageAttribute, AttributeBody> {
    let mut attributes = HashMap::new();
    let mut i = 0;
    while i < header.length as usize {
        match get_attribute(body, header, i) {
            Ok((attribute, body, length)) => {
                let _ = attributes.insert(attribute, body);   
                i += length;
            },
            Err(AttributeError::TooShort) => break,
            Err(AttributeError::ParsingError(length)) => {
                i += length;
                continue;
            },
        };
    }

    attributes
}

fn get_attribute(body: &[u8], header:&MessageHeader, index: usize) -> Result<(MessageAttribute, AttributeBody, usize), AttributeError> {
    let mut i = index;

    if body.len() < index + 4 {
        return Err(AttributeError::TooShort);
    }
    
    let attribute_type = (body[i] as u16) * 256 + (body[i + 1] as u16);
    i += 2;
    let attribute_length = ((body[i] as u16) * 256 + (body[i + 1] as u16)) as usize;
    i += 2;

    if body.len() < i + attribute_length {
        return Err(AttributeError::TooShort);
    }
    let body = &body[i..(i + attribute_length)];
    let attribute_type = match attribute_type {
        1 => MessageAttribute::MappedAddress,
        32 => MessageAttribute::XorMappedAddress,
        8 => MessageAttribute::MessageIntegrity,
        6 => MessageAttribute::Username,
        20 => MessageAttribute::Realm,
        21 => MessageAttribute::Nonce,
        9 => MessageAttribute::ErrorCode,
        10 => MessageAttribute::UnknownAttributes,
        _ => return Err(AttributeError::ParsingError(attribute_length)),
    };

    let attribute = match attribute_type {
        MessageAttribute::MappedAddress => {
            let mut attribute = MappedAddress::new();
            match attribute.deserialise(body, header) {
                Ok(_) => AttributeBody::MappedAddress(attribute),
                Err(_) => return Err(AttributeError::ParsingError(attribute_length)),
            }
        },
        MessageAttribute::XorMappedAddress => {
            let mut attribute = XorMappedAddress::new();
            match attribute.deserialise(body, header) {
                Ok(_) => AttributeBody::XorMappedAddress(attribute),
                Err(_) => return Err(AttributeError::ParsingError(attribute_length)),
            }
        },
        MessageAttribute::MessageIntegrity => {
            let mut attribute = MessageIntegrity::new();
            match attribute.deserialise(body, header) {
                Ok(_) => AttributeBody::MessageIntegrity(attribute),
                Err(_) => return Err(AttributeError::ParsingError(attribute_length)),
            }
        },
        MessageAttribute::Username => {
            let mut attribute = Username::new();
            match attribute.deserialise(body, header) {
                Ok(_) => AttributeBody::Username(attribute),
                Err(_) => return Err(AttributeError::ParsingError(attribute_length)),
            }
        },
        MessageAttribute::Realm => {
            let mut attribute = Realm::new();
            match attribute.deserialise(body, header) {
                Ok(_) => AttributeBody::Realm(attribute),
                Err(_) => return Err(AttributeError::ParsingError(attribute_length)),
            }
        },
        MessageAttribute::Nonce => {
            let mut attribute = Nonce::new();
            match attribute.deserialise(body, header) {
                Ok(_) => AttributeBody::Nonce(attribute),
                Err(_) => return Err(AttributeError::ParsingError(attribute_length)),
            }
        },
        MessageAttribute::ErrorCode => {
            let mut attribute = ErrorCode::new();
            match attribute.deserialise(body, header) {
                Ok(_) => AttributeBody::ErrorCode(attribute),
                Err(_) => return Err(AttributeError::ParsingError(attribute_length)),
            }
        },
        MessageAttribute::UnknownAttributes => {
            let mut attribute = UnknownAttributes::new();
            match attribute.deserialise(body, header) {
                Ok(_) => AttributeBody::UnknownAttributes(attribute),
                Err(_) => return Err(AttributeError::ParsingError(attribute_length)),
            }
        },
    };
    

    Ok((attribute_type, attribute, attribute_length))
}

pub fn serialise_attribute(attribute:&AttributeBody) -> Vec<u8> {
    let (attribute_type, mut attribute_body) = match match attribute {
        AttributeBody::MappedAddress(attribute) => (1, attribute.serialise()),
        AttributeBody::Username(attribute) => (6, attribute.serialise()),
        AttributeBody::MessageIntegrity(attribute) => (8, attribute.serialise()),
        AttributeBody::ErrorCode(attribute) => (9, attribute.serialise()),
        AttributeBody::UnknownAttributes(attribute) => (10, attribute.serialise()),
        AttributeBody::Realm(attribute) => (20, attribute.serialise()),
        AttributeBody::Nonce(attribute) => (21, attribute.serialise()),
        AttributeBody::XorMappedAddress(attribute) => (32, attribute.serialise()),
    } {
        (attribute_type, Some(attribute_body)) => (attribute_type as u16, attribute_body),
        (attribute_type, None) => (attribute_type as u16, vec![]),
    };

    let length = attribute_body.len() as u16;

    let type_bytes = attribute_type.to_be_bytes();
    let length_bytes = length.to_be_bytes();

    let mut serialised_attribute = vec![type_bytes[0], type_bytes[1], length_bytes[0], length_bytes[1]];
    serialised_attribute.append(&mut attribute_body);

    serialised_attribute
}
