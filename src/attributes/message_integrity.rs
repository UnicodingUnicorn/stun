use crate::attributes::Attribute;
use crate::header::MessageHeader;

pub struct MessageIntegrity {
    hash: Option<[u8; 20]>,
}
impl MessageIntegrity {
    pub fn with_hash(hash:[u8; 20]) -> MessageIntegrity {
        MessageIntegrity {
            hash: Some(hash),
        }
    }
}
impl Attribute for MessageIntegrity {
    fn new() -> MessageIntegrity {
        MessageIntegrity {
            hash: None,
        }
    }

    fn serialise(&self) -> Option<Vec<u8>> {
        match self.hash {
            Some(hash) => Some(hash.to_vec()),
            None => None,
        }
    }

    fn deserialise(&mut self, body:&[u8], _header:&MessageHeader) -> Result<(), ()> {
        if body.len() < 20 {
            return Err(());
        }

        let mut buf = [0; 20];
        for i in 0..20 {
            buf[i] = body[i];
        }
        self.hash = Some(buf);

        Ok(())
    }
}
