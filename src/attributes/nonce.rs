use crate::attributes::Attribute;
use crate::header::MessageHeader;

pub struct Nonce {
    nonce: Option<String>,
}
impl Nonce {
    fn with_value(nonce:String) -> Nonce {
        Nonce {
            nonce: Some(nonce),
        }
    }
}
impl Attribute for Nonce {
    fn new() -> Nonce {
        Nonce {
            nonce: None,
        }
    }

    fn serialise(&self) -> Option<Vec<u8>> {
        match &self.nonce {
            Some(nonce) => Some(nonce.clone().as_bytes().to_vec()),
            None => None,
        }
    }

    fn deserialise(&mut self, body:&[u8], _header:&MessageHeader) -> Result<(), ()> {
       if body.len() > 128 { 
            return Err(());
       }
       self.nonce = match String::from_utf8(body.to_vec()) {
            Ok(nonce) => Some(nonce),
            Err(_) => return Err(()),
       };
       Ok(())
    }
}
