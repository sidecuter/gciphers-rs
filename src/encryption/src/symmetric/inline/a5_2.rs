use std::error::Error;
use crate::methods::{bytes_to_hex, bytes_to_string, hex_to_bytes, str_to_bytes};
use super::{process, Register, RegIter, System};
use super::{to_64};

const MASK_R4_10: u32 = 0x400; 
const MASK_R4_3: u32 = 0x8; 
const MASK_R4_7: u32 = 0x80; 

struct Sys {
    r1: Register,
    r2: Register,
    r3: Register,
    r4: Register,
    r1_mag: u32,
    r2_mag: u32,
    r3_mag: u32,
    r4_mag: u32,
    cadr: u32,
    key: u64
}

fn get_bit(x: u32) -> u32 {
    if x != 0 { 1 } else { 0 }
}

fn mag(l: u32, r: u32, size: u8) -> u32 {
    let ri = RegIter::new(size, l & r, 1 << (size - 1));
    let sum: u32 = ri.filter(|x| *x == 1).map(|x| x as u32).sum();
    if sum >= 2 { 1 } else { 0 }
}

impl Sys {
    fn new(key: u64) -> Self {
        let r1 = Register::new(19, 0x72000, 1<<8);
        let r2 = Register::new(22, 0x300000, 1<<10);
        let r3 = Register::new(23, 0x700080, 1<<10);
        let r4 = Register::new(17, 0x10800, 1<<10);
        let cadr = 0;
        Self {
            r1, r2, r3, r4, cadr, key,
            r1_mag: 0xd000, r2_mag: 0x12200,
            r3_mag: 0x52000, r4_mag: 0x488
        }
    }

    fn drop(&mut self) {
        self.r1.drop();
        self.r2.drop();
        self.r3.drop();
        self.r4.drop();
    }

    fn step(&mut self) -> u8 {
        let m = mag(self.r4.value, self.r4_mag, 17);
        let m1 = get_bit(self.r4.value&MASK_R4_10) == m;
        let m2 = get_bit(self.r4.value&MASK_R4_3) == m;
        let m3 = get_bit(self.r4.value&MASK_R4_7) == m;
        self.r4.proto_shift(0);
        self.r1.shift_b(m1) ^ self.r2.shift_b(m2) ^ self.r3.shift_b(m3) ^
        mag(self.r1.value, self.r1_mag, 19) as u8 ^
        mag(self.r2.value, self.r2_mag, 22) as u8 ^
        mag(self.r3.value, self.r3_mag, 23) as u8
    }

    fn fill_key(&mut self) {
        for key_bit in RegIter::new(64, self.key, 1 << 63) {
            self.r1.proto_shift(key_bit);
            self.r2.proto_shift(key_bit);
            self.r3.proto_shift(key_bit);
            self.r4.proto_shift(key_bit);
        }
    }

    fn fill_cadr(&mut self) {
        for cadr_bit in RegIter::new(22, self.cadr, 1 << 21) {
            self.r1.proto_shift(cadr_bit);
            self.r2.proto_shift(cadr_bit);
            self.r3.proto_shift(cadr_bit);
            self.r4.proto_shift(cadr_bit);
        }
    }
}

impl System for Sys {
    fn magority(&self) -> u8 {
        0
    }

    fn prepare(&mut self) {
        self.drop();
        self.fill_key();
        self.fill_cadr();
        self.r4.value |= self.r4_mag;
        for _ in 0..99 {
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
    println!("{:?}", phrase);
    println!("{:?}", hex_to_bytes(key, 8));
    let r = proto(&phrase, key)?;
    Ok(bytes_to_hex(&r))
}

pub fn decrypt(phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let phrase = hex_to_bytes(phrase, 1)?;
    let r = proto(&phrase, key)?;
    bytes_to_string(&r)
}

#[cfg(test)]
mod a5_2_tests {
    use super::*;

    #[test]
    fn test_enc() {
        let phrase = "ото";
        let key = "ffeeddcc77665544";
        let result = encrypt(phrase, key).unwrap();
        assert_eq!(result, "9b84cdab0f44");
    }

    #[test]
    fn test_dec() {
        let phrase = "9b84cdab0f44";
        let key = "ffeeddcc77665544";
        let result = decrypt(phrase, key).unwrap();
        assert_eq!(result, "ото");
    }
}
