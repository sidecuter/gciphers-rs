use num::Integer;
use rand::Rng;

use crate::alphabet::Alphabet;
use crate::asymmetric::pow_mod;
use crate::methods::validate_single;
use std::error::Error;

use super::square_hash;
pub use crate::asymmetric::elgamal::gen_keys;

pub fn sign(phrase: &str, p: usize, g: usize) -> Result<(usize, usize), Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    let m = square_hash(phrase, p);
    let mut rng = rand::thread_rng();
    let mut k = rng.gen_range(2..p - 1);
    while (p - 1).gcd(&k) == 1 {
        k = rng.gen_range(2..p - 1);
    }
    let a = pow_mod(g, k, p);
    let b = 0; // Тут нужно заменить на расширенный алгоритм Евклида
    Ok((a, b))
}

pub fn check_sign(
    phrase: &str,
    p: usize,
    g: usize,
    y: usize,
    (a, b): (usize, usize),
) -> Result<bool, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    let m = square_hash(phrase, p);
    let a1 = (pow_mod(y, a, p) * pow_mod(a, b, p)) % p;
    let a2 = pow_mod(g, m, p);
    Ok(a1 == a2)
}
