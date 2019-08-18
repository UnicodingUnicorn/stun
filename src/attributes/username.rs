use crate::attributes::Attribute;
use crate::header::MessageHeader;

pub struct Username {
    username: Option<String>,
}
impl Username {
    fn with_value(username:String) -> Username {
        Username {
            username: Some(username),
        }
    }
}
impl Attribute for Username {
    fn new() -> Username {
        Username {
            username: None,
        }
    }

    fn serialise(&self) -> Option<Vec<u8>> {
        match &self.username {
            Some(username) => Some(username.clone().as_bytes().to_vec()),
            None => None,
        }
    }

    fn deserialise(&mut self, body:&[u8], _header:&MessageHeader) -> Result<(), ()> {
       if body.len() > 512 { 
            return Err(());
       }
       self.username = match String::from_utf8(body.to_vec()) {
            Ok(username) => Some(username),
            Err(_) => return Err(()),
       };
       Ok(())
    }
}
