use std::error::Error;
use crate::methods::{bytes_to_hex, hex_to_bytes, modd, str_to_bytes};

const S_TABLE: [[u8; 16]; 8] = [
    [1,7,14,13,0,5,8,3,4,15,10,6,9,12,11,2],
    [8,14,2,5,6,9,1,12,15,4,11,0,13,10,3,7],
    [5,13,15,6,9,2,12,10,11,7,8,1,4,3,14,0],
    [7,15,5,10,8,1,6,13,0,9,3,14,11,4,2,12],
    [12,8,2,1,13,4,15,6,7,0,10,5,3,14,9,11],
    [11,3,5,8,2,15,10,13,14,1,7,4,12,9,6,0],
    [6,8,2,3,9,10,5,12,1,14,4,7,11,13,0,15],
    [12,4,6,2,10,5,11,9,14,8,13,7,0,3,15,1]
];

pub fn t (in_data: &[u8]) -> Vec<u8> {
    in_data[0..4].iter().enumerate().map(|(i, number)| {
        let first_part_byte = (*number & 0xf0) >> 4;
        let sec_part_byte = *number & 0x0f;
        let first_part_byte = S_TABLE[i * 2][first_part_byte as usize];
        let sec_part_byte = S_TABLE[i * 2 + 1][sec_part_byte as usize];
        (first_part_byte << 4) | sec_part_byte
    }).collect()
}

pub fn t_reverse(in_data: &[u8]) -> Vec<u8> {
    in_data[0..4].iter().enumerate().map(|(i, number)| {
        let first_part_byte = (*number & 0xf0) >> 4;
        let sec_part_byte = *number & 0x0f;
        let first_part_byte = S_TABLE[i * 2].iter()
            .position(|x| *x == first_part_byte).unwrap() as u8;
        let sec_part_byte = S_TABLE[i * 2 + 1].iter()
            .position(|x| *x == sec_part_byte).unwrap() as u8;
        (first_part_byte << 4) | sec_part_byte
    }).collect()
}

fn to_32(vec: &[u8]) -> u32 {
    let mut shift: i32 = 24;
    vec.iter().map(|num| {
        let result = (*num as u32) << shift;
        shift -= 8;
        result
    }).sum()
}

fn from_32(num: u32) -> Vec<u8> {
    vec![
        ((num >> 24) & 0xff) as u8,
        ((num >> 16) & 0xff) as u8,
        ((num >> 8) & 0xff) as u8,
        (num & 0xff) as u8
    ]
}

fn add_32(left: &[u8], right: &[u8]) -> Vec<u8> {
    let left_32: u32 = to_32(left);
    let right_32: u32 = to_32(right);
    let result_32 = ((left_32 as u64 + right_32 as u64) % 0x100000000u64) as u32;
    from_32(result_32)
}

pub fn g(key: &[u8], a: &[u8]) -> Vec<u8> {
    let internal = add_32(a, key);
    let internal = t(&internal);
    let mut result_32 = to_32(&internal);
    result_32 = (result_32 << 11) | (result_32>>21);
    from_32(result_32)
}

fn xor_32(left: &[u8], right: &[u8]) -> Vec<u8> {
    left.iter().zip(right.iter()).map(|(left, right)| *left ^ *right).collect()
}

fn expand_key(key: &[u8]) -> Vec<&[u8]> {
    let mut result: Vec<&[u8]> = (0..24).map(|i| {
        let i1 = modd(i*4, 32);
        let i2 = modd(i*4+4, 32);
        let i2 = if i2 == 0 { i2 + 32 } else { i2 };
        &key[i1..i2]
    }).collect();
    result.append(&mut (0..=7).rev().map(
        |i| &key[i*4..i*4+4]
    ).collect::<Vec<&[u8]>>());
    result
}

pub fn feistel_net_node(left: &[u8], right: &[u8], key: &[u8]) -> (Vec<u8>, Vec<u8>) {
    (right.to_vec(), xor_32(left, &g(right, key)))
}

