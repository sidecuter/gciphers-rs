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
    // print!("0b{:0size$b}\t", l&r, size=size as usize);
    let ri = RegIter::new(size, l & r, 1 << (size - 1));
    let sum = ri.filter(|x| *x == 1).map(|x| x as u32).sum();
    /*let result = */get_bit(sum)/*;
    print!("{result}\t");
    result*/
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
        // print!("0b{:0size$b}\t", self.r1.value, size=self.r1.size as usize);
        // print!("0b{:0size$b}\t", self.r2.value, size=self.r2.size as usize);
        // print!("0b{:0size$b}\t", self.r3.value, size=self.r3.size as usize);
        // print!("0b{:0size$b}\t", self.r4.value, size=self.r4.size as usize);
        let m = mag(self.r4.value, self.r4_mag, 17);
        let m1 = get_bit(self.r4.value&MASK_R4_10) == m;
        let m2 = get_bit(self.r4.value&MASK_R4_3) == m;
        let m3 = get_bit(self.r4.value&MASK_R4_7) == m;
        // print!("{}\t", if m1 { 1 } else { 0 });
        // print!("{}\t", if m2 { 1 } else { 0 });
        // print!("{}\t", if m3 { 1 } else { 0 });
        self.r4.proto_shift(0);
        /*let result =*/ self.r1.shift_b(m1) ^ self.r2.shift_b(m2) ^ self.r3.shift_b(m3) ^
        mag(self.r1.value, self.r1_mag, 19) as u8 ^
        mag(self.r2.value, self.r2_mag, 22) as u8 ^
        mag(self.r3.value, self.r3_mag, 23) as u8/*;*/
        // println!("{result}");
        /*result*/
    }

    fn fill_key(&mut self) {
        //let key: Vec<u8> = RegIter::new(64, self.key, 1 << 63).collect();
        //for key_bit in key.into_iter().rev() {
        for key_bit in RegIter::new(64, self.key, 1 << 63) {
            self.r1.proto_shift(key_bit);
            self.r2.proto_shift(key_bit);
            self.r3.proto_shift(key_bit);
            self.r4.proto_shift(key_bit);
        }
    }

    fn fill_cadr(&mut self) {
        //let cadr: Vec<u8> = RegIter::new(22, self.cadr, 1 << 21).collect();
        //for cadr_bit in cadr.into_iter().rev() {
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
        //println!("\n\n\n\n");
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
mod a5_2_tests {
    use super::*;

    #[test]
    fn test_enc() {
        let phrase = "ото";
        let key = "ffeeddcc77665544";
        let result = encrypt(phrase, key).unwrap();
        assert_eq!(result, "752ffe26adc9");
    }

    #[test]
    fn test_dec() {
        let phrase = "752ffe26adc9";
        let key = "ffeeddcc77665544";
        let result = decrypt(phrase, key).unwrap();
        assert_eq!(result, "ото");
    }
}
