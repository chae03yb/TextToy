#[cfg(test)]
mod tests {
    use crate::encoding::{Decoder, Encoder, SupportedEncoding};
    use super::*;

    #[test]
    fn string_push() {
        let mut content: Vec<String> = vec![ String::from("llo, world!") ];
        let mut cursor_col: usize = 0;
        let cursor_row: usize = 0;

        fn ac(c: &mut Vec<String>, row: usize, col: &mut usize, ch: char) {
            c[row].insert(*col, ch);
            *col += 1;
        }

        ac(&mut content, cursor_row, &mut cursor_col, 'H');
        ac(&mut content, cursor_row, &mut cursor_col, 'e');

        assert_eq!(2, cursor_col);
        assert_eq!(String::from("Hello, world!"), content[0]);
    }

    #[test]
    fn string_pop() {
        let mut content: Vec<String> = vec![ String::from("Hiello, world!") ];
        let mut cursor_col: usize = 1;
        let cursor_row: usize = 0;

        fn dc(c: &mut Vec<String>, row: usize, col: &mut usize) {
            c[row].remove(*col);
            *col -= 1;
        }

        dc(&mut content, cursor_row, &mut cursor_col);

        assert_eq!(0, cursor_col);
        assert_eq!(String::from("Hello, world!"), content[0]);
    }

    #[test]
    fn test_decoding() {
        let ascii_decoder = Decoder::new(SupportedEncoding::ASCII);
        let utf8_decoder = Decoder::new(SupportedEncoding::UTF8);
        let non_ascii_decoder = Decoder::new(SupportedEncoding::ASCII);
        
        let original_content = "Test String";
        let encoded_data = [0x54_u8, 0x65, 0x73, 0x74, 0x20, 0x53, 0x74, 0x72, 0x69, 0x6E, 0x67].as_ref();
        let non_ascii_data = [0x54_u8, 129, 150, 254, 95].as_ref();

        let mut result_utf8 = String::new();
        let mut result_ascii = String::new();
        let mut result_non_ascii_data = String::new();

        ascii_decoder.decode(encoded_data, &mut result_ascii);
        utf8_decoder.decode(encoded_data, &mut result_utf8);
        let err_msg = non_ascii_decoder.decode(non_ascii_data, &mut result_non_ascii_data);  // result: T���_
        println!("test_decoding_result: {:?}", result_non_ascii_data);  // cargo test -- --nocapture
        println!("test_decoding_err_msg: {:?}", err_msg);

        assert_eq!(result_ascii, original_content);
        assert_eq!(result_utf8, original_content);
        assert_ne!(result_non_ascii_data, original_content);
    }

    #[test]
    fn test_encoding() {
        let ascii_encoder = Encoder::new(SupportedEncoding::ASCII);
        let utf8_encoder = Encoder::new(SupportedEncoding::UTF8);
        let non_ascii_encoder = Encoder::new(SupportedEncoding::ASCII);
        
        let original_content = "Test String";
        let non_ascii_content = "테스트하다";
        let encoded_data = [0x54_u8, 0x65, 0x73, 0x74, 0x20, 0x53, 0x74, 0x72, 0x69, 0x6E, 0x67].as_ref();
        let non_ascii_data = [0xED_u8, 0x85, 0x8C, 0x8A, 0xA4, 0xED, 0x8A, 0xB8, 0xED, 0x95, 0x98, 0xEB, 0x8B, 0xA4].as_ref();

        let mut result_utf8: Vec<u8> = Vec::new();
        let mut result_ascii: Vec<u8> = Vec::new();
        let mut result_non_ascii: Vec<u8> = Vec::new();

        ascii_encoder.encode(original_content, &mut result_ascii);
        utf8_encoder.encode(original_content, &mut result_utf8);
        let err_msg = non_ascii_encoder.encode(non_ascii_content, &mut result_non_ascii);
        println!("test_encoding_err_msg: {:?}", err_msg);
        
        assert_eq!(result_utf8, encoded_data);
        assert_eq!(result_ascii, encoded_data);
        assert_ne!(result_non_ascii, non_ascii_data);
    }
}
