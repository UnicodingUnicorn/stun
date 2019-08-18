use crate::attributes::Attribute;
use crate::header::MessageHeader;

pub enum ErrorCodeType {
    TryAlternate,       // 300    
    BadRequest,         // 400
    Unauthorised,       // 401
    UnknownAttribute,   // 420
    StaleNonce,         // 438
    ServerError,        // 500
}
pub struct ErrorCode {
    code: Option<ErrorCodeType>,
    message: Option<String>,
}
impl ErrorCode {
    fn with_code(code:u16, message:String) -> ErrorCode {
        let code = match code {
            300 => Some(ErrorCodeType::TryAlternate),
            400 => Some(ErrorCodeType::BadRequest),
            401 => Some(ErrorCodeType::Unauthorised),
            420 => Some(ErrorCodeType::UnknownAttribute),
            438 => Some(ErrorCodeType::StaleNonce),
            500 => Some(ErrorCodeType::ServerError),
            _ => None,
        };
        ErrorCode {
            code,
            message: Some(message),
        }
    }
}
impl Attribute for ErrorCode {
    fn new() -> ErrorCode {
        ErrorCode {
            code: None,
            message: None,
        }
    }

    fn serialise(&self) -> Option<Vec<u8>> {
        let code = match self.code {
            Some(ErrorCodeType::TryAlternate) => 300,
            Some(ErrorCodeType::BadRequest) => 400,
            Some(ErrorCodeType::Unauthorised) => 401,
            Some(ErrorCodeType::UnknownAttribute) => 420,
            Some(ErrorCodeType::StaleNonce) => 438,
            Some(ErrorCodeType::ServerError) => 500,
            None => return None,
        };
        let class = (code / 100) as u8;
        let error_number = (code % 100) as u8;

        let mut message = match &self.message {
            Some(message) => message.clone().as_bytes().to_vec(),
            None => return None,
        };
        
        let mut buf = vec![0, 0, class, error_number];
        buf.append(&mut message);
        Some(buf)
    }

    fn deserialise(&mut self, body:&[u8], _header:&MessageHeader) -> Result<(), ()> {
        if body.len() < 4 {
            return Err(());
        }

        let class = body[3] & 0b00000111;
        let code = class as u16 * 100 + body[4] as u16 % 100;
        let error_code = match code {
            300 => ErrorCodeType::TryAlternate,
            400 => ErrorCodeType::BadRequest,
            401 => ErrorCodeType::Unauthorised,
            420 => ErrorCodeType::UnknownAttribute,
            438 => ErrorCodeType::StaleNonce,
            500 => ErrorCodeType::ServerError,
            _ => return Err(()),
        };

        let message_bytes = &body[5..body.len()];
        let message = match String::from_utf8(message_bytes.to_vec()) {
            Ok(message) => message,
            Err(_) => return Err(()),
        };

        self.code = Some(error_code);
        self.message = Some(message);

        Ok(())
    }
}
