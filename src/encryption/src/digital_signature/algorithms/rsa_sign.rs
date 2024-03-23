use crate::alphabet::Alphabet;
use crate::asymmetric::pow_mod;
use crate::errors::InvalidKeyError;
use crate::methods::validate_single;
use std::error::Error;

use super::square_hash;
pub use crate::asymmetric::rsa::gen_keys;

pub fn sign(phrase: &str, n: usize, d: usize) -> Result<usize, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    if d >= n {
        Err(InvalidKeyError::new("D должно быть меньше либо равно n"))?;
    }
    let m = square_hash(phrase, n);
    let result = pow_mod(m, d, n);
    Ok(result)
}

pub fn check_sign(phrase: &str, n: usize, e: usize, s: usize) -> Result<bool, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    if e >= n {
        Err(InvalidKeyError::new("E должно быть меньше либо равно n"))?;
    }
    let m = square_hash(phrase, n);
    let ms = pow_mod(s, e, n);
    Ok(m == ms)
}
