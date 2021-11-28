use kanji;
use regex::Regex;

enum EncodeModeType {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
}

struct EncodeMode {
    char: char,
    mode: Vec<EncodeModeType>,
    bits: u8,
}

impl EncodeMode {
    pub fn new(char: char, mode: Vec<EncodeModeType>) -> EncodeMode {
        EncodeMode {
            char: char,
            mode: mode,
            bits: 0,
        }
    }

    pub fn set_bits(&mut self, bits: u8) {
        self.bits = bits;
    }

    pub fn get_encoding_mode(&self) -> &Vec<EncodeModeType> {
        &self.mode
    }

    pub fn push(&mut self, mode: EncodeModeType) {
        self.mode.push(mode);
    }
}

pub(crate) fn qr_encode_mode(str: &str) {
    let mut list = qr_encode_mode_select(str);
    qr_bits_encode(&mut list);
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

fn qr_bits_encode(mut list: &mut Vec<EncodeMode>) {
    for l in list {
        for mode in l.get_encoding_mode() {
            match mode {
                EncodeModeType::Numeric => {
                    println!("{}", l.char);
                }
                EncodeModeType::Alphanumeric => {
                    println!("{}", l.char);
                }
                EncodeModeType::Byte => {
                    println!("{}", l.char);
                }
                EncodeModeType::Kanji => {
                    println!("{}", l.char);
                }
            }
        }
    }
}
