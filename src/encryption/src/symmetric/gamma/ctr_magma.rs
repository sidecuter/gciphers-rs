use std::u8;

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
