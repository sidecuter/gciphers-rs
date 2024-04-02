use rand::Rng;

use crate::methods::str_to_bytes;

use self::hash::hash_square;

mod hash {
    pub fn hash_square(m: Vec<u8>) -> u128 {
        let modula = u128::MAX;
        let mut h = 0;
        for mi in m {
            let mi = mi as u128;
            h = ((h + mi) * (h + mi)) % modula;
        }
        h
    }
}

fn pow_mod(left: u128, right: u128, modula: u128) -> u128 {
    let mut result = 1;
    let mut i = 0;
    while i < right {
        result *= left;
        result %= modula;
        i += 1;
    }
    result
}

pub fn sign(message: &str, a: u128, p: u128, x: u128, q: u128) -> (u128, u128) {
    let message_bytes = str_to_bytes(message, 8).expect("творится какая-то дичь");
    let mut rang = rand::thread_rng();
    let mut rs = 0;
    let mut h = hash_square(message_bytes);
    if h == 0 {
        h = 1;
    }
    let mut s = 0;
    let mut k = rang.gen_range(1..q);
    let mut r;
    while s == 0 {
        while rs == 0 {
            k = rang.gen_range(1..q);
            r = pow_mod(a, k, p);
            rs = r % q;
        }
        s = (x * rs + k * h) % q;
    }
    (rs, s)
}

pub fn check_sign(
    message: &str,
    p: u128,
    q: u128,
    a: u128,
    y: u128,
    (rs, s): (u128, u128),
) -> bool {
    let message_bytes = str_to_bytes(message, 8).expect("");
    let mut h = hash_square(message_bytes);
    if h == 0 {
        h = 1;
    }
    let v = pow_mod(h, q - 2, q);
    let z1 = (s * v) % q;
    let z2 = ((q - rs) * v) % q;
    let u = ((pow_mod(a, z1, p) * pow_mod(y, z2, p)) % p) % q;
    u == rs
}
