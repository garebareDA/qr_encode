use std::fmt::Debug;

use crate::error::Error;
use kanji;
use regex::Regex;

#[derive(Debug)]
pub enum EncodeModeType {
    Numeric = 0001,
    Alphanumeric = 0010,
    Byte = 0100,
    Kanji = 1000,
}

#[derive(Debug)]
struct EncodeMode {
    char: char,
    mode: Vec<EncodeModeType>,
}

pub struct Encoded {
    mode: EncodeModeType,
    bits: Vec<i32>,
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

    pub fn get_char(&self) -> &char {
        &self.char
    }

    pub fn push(&mut self, mode: EncodeModeType) {
        self.mode.push(mode);
    }
}

impl Encoded {
    pub fn new(mode: EncodeModeType, bits: Vec<i32>) -> Encoded {
        Encoded {
            mode: mode,
            bits: bits,
        }
    }

    pub fn get_bits(&self) -> &Vec<i32> {
        &self.bits
    }
}

pub fn qr_encode_mode(str: &str) -> Result<Encoded, Error> {
    let list = qr_encode_mode_select(str);
    return qr_bits_encode(list);
}

fn qr_encode_mode_select(str: &str) -> Vec<EncodeMode> {
    let mut encode_mode_list: Vec<EncodeMode> = Vec::new();
    let alphabet_regex = Regex::new(r"^([a-z][A-Z]+)").unwrap();
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

fn qr_bits_encode(list: Vec<EncodeMode>) -> Result<Encoded, Error> {
    let encode_mode = list.get(0).unwrap().get_encoding_mode().get(0).unwrap();
    match encode_mode {
        EncodeModeType::Numeric => return qr_bits_encode_numeric(list),
        EncodeModeType::Alphanumeric => return Err(Error::InvalidEncode("Alphanumeric not supported".to_string())),
        EncodeModeType::Kanji => return Err(Error::InvalidEncode("Kanji not supported".to_string())),
        EncodeModeType::Byte => return Err(Error::InvalidEncode("Byte not supported".to_string())),
    }
}

fn qr_bits_encode_numeric(list: Vec<EncodeMode>) -> Result<Encoded, Error> {
    let r = list.len() / 3;
    let c = list.len() % 3;
    let mut bits: Vec<i32> = Vec::new();
    if r != 0 {
        for i in 0..r {
            let i = i * 3;
            let c1 = *list.get(i).unwrap().get_char() as i32 - 48;
            let c2: i32 = *list.get(i + 1).unwrap().get_char() as i32 - 48;
            let c3: i32 = *list.get(i + 2).unwrap().get_char() as i32 - 48;
            bits.push(c1 * 100 + c2 * 10 + c3);
        }
    }

    if c == 1 {
        let c1 = *list.last().unwrap().get_char() as i32 - 48;
        bits.push(c1);
    } else if c == 2 {
        let c1 = *list.get(list.len() - 2).unwrap().get_char() as i32 - 48;
        let c2 = *list.last().unwrap().get_char() as i32 - 48;
        bits.push(c1 * 10 + c2);
    }
    let encoded = Encoded::new(EncodeModeType::Numeric, bits);
    return Ok(encoded);
}
