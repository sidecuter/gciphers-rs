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

pub fn t (in_data: &[u8]) -> [u8; 4] {
    let mut result: [u8; 4] = [0; 4];
    for (i, number) in in_data.iter().enumerate() {
        let first_part_byte = (*number & 0xf0) >> 4;
        let sec_part_byte = *number & 0x0f;
        let first_part_byte = S_TABLE[i * 2][first_part_byte as usize];
        let sec_part_byte = S_TABLE[i * 2 + 1][sec_part_byte as usize];
        result[i] = (first_part_byte << 4) | sec_part_byte;
    }
    result
}

pub fn t_reverse(in_data: &[u8]) -> [u8; 4] {
    let mut result = [0; 4];
    for (i, number) in in_data.iter().enumerate() {
        let first_part_byte = (*number & 0xf0) >> 4;
        let sec_part_byte = *number & 0x0f;
        let first_part_byte = S_TABLE[i * 2].iter()
            .position(|x| *x == first_part_byte).unwrap() as u8;
        let sec_part_byte = S_TABLE[i * 2 + 1].iter()
            .position(|x| *x == sec_part_byte).unwrap() as u8;
        result[i] = (first_part_byte << 4) | sec_part_byte;
    }
    result
}

fn to_32(vec: &[u8]) -> u32 {
    let mut result: u32;
    result = vec[0] as u32;
    result = (result << 8) + vec[1] as u32;
    result = (result << 8) + vec[2] as u32;
    result = (result << 8) + vec[3] as u32;
    result
}

fn from_32(num: u32) -> [u8; 4] {
    let mut result = [0; 4];
    result[3] = (num & 0xff) as u8;
    result[2] = ((num >> 8) & 0xff) as u8;
    result[1] = ((num >> 16) & 0xff) as u8;
    result[0] = ((num >> 24) & 0xff) as u8;
    result
}

fn add_32(left: &[u8], right: &[u8]) -> [u8; 4] {
    let left_32: u32 = to_32(left);
    let right_32: u32 = to_32(right);
    let result_32 = ((left_32 as u64 + right_32 as u64) % 0x100000000u64) as u32;
    from_32(result_32)
}

pub fn g(key: &[u8], a: &[u8]) -> [u8; 4] {
    let internal = add_32(a, key);
    let internal = t(&internal);
    let mut result_32 = to_32(&internal);
    result_32 = (result_32 << 11) | (result_32>>21);
    from_32(result_32)
}

#[cfg(test)]
mod magma_tests {
    use crate::methods::{str_to_bytes, bytes_to_hex, hex_to_bytes, bytes_to_string};
    use super::*;

    #[test]
    fn test_t() {
        let data: [[u8; 4]; 4] = [
            [ 0xfd, 0xb9, 0x75, 0x31 ],
            [ 0x2a, 0x19, 0x6f, 0x34 ],
            [ 0xeb, 0xd9, 0xf0, 0x3a ],
            [ 0xb0, 0x39, 0xbb, 0x3d ]
        ];
        let validate: [[u8; 4]; 4] = [
            [ 0x2a, 0x19, 0x6f, 0x34 ],
            [ 0xeb, 0xd9, 0xf0, 0x3a ],
            [ 0xb0, 0x39, 0xbb, 0x3d ],
            [ 0x68, 0x69, 0x54, 0x33 ]
        ];
        for (values, results) in data.iter().zip(validate.iter()) {
            let result = t(values);
            assert_eq!(*results, result);
        }
    }

    #[test]
    fn test_t_reverse() {
        let data: [[u8; 4]; 4] = [
            [ 0x2a, 0x19, 0x6f, 0x34 ],
            [ 0xeb, 0xd9, 0xf0, 0x3a ],
            [ 0xb0, 0x39, 0xbb, 0x3d ],
            [ 0x68, 0x69, 0x54, 0x33 ]
        ];
        let validate: [[u8; 4]; 4] = [
            [ 0xfd, 0xb9, 0x75, 0x31 ],
            [ 0x2a, 0x19, 0x6f, 0x34 ],
            [ 0xeb, 0xd9, 0xf0, 0x3a ],
            [ 0xb0, 0x39, 0xbb, 0x3d ]
        ];
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
            "c812e316e83756dc663759dc633758dc63f7eb71c812e31ccebdeb75c814eb7fc81aeb7fe83f70dc6e3754dc633757dc68f7eb76c811e314cebb2bdc623756dc6cf7eb79c817eb72c814eb7ec815eb7cc811e316e357cb6c"
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
        let keys: [[u8; 4]; 4] = [
            [ 0x87, 0x65, 0x43, 0x21 ],
            [ 0xfd, 0xcb, 0xc2, 0x0c ],
            [ 0x7e, 0x79, 0x1a, 0x4b ],
            [ 0xc7, 0x65, 0x49, 0xec ]
        ];
        let validate: [[u8; 4]; 4] = [
            [ 0xfd, 0xcb, 0xc2, 0x0c ],
            [ 0x7e, 0x79, 0x1a, 0x4b ],
            [ 0xc7, 0x65, 0x49, 0xec ],
            [ 0x97, 0x91, 0xc8, 0x49 ]
        ];
        let data: [[u8; 4]; 4] = [
            [ 0xfe, 0xdc, 0xba, 0x98 ],
            [ 0x87, 0x65, 0x43, 0x21 ],
            [ 0xfd, 0xcb, 0xc2, 0x0c ],
            [ 0x7e, 0x79, 0x1a, 0x4b ],
        ];
        for (i , datum) in data.iter().enumerate() {
            assert_eq!(validate.get(i).unwrap(), &g(keys.get(i).unwrap(), datum));
        }
    }
}
