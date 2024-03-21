use std::error::Error;
use crate::methods::{bytes_to_hex, hex_to_bytes};

const NK: usize = 4;
const NB: usize = 4;
const NR: usize = 10;

const S: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

const S_REVERSE: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

const R_CON: [[u8; 4]; 10] = [
    [ 0x01, 0x00, 0x00, 0x00 ],
    [ 0x02, 0x00, 0x00, 0x00 ],
    [ 0x04, 0x00, 0x00, 0x00 ],
    [ 0x08, 0x00, 0x00, 0x00 ],
    [ 0x10, 0x00, 0x00, 0x00 ],
    [ 0x20, 0x00, 0x00, 0x00 ],
    [ 0x40, 0x00, 0x00, 0x00 ],
    [ 0x80, 0x00, 0x00, 0x00 ],
    [ 0x1b, 0x00, 0x00, 0x00 ],
    [ 0x36, 0x00, 0x00, 0x00 ],
];

const MIX: [u8; 4] = [0x02, 0x03, 0x01, 0x01];
const INV_MIX: [u8; 4] = [0x0e, 0x0b, 0x0d, 0x09];

fn key_expansion(key: &[u8]) -> Vec<Vec<u8>> {
    let mut key: Vec<u8> = key.to_vec();
    key.append(&mut vec![0x01u8; 4*NK-key.len()]);
    let mut result: Vec<Vec<u8>> = fill_state(&key);
    for col in NK..NB*(NR+1) {
        if col % NK == 0 {
            let mut tmp: Vec<u8> = (1..4usize).map(|row| result[row][col-1]).collect();
            tmp.push(result[0][col-1]);
            for item in tmp.iter_mut() {
                let sbox_elem =  S[*item as usize];
                *item = sbox_elem;
            }
            for row in 0..4 {
                let s = result[row][col - 4]^tmp[row]^R_CON[col/NK - 1][row];
                result[row].push(s)
            }
        } else {
            for row in result[0..4].iter_mut() {
                let s = row[col - 4]^row[col - 1];
                row.push(s);
            }
        }
    }
    result
}

fn add_round_key(state: &[Vec<u8>], key_schedule: &[Vec<u8>], round: usize) -> Vec<Vec<u8>> {
    let mut result = vec![
        vec![0; NK],
        vec![0; NK],
        vec![0; NK],
        vec![0; NK],
    ];
    for col in 0..NK {
        let s0 = state[0][col]^key_schedule[0][NB*round + col];
        let s1 = state[1][col]^key_schedule[1][NB*round + col];
        let s2 = state[2][col]^key_schedule[2][NB*round + col];
        let s3 = state[3][col]^key_schedule[3][NB*round + col];
        result[0][col] = s0;
        result[1][col] = s1;
        result[2][col] = s2;
        result[3][col] = s3;
    }
    result
}

fn sub_bytes(state: &[Vec<u8>], sbox: &[u8]) -> Vec<Vec<u8>> {
    state.iter().map(|line| (*line).iter().map(|item| sbox[*item as usize]
    ).collect()).collect()
}

fn left_shift(line: &[u8], count: usize) -> Vec<u8> {
    let mut result = line[count..].to_vec();
    if count != 0 {
        result.append(&mut line[0..count].to_vec());
    }
    result
}

fn right_shift(line: &[u8], count: usize) -> Vec<u8> {
    let mut result = line[line.len()-count..].to_vec();
    result.append(&mut line[0..line.len()-count].to_vec());
    result
}

fn shift_rows(state: &[Vec<u8>]) -> Vec<Vec<u8>> {
    state.iter().enumerate().map(|(i, line)| left_shift(line, i)).collect()
}

fn inv_shift_rows(state: &[Vec<u8>]) -> Vec<Vec<u8>> {
    state.iter().enumerate().map(|(i, line)| right_shift(line, i)).collect()
}

fn gf_mul(mut left: u8, mut right: u8) -> u8 {
    let mut result: u8 = 0;
    let mut hi_bit: u8;
    for _ in 0..8 {
        if right & 1 != 0 { result ^= left }
        hi_bit = left & 0x80;
        left <<= 1;
        if hi_bit != 0 { left ^= 0x1b }
        right >>= 1;
    }
    result
}

