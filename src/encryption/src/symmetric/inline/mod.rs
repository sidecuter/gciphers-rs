use std::ops::{BitAnd, ShrAssign};

pub mod a5_1;
pub mod a5_2;

fn to_64(vec: &[u8]) -> u64 {
    let mut shift: i64 = 56;
    vec.iter().map(|num| {
        let result = (*num as u64) << shift;
        shift -= 8;
        result
    }).sum()
}

#[derive(Clone)]
struct RegIter<T>
    where T: num::Integer + Copy + Clone + Eq + BitAnd<Output=T> + ShrAssign + num::FromPrimitive
{
    size: u8,
    and_val: T,
    value: T
}

impl<T> RegIter<T>
    where T: num::Integer + Copy + Clone + Eq + BitAnd<Output=T> + ShrAssign + num::FromPrimitive
{
    fn new(size: u8, value: T, and_val: T) -> Self {
        Self {
            size, value, and_val
        }
    }
}

impl<T> Iterator for RegIter<T>
    where T: num::Integer + Copy + Clone + Eq + BitAnd<Output=T> + ShrAssign + num::FromPrimitive
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size != 0 {
            self.size -= 1;
            let result: T = self.and_val & self.value;
            self.and_val >>= T::from_i8(1).unwrap();
            Some(if result != T::from_i8(0).unwrap() { 1 } else { 0 })
        } else { None }
    }
}

struct Register {
    size: u8,
    scrambler: u32,
    value: u32,
    control: u32
}

trait System {
    fn magority(&self) -> u8;

    fn prepare(&mut self);

    fn takt(&mut self) -> u128;

    fn process(&mut self, phrase_len_bits: usize) -> Vec<RegIter<u128>> {
        let mut res = Vec::new();
        let ost = phrase_len_bits % 114;
        let count = if ost != 0 {
            phrase_len_bits / 114 + 1
        } else { phrase_len_bits / 114 };
        let ost = if ost == 0 { 114 } else { ost };
        for i in 0..count {
            let left = self.takt();
            if i == count - 1 {
                res.push(
                    RegIter::new(ost as u8, left, 0x20000000000000000000000000000)
                );
            } else {
                res.push(
                    RegIter::new(114, left, 0x20000000000000000000000000000)
                );
            }
        }
        res
    }
}

impl Register {
    fn new(size: u8, scrambler: u32, control: u32) -> Self {
        Self { size, scrambler, value: 0, control }
    }

    fn drop(&mut self) {
        self.value = 0;
    }

    fn get_iter(&self, pos: u8) -> RegIter<u32> {
        let and_unit: u32 = 1 << (self.size - 1);
        match pos {
            1 => RegIter::new(self.size, self.scrambler, and_unit),
            2 => RegIter::new(self.size, self.value, and_unit),
            _ => panic!("Нет такой опции")
        }
    }

    fn proto_shift(&mut self, xor: u8) -> u8 {
        let and_unit: u32 = 1 << (self.size - 1);
        let mut new_value = if self.value & and_unit != 0 { 1 } else { 0 };
        let ret_value = new_value;
        for (s_bit, v_bit) in self.get_iter(1).zip(self.get_iter(2)).skip(1) {
            if s_bit != 0 {
                new_value ^= v_bit;
            }
        }
        new_value ^= xor;
        self.value <<= 1;
        self.value |= new_value as u32;
        self.value &= (and_unit << 1) - 1;
        ret_value
    }

    fn shift_m(&mut self, magority: u8) -> u8 {
        if magority != self.get_control_bit() {
            if self.value & (1 << (self.size - 1)) != 0 { 1 } else { 0 }
        }
        else { self.proto_shift(0) }
    }

    fn shift_b(&mut self, con: bool) -> u8 {
        if !con {
            if self.value & (1 << (self.size - 1)) != 0 { 1 } else { 0 }
        }
        else { self.proto_shift(0) }
    }

    fn get_control_bit(&self) -> u8 {
        if self.value & self.control != 0 { 1 } else { 0 }
    }
}

struct BitIter {
    value: u128,
    mask: u128,
    find: bool
}

impl Iterator for BitIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.find {
            if self.value & self.mask != 0 {
                self.find = true;
            }
            self.mask >>= 1;
        }
        if self.mask == 0 { return None; }
        let result = if self.value & self.mask != 0 { 1 } else { 0 };
        Some(result)
    }
}

fn join_gamma(gamma: &[RegIter<u128>]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut val = 0;
    let mut length = 0;
    for item in gamma {
        for bit in (*item).clone() {
            if length == 8 {
                result.push(val);
                length = 0;
                val = 0;
            }
            length += 1;
            val = (val << 1) | bit;
        }
    }
    result.push(val);
    result
}

fn process(phrase: &[u8], gamma: &[RegIter<u128>]) -> Vec<u8> {
    let mut result = Vec::new();
    let gamma = join_gamma(gamma);
    for (left, right) in phrase.iter().zip(gamma.iter()) {
        result.push(left ^ right);
    }
    result
}
