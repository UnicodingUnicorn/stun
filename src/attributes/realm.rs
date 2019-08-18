use crate::attributes::Attribute;
use crate::header::MessageHeader;

pub struct Realm {
    realm: Option<String>,
}
impl Realm {
    fn with_value(realm:String) -> Realm {
        Realm {
            realm: Some(realm),
        }
    }
}
impl Attribute for Realm {
    fn new() -> Realm {
        Realm {
            realm: None,
        }
    }

    fn serialise(&self) -> Option<Vec<u8>> {
        match &self.realm {
            Some(realm) => Some(realm.clone().as_bytes().to_vec()),
            None => None,
        }
    }

    fn deserialise(&mut self, body:&[u8], _header:&MessageHeader) -> Result<(), ()> {
       if body.len() > 128 { 
            return Err(());
       }
       self.realm = match String::from_utf8(body.to_vec()) {
            Ok(realm) => Some(realm),
            Err(_) => return Err(()),
       };
       Ok(())
    }
}
