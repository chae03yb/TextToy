use std::borrow::Cow;
use encoding;
use encoding::codec::{
    utf_8::UTF8Encoding,
    ascii::ASCIIEncoding,
};
use encoding::{DecoderTrap, EncoderTrap, Encoding};


pub(crate) enum SupportedEncoding {
    UTF8,
    ASCII,
}

pub struct Encoder {
    pub default_encoding: SupportedEncoding,
    trap: EncoderTrap
}

pub struct Decoder {
    pub default_encoding: SupportedEncoding,
    trap: DecoderTrap
}

impl Encoder {
    pub fn new(default_encoding: SupportedEncoding) -> Encoder {
        Encoder {
            default_encoding,
            trap: EncoderTrap::Replace,
        }
    }
    
    pub fn encode<'a>(self, content: &str, buffer: &mut Vec<u8>) -> Option<Cow<'a, str>> {
        let encode_result = match self.default_encoding {
            SupportedEncoding::UTF8  =>  UTF8Encoding.encode(content, self.trap),
            SupportedEncoding::ASCII => ASCIIEncoding.encode(content, self.trap),
        };
        match encode_result {
            Ok(s) => {
                buffer.append(&mut s.to_vec());
                None
            },
            Err(s) => {
                Option::from(s)
            },
        }
    }
}

impl Decoder {
    pub fn new(default_encoding: SupportedEncoding) -> Decoder {
        Decoder {
            default_encoding,
            trap: DecoderTrap::Replace,
        }
    }
    pub fn decode<'a>(self, content: &[u8], buffer: &mut String) -> Option<Cow<'a, str>> {
        //! 만약 오류가 발생했을 경우 오류 메시지를 Cow<'static, str> 타입으로 반환, 아닐 경우 None
        let decode_result = match self.default_encoding {
            SupportedEncoding::UTF8  =>  UTF8Encoding.decode(content, self.trap),
            SupportedEncoding::ASCII => ASCIIEncoding.decode(content, self.trap),
        };
        match decode_result {
            Ok(s) => {
                buffer.push_str(s.as_str());
                None
            },
            Err(s) => {
                Option::from(s)
            },
        }
    }
}
