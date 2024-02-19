use std::{error::Error, u8};

use crate::magma::feistel_net_32;
use crate::methods::{bytes_to_hex, hex_to_bytes};
use crate::symmetric::combinational::magma::expand_key;

fn add_xor(left: &[u8], right: &[u8]) -> Vec<u8> {
    left.iter().zip(right.iter()).map(|(left, right)| *left ^ *right).collect()
}

fn inc_ctr(counter: &[u8]) -> Vec<u8> {
    let mut buffer: usize = 0;
    let bits: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
    counter.iter().rev().zip(bits.iter().rev()).map(|(elem, bit)| {
        buffer = *elem as usize + *bit as usize + (buffer >> 8);
        (buffer & 0xff) as u8
    }).collect()
}

fn null_len(vec: &[u8]) -> usize {
    let mut count = 0_usize;
    for elem in vec.iter().rev() {
        if *elem == 0 { count += 1; }
        else { break; }
    }
    count
}


pub fn ctr_magma(phrase: &str, init_v: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let init_v = hex_to_bytes(init_v, 4)?;
    let mut ctr: Vec<u8> = init_v.iter().map(
        |element| *element
    ).collect();
    ctr.extend(vec![0x00u8; 4]);
    let key = hex_to_bytes(&key, 32)?;
    let keys = expand_key(&key);
    let mut gamma: Vec<u8>;
    let phrase = hex_to_bytes(phrase, 8)?;
    let null_count = null_len(&phrase);
    let mut result_v: Vec<u8> = Vec::new();
    for part in phrase.windows(8).step_by(8) {
        gamma = feistel_net_32(&ctr, &keys);
        ctr = inc_ctr(&ctr);
        result_v.extend(add_xor(part, &gamma));
    }
    Ok(bytes_to_hex(&result_v[0..result_v.len() - null_count]))
}
