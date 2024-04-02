use rand::Rng;
use crate::alphabet::Alphabet;

fn square_hash(phrase: &str, modula: u128) -> u128 {
    let alphabet = Alphabet::new();
    let mut hi = 0;
    for mi in phrase.chars().map(|letter| alphabet.index_of(letter) + 1) {
        let mi = mi as u128;
        hi = ((hi + mi) * (hi + mi)) % modula;
    }
    hi
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

pub fn sign(message: &str, a: u128, p: u128, x: u128, q: u128, m: u128) -> (u128, u128) {
    let mut rang = rand::thread_rng();
    let mut rs = 0;
    let mut h = square_hash(message, m);
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
    m: u128,
    (rs, s): (u128, u128),
) -> bool {
    let mut h = square_hash(message, m);
    if h == 0 {
        h = 1;
    }
    let v = pow_mod(h, q - 2, q);
    let z1 = (s * v) % q;
    let z2 = ((q - rs) * v) % q;
    let u = ((pow_mod(a, z1, p) * pow_mod(y, z2, p)) % p) % q;
    u == rs
}
