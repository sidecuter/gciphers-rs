use crate::alphabet::Alphabet;
use crate::asymmetric::pow_mod;
use crate::errors::InvalidKeyError;
use crate::methods::validate_single;
use std::error::Error;

use super::square_hash;
pub use crate::asymmetric::rsa::gen_keys;

pub fn sign(phrase: &str, n: usize, d: usize, modula: usize) -> Result<usize, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    if d >= n {
        Err(InvalidKeyError::new("D должно быть меньше либо равно n"))?;
    }
    let m = square_hash(phrase, modula);
    let result = pow_mod(m, d, n);
    Ok(result)
}

pub fn check_sign(
    phrase: &str,
    n: usize,
    e: usize,
    s: usize,
    modula: usize,
) -> Result<bool, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    if e >= n {
        Err(InvalidKeyError::new("E должно быть меньше либо равно n"))?;
    }
    let m = square_hash(phrase, modula);
    let ms = pow_mod(s, e, n);
    Ok(m == ms)
}

#[cfg(test)]
mod rsa_sign_test {
    use super::*;

    #[test]
    fn test_sign() {
        let phrase = "отодногопорченогояблокавесьвоззагниваеттчк";
        let n = 77;
        let d = 37;
        let modula = 11;
        let valid = 60;
        let result = sign(phrase, n, d, modula).unwrap();
        assert_eq!(result, valid);
    }

    #[test]
    fn test_check_sign() {
        let phrase = "отодногопорченогояблокавесьвоззагниваеттчк";
        let n = 77;
        let e = 13;
        let modula = 11;
        let s = 60;
        let result = check_sign(phrase, n, e, s, modula).unwrap();
        assert!(result)
    }
}
