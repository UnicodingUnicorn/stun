use crate::attributes::Attribute;
use crate::header::MessageHeader;

pub struct UnknownAttributes {
    attributes: Option<Vec<u16>>,
}
impl UnknownAttributes {
    pub fn new_empty() -> UnknownAttributes {
        UnknownAttributes {
            attributes: Some(vec![]), 
        }
    }

    pub fn add(&mut self, attribute:u16) {
        if let Some(attributes) = &mut self.attributes {
            attributes.push(attribute);
        }
    }
}
impl Attribute for UnknownAttributes {
    fn new() -> UnknownAttributes {
        UnknownAttributes {
            attributes: None,
        }
    }

    fn serialise(&self) -> Option<Vec<u8>> {
        let attributes = match &self.attributes {
            Some(attributes) => attributes,
            None => return None,
        };

        let buf = attributes.iter().fold(vec![], |mut acc, attribute| {
            let bytes = attribute.to_be_bytes();
            acc.push(bytes[0]);
            acc.push(bytes[1]);
            acc
        });

        Some(buf)
    }

    fn deserialise(&mut self, body:&[u8], _header:&MessageHeader) -> Result<(), ()> {
        if body.len() % 2 != 0 {
            return Err(());
        }

        let mut attributes = vec![];
        for i in (0..body.len()).step_by(2) {
            let attribute = u16::from_be_bytes([body[i], body[i + 1]]);
            attributes.push(attribute);
        }

        self.attributes = Some(attributes);

        Ok(())
    }
}
