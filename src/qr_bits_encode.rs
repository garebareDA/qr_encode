use kanji;
use regex::Regex;
use crate::error::Error;

enum EncodeModeType {
    Numeric = 0001,
    Alphanumeric = 0010,
    Byte = 0100,
    Kanji = 1000,
}

struct EncodeMode {
    char: char,
    mode: Vec<EncodeModeType>,
}

struct Encoded {
    string: String,
    mode: EncodeModeType,
    bits: Vec<u8>,
}

impl EncodeMode {
    pub fn new(char: char, mode: Vec<EncodeModeType>) -> EncodeMode {
        EncodeMode {
            char: char,
            mode: mode,
        }
    }

    pub fn get_encoding_mode(&self) -> &Vec<EncodeModeType> {
        &self.mode
    }

    pub fn push(&mut self, mode: EncodeModeType) {
        self.mode.push(mode);
    }
}

pub(crate) fn qr_encode_mode(str: &str) -> Result<(), Error> {
    let list = qr_encode_mode_select(str);
    qr_bits_encode(&list);
    return Ok(());
}

fn qr_encode_mode_select(str: &str) -> Vec<EncodeMode> {
    let mut encode_mode_list: Vec<EncodeMode> = Vec::new();
    let alphabet_regex = Regex::new(r"^[a-zA-Z $%*+\-./:={}[]<>!?]*$").unwrap();
    let numeric_regex = Regex::new(r"^[0-9]*$").unwrap();
    for c in str.chars() {
        let mut mode = EncodeMode::new(c, Vec::new());
        if alphabet_regex.is_match(c.to_string().as_str()) || c == ' ' {
            mode.push(EncodeModeType::Alphanumeric);
        }
        if numeric_regex.is_match(c.to_string().as_str()) {
            mode.push(EncodeModeType::Numeric);
        }
        if kanji::is_hiragana(&c) || kanji::is_katakana(&c) || kanji::is_kanji(&c) {
            mode.push(EncodeModeType::Kanji);
        }
        mode.push(EncodeModeType::Byte);
        encode_mode_list.push(mode);
    }
    return encode_mode_list;
}

fn qr_bits_encode(list: &Vec<EncodeMode>) -> Result<(), Error> {
    

    return Ok(());
}

fn qr_bits_encode_numeric(str: &str) {
    let mut list = qr_encode_mode_select(str);
}