fn mix_columns(state: &[Vec<u8>], coef: &[u8]) -> Vec<Vec<u8>> {
    (0..4).map(|i| (0..NB).map(|j| {
        let mut s = 0;
        for (k, coef) in right_shift(coef, i).iter().enumerate() {
            s ^= gf_mul(state[k][j], *coef);
        }
        s
    }).collect()).collect()
}

fn fill_state(input: &[u8]) -> Vec<Vec<u8>> {
    (0..4).map(|r| (0..NB).map(|c| input[r+4*c]).collect()).collect()
}

fn fill_result(state: &[Vec<u8>]) -> Vec<u8> {
    let mut result = Vec::new();
    for r in 0..4 {
        result.append(&mut (0..NB).map(|c| state[c][r]).collect());
    }
    result
}

fn enc(input: &[u8], key_schedule: &[Vec<u8>]) -> Vec<u8> {
    let mut state = fill_state(input);
    state = add_round_key(&state, key_schedule, 0);
    for rnd in 1..NR {
        state = sub_bytes(&state, &S);
        state = shift_rows(&state);
        state = mix_columns(&state, &MIX);
        state = add_round_key(&state, key_schedule, rnd);
    }
    state = sub_bytes(&state, &S);
    state = shift_rows(&state);
    state = add_round_key(&state, key_schedule, NR);
    fill_result(&state)
}

fn dec(input: &[u8], key_schedule: &[Vec<u8>]) -> Vec<u8> {
    let mut state = fill_state(input);
    state = add_round_key(&state, key_schedule, NR);
    for rnd in (1..NR).rev() {
        state = inv_shift_rows(&state);
        state = sub_bytes(&state, &S_REVERSE);
        state = add_round_key(&state, key_schedule, rnd);
        state = mix_columns(&state, &INV_MIX);
    }
    state = inv_shift_rows(&state);
    state = sub_bytes(&state, &S_REVERSE);
    state = add_round_key(&state, key_schedule, 0);
    fill_result(&state)
}

fn proto<T>(input: &[u8], key: &str, func: T) -> Result<Vec<u8>, Box<dyn Error>>
    where T: Fn(&[u8], &[Vec<u8>]) -> Vec<u8>
{
    let key = hex_to_bytes(key, 16)?;
    let key_schedule = key_expansion(&key);
    let mut result = Vec::new();
    for part in input.windows(16).step_by(16) {
        result.append(&mut func(part, &key_schedule));
    }
    Ok(result)
}

pub fn encrypt(input: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let input = hex_to_bytes(input, 16)?;
    Ok(bytes_to_hex(&proto(&input, key, enc)?))
}

pub fn decrypt(input: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let input = hex_to_bytes(input, 16)?;
    Ok(bytes_to_hex(&proto(&input, key, dec)?))
}

#[cfg(test)]
mod aes_tests {
    use super::*;
    use crate::methods::{hex_to_str, str_to_hex};

