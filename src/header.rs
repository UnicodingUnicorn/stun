#[derive(Copy, Clone)]
pub enum MessageType {
    Binding
}

#[derive(Copy, Clone)]
pub enum MessageClass {
    Request,
    Indication,
    Success,
    Error,
}

pub struct MessageHeader {
    pub mtype: MessageType,
    pub mclass: MessageClass,
    pub length: u16,
    pub id: [u8; 12],
}
impl MessageHeader {
    pub fn serialise(&self) -> Vec<u8> {
        let mut message_type:u16 = match &self.mtype {
            MessageType::Binding => 1,
        };
        message_type |= match &self.mclass {
            MessageClass::Request => 0b0000000000000000,
            MessageClass::Indication => 0b0000000000010000,
            MessageClass::Success => 0b0000000100000000,
            MessageClass::Error => 0b0000000100010000,
        };

        let mut serialised_header = message_type.to_be_bytes().to_vec();
        let length_bytes = self.length.to_be_bytes();
        serialised_header.push(length_bytes[0]);
        serialised_header.push(length_bytes[1]);
        let mut id = self.id.to_vec();
        serialised_header.append(&mut id);

        serialised_header
    }
}

pub fn verify_header(header:&[u8]) -> Result<MessageHeader, u32> {
    // Check header length is 20 bytes
    if header.len() != 20 {
        return Err(1);
    }

    // Verify first two bits is 0
    if header[0] >> 6 != 0 {
        return Err(1);
    }

    // Check magic cookie
    if !(header[4] == 0x21 && header[5] == 0x12 && header[6] == 0xA4 && header[7] == 0x42) {
        return Err(1);
    }

    // Parse header
    let mclass = get_message_class(header[0], header[1]);
    let mtype = match get_message_type(header[0], header[1]) {
        Some(mtype) => mtype,
        None => return Err(1),
    };
    let length = 256 * (header[2] as u16) + (header[3] as u16);
    let mut id = [0; 12];
    id.copy_from_slice(&header[8..]);
    
    let header = MessageHeader {
        mtype,
        mclass,
        length,
        id,
    };
    Ok(header)
}

fn get_message_class(b1: u8, b2: u8) -> MessageClass {
    let cb1 = (b1 & 1) != 0;
    let cb2 = (b2 & 16) != 0;
    match (cb1, cb2) {
        (false, false) => MessageClass::Request,
        (false, true) => MessageClass::Indication,
        (true, false) => MessageClass::Success,
        (true, true) => MessageClass::Error,
    }
}

fn get_message_type(b1: u8, b2: u8) -> Option<MessageType> {
    // Check for Binding
    let p1 = b1 == 1 || b1 == 0;
    let p2 = b2 == 17 || b2 == 1;
    if p1 && p2 {
        Some(MessageType::Binding)
    } else {
        None
    }
}