fn feistel_net_32(val: &[u8], keys: &[&[u8]]) -> Vec<u8> {
    let mut left: Vec<u8> = val[0..4].to_vec();
    let mut right: Vec<u8> = val[4..8].to_vec();
    let mut key = keys[0..32].iter();
    (right, left) = loop {
        (left, right) = match key.next() {
            Some(key) => feistel_net_node(&left, &right, key),
            None => break (left, right)
        };
    };
    let mut result = left;
    result.append(&mut right);
    result
}

fn proto(phrase: &str, keys: &[&[u8]]) ->Result<String, Box<dyn Error>> {
    let phrase = hex_to_bytes(phrase, 8)?;
    let mut result = String::new();
    for fragment in phrase.windows(8).step_by(8) {
        let fragment = feistel_net_32(fragment, keys);
        result.push_str(&bytes_to_hex(&fragment));
    }
    Ok(result)
}

pub fn encrypt(phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let key = hex_to_bytes(key, 32)?;
    let expanded_keys = expand_key(&key);
    proto(phrase, &expanded_keys)
}

pub fn decrypt(phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let key = hex_to_bytes(key, 32)?;
    let mut expanded_keys = expand_key(&key);
    expanded_keys.reverse();
    proto(phrase, &expanded_keys)
}

pub fn prepair_phrase(phrase: &str) -> Result<String, Box<dyn Error>>{
    Ok(bytes_to_hex(&str_to_bytes(phrase, 8)?))
}

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
    let mut ctr: Vec<u8> = init_v.to_vec();
    ctr.extend(vec![0x00u8; 4]);
    let key = hex_to_bytes(key, 32)?;
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

#[cfg(test)]
mod magma_tests {
    use crate::methods::{str_to_bytes, bytes_to_hex, hex_to_bytes, bytes_to_string};
    use super::*;