    #[test]
    fn test_key_expansion() {
        let key: [u8; 16] = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
            0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        let validate: [[u8; 4]; 44] = [
            [0x2b, 0x7e, 0x15, 0x16],
            [0x28, 0xae, 0xd2, 0xa6],
            [0xab, 0xf7, 0x15, 0x88],
            [0x09, 0xcf, 0x4f, 0x3c],
            [0xa0, 0xfa, 0xfe, 0x17],
            [0x88, 0x54, 0x2c, 0xb1],
            [0x23, 0xa3, 0x39, 0x39],
            [0x2a, 0x6c, 0x76, 0x05],
            [0xf2, 0xc2, 0x95, 0xf2],
            [0x7a, 0x96, 0xb9, 0x43],
            [0x59, 0x35, 0x80, 0x7a],
            [0x73, 0x59, 0xf6, 0x7f],
            [0x3d, 0x80, 0x47, 0x7d],
            [0x47, 0x16, 0xfe, 0x3e],
            [0x1e, 0x23, 0x7e, 0x44],
            [0x6d, 0x7a, 0x88, 0x3b],
            [0xef, 0x44, 0xa5, 0x41],
            [0xa8, 0x52, 0x5b, 0x7f],
            [0xb6, 0x71, 0x25, 0x3b],
            [0xdb, 0x0b, 0xad, 0x00],
            [0xd4, 0xd1, 0xc6, 0xf8],
            [0x7c, 0x83, 0x9d, 0x87],
            [0xca, 0xf2, 0xb8, 0xbc],
            [0x11, 0xf9, 0x15, 0xbc],
            [0x6d, 0x88, 0xa3, 0x7a],
            [0x11, 0x0b, 0x3e, 0xfd],
            [0xdb, 0xf9, 0x86, 0x41],
            [0xca, 0x00, 0x93, 0xfd],
            [0x4e, 0x54, 0xf7, 0x0e],
            [0x5f, 0x5f, 0xc9, 0xf3],
            [0x84, 0xa6, 0x4f, 0xb2],
            [0x4e, 0xa6, 0xdc, 0x4f],
            [0xea, 0xd2, 0x73, 0x21],
            [0xb5, 0x8d, 0xba, 0xd2],
            [0x31, 0x2b, 0xf5, 0x60],
            [0x7f, 0x8d, 0x29, 0x2f],
            [0xac, 0x77, 0x66, 0xf3],
            [0x19, 0xfa, 0xdc, 0x21],
            [0x28, 0xd1, 0x29, 0x41],
            [0x57, 0x5c, 0x00, 0x6e],
            [0xd0, 0x14, 0xf9, 0xa8],
            [0xc9, 0xee, 0x25, 0x89],
            [0xe1, 0x3f, 0x0c, 0xc8],
            [0xb6, 0x63, 0x0c, 0xa6],
        ];
        let mut result: [[u8; 4]; 44] = [[0; 4]; 44];
        for (i, line) in key_expansion(&key).into_iter().enumerate() {
            for (j, item) in line.into_iter().enumerate() {
                result[j][i] = item;
            }
        }
        assert_eq!(result, validate);
    }

    #[test]
    fn test_enc() {
        let phrase = [0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d,
            0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34];
        let key: [u8; 16] = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
            0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        let valid = [0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb,
            0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b, 0x32];
        let key_sch = key_expansion(&key);
        let result = enc(&phrase, &key_sch);
        assert_eq!(result, valid);
    }

    #[test]
    fn test_dec() {
        let phrase = [0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb,
            0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b, 0x32];
        let key: [u8; 16] = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
            0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        let valid = [0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d,
            0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34];
        let key_sch = key_expansion(&key);
        let result = dec(&phrase, &key_sch);
        assert_eq!(result, valid);
    }

    #[test]
    fn test_encrypt() {
        let phrase = str_to_hex("От одного порченого яблока весь воз загнивает.", 1);
        let key = "2b7e151628aed2a6abf7158809cf4f3c";
        let valid = "8d07448105b69375e77ae4826efa2cd95bbd55430f1a894adf175ad47741caf2f03d3b8f32eede0a578380d4578bfe9ff284f34bf3d995fe80d17d323cd8d1e7b628fca087c7a232f48ce6d172b8770bd198b67d94f76958180cc9c32c2b3ec3";
        assert_eq!(valid, encrypt(&phrase, key).unwrap());
    }
    #[test]
    fn test_decrypt() {
        let phrase = "8d07448105b69375e77ae4826efa2cd95bbd55430f1a894adf175ad47741caf2f03d3b8f32eede0a578380d4578bfe9ff284f34bf3d995fe80d17d323cd8d1e7b628fca087c7a232f48ce6d172b8770bd198b67d94f76958180cc9c32c2b3ec3";
        let key = "2b7e151628aed2a6abf7158809cf4f3c";
        let valid = "От одного порченого яблока весь воз загнивает.";
        assert_eq!(valid, hex_to_str(&decrypt(phrase, key).unwrap()).unwrap());
    }
}
