use num::Integer;
use rand::Rng;

use crate::alphabet::Alphabet;
use crate::asymmetric::pow_mod;
use crate::methods::{modd, validate_single};
use std::error::Error;

use super::square_hash;
pub use crate::asymmetric::elgamal::gen_keys;

fn get_b(m: usize, x: usize, a: usize, k: usize, p: usize) -> usize {
    let xa = modd((x * a) as isize, p);
    let m_xa = modd(m as isize - xa as isize, p);
    let k_rev = pow_mod(k, p - 1, p);
    (m_xa * k_rev) % p
}

pub fn sign(phrase: &str, p: usize, g: usize, x: usize, modula: usize, k: Option<usize>)
    -> Result<(usize, usize), Box<dyn Error>>
{
    let alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    let m = square_hash(phrase, modula);
    let k = if k.is_none() {
        let mut rng = rand::thread_rng();
        let mut k = rng.gen_range(2..p - 1);
        while (p - 1).gcd(&k) == 1 {
            k = rng.gen_range(2..p - 1);
        }
        k
    } else {
        k.unwrap()
    };
    let a = pow_mod(g, k, p);
    let b = get_b(m, x, a, k, p - 1);
    Ok((a, b))
}

pub fn check_sign(
    phrase: &str,
    p: usize,
    g: usize,
    y: usize,
    modula: usize,
    (a, b): (usize, usize),
) -> Result<bool, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    let m = square_hash(phrase, modula);
    let a1 = (pow_mod(y, a, p) * pow_mod(a, b, p)) % p;
    let a2 = pow_mod(g, m, p);
    Ok(a1 == a2)
}

#[cfg(test)]
mod egsa_tests {
    use super::*;

    #[test]
    fn test_get_b() {
        let m = 4;
        let k = 5;
        let a = 31;
        let p = 37;
        let x = 3;
        let b = get_b(m, x, a, k, p - 1);
        assert_eq!(b, 11);
    }

    #[test]
    fn test_sign() {
        let phrase = "отодногопорченогояблокавесьвоззагниваеттчк";
        let p = 37;
        let g = 31;
        let x = 3;
        let modula = 11;
        let valid = (31, 11);
        let result = sign(phrase, p, g, x, modula, Some(5)).unwrap();
        assert_eq!(result, valid);
    }

    #[test]
    fn test_check_sign() {
        let phrase = "отодногопорченогояблокавесьвоззагниваеттчк";
        let p = 37;
        let g = 31;
        let y = 6;
        let modula = 11;
        let s = (31, 11);
        let result = check_sign(phrase, p, g, y, modula, s).unwrap();
        assert!(result);
    }
}