    fn init_data() -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
        let l1: Vec<Vec<u8>> = vec![
            vec![ 0xfd, 0xb9, 0x75, 0x31 ],
            vec![ 0x2a, 0x19, 0x6f, 0x34 ],
            vec![ 0xeb, 0xd9, 0xf0, 0x3a ],
            vec![ 0xb0, 0x39, 0xbb, 0x3d ]
        ];
        let l2: Vec<Vec<u8>> = vec![
            vec![ 0x2a, 0x19, 0x6f, 0x34 ],
            vec![ 0xeb, 0xd9, 0xf0, 0x3a ],
            vec![ 0xb0, 0x39, 0xbb, 0x3d ],
            vec![ 0x68, 0x69, 0x54, 0x33 ]
        ];
        (l1, l2)
    }

    #[test]
    fn test_t() {
        let (data, validate) = init_data();
        for (values, results) in data.iter().zip(validate.iter()) {
            let result = t(values);
            assert_eq!(*results, result);
        }
    }

    #[test]
    fn test_t_reverse() {
        let (validate, data) = init_data();
        for (values, results) in data.iter().zip(validate.iter()) {
            let result = t_reverse(values);
            assert_eq!(*results, result);
        }
    }

    #[test]
    fn test_t_encrypt() {
        let bytes = str_to_bytes(
            "от одного порченого яблока весь воз загнивает.", 4
        ).unwrap();
        let mut result = String::new();
        for i in (0..bytes.len()).step_by(4) {
            let buffer: Vec<u8> = t(&bytes[i..i+4]).into_iter().collect();
            result.push_str(&bytes_to_hex(&buffer));
        }
        assert_eq!(result, "c812e316e83756dc663759dc633758dc63f7eb71c812e31ccebdeb75c814eb7fc81aeb7fe83f70dc6e3754dc633757dc68f7eb76c811e314cebb2bdc623756dc6cf7eb79c817eb72c814eb7ec815eb7cc811e316e357cb6c");
    }

    #[test]
    fn test_t_decrypt() {
        let bytes = hex_to_bytes(
            "c812e316e83756dc663759dc633758dc63f7eb71c812e31ccebdeb75c814eb7fc81aeb7fe83f70dc6e3754dc633757dc68f7eb76c811e314cebb2bdc623756dc6cf7eb79c817eb72c814eb7ec815eb7cc811e316e357cb6c",
            4
        ).unwrap();
        let mut data = Vec::new();
        for i in (0..bytes.len()).step_by(4) {
            let mut buffer: Vec<u8> = t_reverse(&bytes[i..i+4]).into_iter().collect();
            data.append(&mut buffer);
        }
        let result = bytes_to_string(&data).unwrap();
        assert_eq!(result, "от одного порченого яблока весь воз загнивает.");
    }

    #[test]
    fn test_g() {
        let keys: Vec<Vec<u8>> = vec![
            vec![ 0x87, 0x65, 0x43, 0x21 ],
            vec![ 0xfd, 0xcb, 0xc2, 0x0c ],
            vec![ 0x7e, 0x79, 0x1a, 0x4b ],
            vec![ 0xc7, 0x65, 0x49, 0xec ]
        ];
        let validate: Vec<Vec<u8>> = vec![
            vec![ 0xfd, 0xcb, 0xc2, 0x0c ],
            vec![ 0x7e, 0x79, 0x1a, 0x4b ],
            vec![ 0xc7, 0x65, 0x49, 0xec ],
            vec![ 0x97, 0x91, 0xc8, 0x49 ]
        ];
        let data: Vec<Vec<u8>> = vec![
            vec![ 0xfe, 0xdc, 0xba, 0x98 ],
            vec![ 0x87, 0x65, 0x43, 0x21 ],
            vec![ 0xfd, 0xcb, 0xc2, 0x0c ],
            vec![ 0x7e, 0x79, 0x1a, 0x4b ],
        ];
        for (i , datum) in data.iter().enumerate() {
            assert_eq!(validate.get(i).unwrap(), &g(keys.get(i).unwrap(), datum));
        }
    }

    #[test]
    fn test_feistel_net_node() {
        let key: Vec<u8> = vec![ 0x87, 0x65, 0x43, 0x21 ];
        let left: Vec<u8> = vec![ 0xfe, 0xdc, 0xba, 0x98 ];
        let right: Vec<u8> = vec![ 0xfd, 0xcb, 0xc2, 0x0c ];
        let (right_r, left_r) = feistel_net_node(&left, &right, &key);
        let (r, l) = feistel_net_node(&left_r, &right_r, &key);
        assert_eq!((left, right), (l, r));
    }

    #[test]
    fn test_expand_key() {
        let validate: Vec<&str> = vec![
            "ffeeddcc", "bbaa9988", "77665544", "33221100", "f0f1f2f3", "f4f5f6f7", "f8f9fafb", "fcfdfeff",
            "ffeeddcc", "bbaa9988", "77665544", "33221100", "f0f1f2f3", "f4f5f6f7", "f8f9fafb", "fcfdfeff",
            "ffeeddcc", "bbaa9988", "77665544", "33221100", "f0f1f2f3", "f4f5f6f7", "f8f9fafb", "fcfdfeff",
            "fcfdfeff", "f8f9fafb", "f4f5f6f7", "f0f1f2f3", "33221100", "77665544", "bbaa9988", "ffeeddcc",
        ];
        let key = hex_to_bytes(
            "ffeeddccbbaa99887766554433221100f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
            32
        ).unwrap();
        let mut val = validate.iter();
        for key in expand_key(&key) {
            assert_eq!(*val.next().unwrap(), bytes_to_hex(key));
        }
    }

    #[test]
    fn test_feistel_net_32() {
        let key = hex_to_bytes(
            "ffeeddccbbaa99887766554433221100f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
            32
        ).unwrap();
        let val = hex_to_bytes("fedcba9876543210", 8).unwrap();
        let result = bytes_to_hex(&feistel_net_32(&val, &expand_key(&key)));
        assert_eq!(result, "4ee901e5c2d8ca3d");
    }

    #[test]
    fn test_encrypt() {
        let key = "ffeeddccbbaa99887766554433221100f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff";
        let phrase = "fedcba9876543210";
        assert_eq!(encrypt(phrase, key).unwrap(), "4ee901e5c2d8ca3d");
    }

    #[test]
    fn test_decrypt() {
        let key = "ffeeddccbbaa99887766554433221100f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff";
        let phrase = "4ee901e5c2d8ca3d";
        assert_eq!(decrypt(phrase, key).unwrap(), "fedcba9876543210");
    }
}
