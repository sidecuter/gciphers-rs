use std::error::Error;
use crate::methods::{bytes_to_hex, bytes_to_string, hex_to_bytes, str_to_bytes};
use super::{process, Register, RegIter};
use super::{System, to_64};

struct Sys {
    r1: Register,
    r2: Register,
    r3: Register,
    cadr: u32,
    key: u64
}

impl Sys {
    fn new(key: u64) -> Self {
        let r1 = Register::new(19, 0x72000, 1<<8);
        let r2 = Register::new(22, 0x300000, 1<<10);
        let r3 = Register::new(23, 0x700080, 1<<10);
        let cadr = 0;
        Self { r1, r2, r3, cadr, key }
    }

    fn drop(&mut self) {
        self.r1.drop();
        self.r2.drop();
        self.r3.drop();
    }

    fn step(&mut self) -> u8 {
        let m = self.magority();
        self.r1.shift_m(m) ^ self.r2.shift_m(m) ^ self.r3.shift_m(m)
    }

    fn fill_key(&mut self) {
        //let key: Vec<u8> = RegIter::new(64, self.key, 1 << 63).collect();
        //for key_bit in key.into_iter().rev() {
        for key_bit in RegIter::new(64, self.key, 1 << 63) {
            self.r1.proto_shift(key_bit);
            self.r2.proto_shift(key_bit);
            self.r3.proto_shift(key_bit);
        }
    }

    fn fill_cadr(&mut self) {
        //let cadr: Vec<u8> = RegIter::new(22, self.cadr, 1 << 21).collect();
        //for cadr_bit in cadr.into_iter().rev() {
        for cadr_bit in RegIter::new(22, self.cadr, 1 << 21) {
            self.r1.proto_shift(cadr_bit);
            self.r2.proto_shift(cadr_bit);
            self.r3.proto_shift(cadr_bit);
        }
    }
}

impl System for Sys {
    fn magority(&self) -> u8 {
        let x = self.r1.get_control_bit();
        let y = self.r2.get_control_bit();
        let z = self.r3.get_control_bit();
        (x&y)|(x&z)|(y&z)
    }

    fn prepare(&mut self) {
        self.drop();
        self.fill_key();
        self.fill_cadr();
        for _ in 0..100 {
            self.step();
        }
    }

    fn takt(&mut self) -> u128 {
        let mut res: u128 = 0;
        self.prepare();
        for _ in 0..114 {
            res <<= 1;
            res |= self.step() as u128;
        }
        self.cadr += 1;
        res
    }
}

fn proto(phrase: &[u8], key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let key = to_64(&hex_to_bytes(key, 8)?);
    let mut sys = Sys::new(key);
    let buffer = sys.process(phrase.len() * 8);
    let proc = process(phrase, &buffer);
    Ok(proc)
}

pub fn encrypt(phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let phrase = str_to_bytes(phrase, 1)?;
    let r = proto(&phrase, key)?;
    Ok(bytes_to_hex(&r))
}

pub fn decrypt(phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let phrase = hex_to_bytes(phrase, 1)?;
    let r = proto(&phrase, key)?;
    bytes_to_string(&r)
}

#[cfg(test)]
mod a5_1_tests {
    use super::*;
    
    #[test]
    fn test_create_sys() {
        let s = Sys::new(0);
        assert_eq!(0b1110010000000000000, s.r1.scrambler);
        assert_eq!(0b1100000000000000000000, s.r2.scrambler);
        assert_eq!(0b11100000000000010000000, s.r3.scrambler);
    }

    #[test]
    fn test_reg() {
        let mut r = Register::new(5, 0x16, 0x10);
        r.value = 0b10111;
        assert_eq!(r.get_control_bit(), 1);
        assert_eq!(r.proto_shift(0), 1);
        assert_eq!(r.value, 0b01111);
        assert_eq!(r.proto_shift(0), 0);
        assert_eq!(r.value, 0b11110);
    }
    
    #[test]
    fn test_enc() {
        let phrase = "ото";
        let key = "ffeeddcc77665544";
        let result = encrypt(phrase, key).unwrap();
        assert_eq!(result, "f48466990c59");
    }
    
    #[test]
    fn test_dec() {
        let phrase = "f48466990c59";
        let key = "ffeeddcc77665544";
        let result = decrypt(phrase, key).unwrap();
        assert_eq!(result, "ото");
    }
}
