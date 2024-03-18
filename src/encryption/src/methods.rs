use std::error::Error;
use hex::{encode, decode};
use std::str;
use crate::alphabet::Alphabet;
use crate::errors::NullSizedValue;

pub fn modd(num: isize, limit: usize) -> usize {
    let limit = limit as isize;
    if num < 0 {
        ((limit - (-num)) % limit) as usize
    } else {
        (num % limit) as usize
    }
}

pub fn hex_to_str(text: &str) -> Result<String, Box<dyn Error>> {
    bytes_to_string(&hex_to_bytes(text, 1)?)
}

pub fn str_to_hex(text: &str, border: usize) -> String {
    bytes_to_hex(&str_to_bytes(text, border).unwrap())
}

pub fn str_to_bytes(text: &str, border: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    let bytes = hex_to_bytes(&encode(text), border)?;
    Ok(bytes)
}

pub fn bytes_to_string(buffer: &[u8]) -> Result<String, Box<dyn Error>> {
    let result = str::from_utf8(buffer)?.to_owned();
    Ok(result.trim_matches('\u{0000}').to_owned())
}

pub fn bytes_to_hex(buffer: &[u8]) -> String {
    encode(buffer)
}

pub fn hex_to_bytes(text: &str, border: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bytes_vec = decode(text)?;
    let null_count = bytes_vec.len() % border;
    if null_count > 0 {
        let null = vec![0_u8; border - null_count];
        bytes_vec.extend(null);
    }
    Ok(bytes_vec)
}

pub fn validate_single(alphabet: &Alphabet, phrase: &str) -> Result<(), Box<dyn Error>> {
    if phrase.is_empty() { Err(NullSizedValue::new("Фраза"))?; }
    alphabet.validate(phrase)
}

pub fn validate_two(alphabet: &Alphabet, text: &str, key: &str) -> Result<(), Box<dyn Error>> {
    if text.is_empty() { Err(NullSizedValue::new("Фраза"))?; }
    if key.is_empty() { Err(NullSizedValue::new("Ключ"))?; }
    alphabet.validate(text)?;
    alphabet.validate(key)
}

pub fn transform(val: &str, message: &str) -> Result<isize, Box<dyn Error>> {
    if val.is_empty() { Err(NullSizedValue::new(message))?;}
   Ok(val.parse::<isize>()?)
}

#[cfg(test)]
mod method_tests {
    use crate::methods::{bytes_to_hex, bytes_to_string, hex_to_bytes, str_to_bytes};

    #[test]
    fn test_bytes_to_hex() {
        let buf: Vec<u8> = vec![120, 215];
        let valid = "78d7".to_string();
        let result = bytes_to_hex(&buf);
        assert_eq!(result, valid);
    }

    #[test]
    fn test_hex_to_bytes() {
        let valid: Vec<u8> = vec![120, 215];
        let buf = "78d7".to_string();
        let result = hex_to_bytes(&buf, 2).unwrap();
        assert_eq!(result, valid);
    }

    #[test]
    fn test_str_to_bytes() {
        let test = "а";
        let res = str_to_bytes(test, 4).unwrap();
        let valid: Vec<u8> = vec![208, 176, 0, 0];
        assert_eq!(res, valid);
    }

    #[test]
    fn test_bytes_to_string() {
        let test: Vec<u8> = vec![208, 176, 0, 0];
        let res = bytes_to_string(&test).unwrap();
        let valid = String::from("а");
        assert_eq!(res, valid);
    }

    #[test]
    #[should_panic]
    fn test_hex_to_bytes_panic() {
        let buf = "78g7".to_string();
        hex_to_bytes(&buf, 2).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_bytes_to_string_panic() {
        let test: Vec<u8> = vec![0, 176, 0, 0];
        bytes_to_string(&test).unwrap();
    }
}