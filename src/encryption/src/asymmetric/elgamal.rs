use std::error::Error;
use itertools::Itertools;
use num::Integer;
use rand::Rng;
use primes::is_prime;
use crate::alphabet::Alphabet;
use crate::asymmetric::{get_numbers, pow_mod, to_string};
use crate::errors::{InvalidIndex, InvalidKeyError, InvalidTextError};
use crate::methods::validate_single;

struct Generator {
    phi: usize,
    count: usize,
    elems: Option<Vec<usize>>,
    index: usize
}

impl Generator {
    fn new(phi: usize, count: usize, elems: Option<Vec<usize>>) -> Self {
        if let Some(elems) = elems {
            Self {
                phi, count, elems: Some(elems), index: 0
            }
        } else { Self { phi, count, elems: None, index: 0 } }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();
        if self.count > 0 {
            let elem = if let Some(elems) = self.elems.clone() {
                let elem = elems[self.index];
                self.index = (self.index + 1) % elems.len();
                elem
            } else {
                let mut elem = rng.gen_range(2..self.phi);
                while self.phi.gcd(&elem) != 1 {
                    elem = rng.gen_range(2..self.phi);
                }
                elem
            };
            self.count -= 1;
            Some(elem)
        } else { None }
    }
}

pub fn gen_keys() -> (usize, usize, usize, usize) {
    let alphabet = Alphabet::new();
    let mut rng = rand::thread_rng();
    let mut p = rng.gen_range(2..alphabet.len());
    while !is_prime(p as u64) {
        p = rng.gen_range(2..alphabet.len());
    }
    let x = rng.gen_range(2..alphabet.len());
    let g = rng.gen_range(2..alphabet.len());
    let y = pow_mod(g, x, p);
    (p, x, g, y)
}

fn validate(p: usize, g: usize) -> Result<(), Box<dyn Error>> {
    let alphabet = Alphabet::new();
    if p <= alphabet.len() {
        Err(InvalidIndex)?;
    }
    if g >= p || g == 1 {
        Err(InvalidIndex)?;
    }
    Ok(())
}

fn validate_dec(phrase: &str, x: usize, p: usize) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let alphabet = Alphabet::from("0123456789".to_string());
    alphabet.validate(phrase)?;
    let len = p.to_string().len();
    if x >= p || x == 1 { Err(InvalidKeyError::new("x должно быть меньше p"))?; }
    if phrase.chars().count() % (len * 2) != 0 {
        Err(InvalidTextError)?;
    }
    let result: Vec<_> = get_numbers(phrase, len)
        .into_iter()
        .tuple_windows()
        .step_by(2)
        .collect();
    for (ai, bi) in result.iter() {
        if *ai >= p || *bi >= p {
            Err(InvalidTextError)?;
        }
    }
    Ok(result)
}

pub fn encrypt(phrase: &str, p: usize, g: usize, y: usize, r: Option<Vec<usize>>)
    -> Result<String, Box<dyn Error>>
{
    let alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    validate(p, g)?;
    let phi = p-1;
    let len = p.to_string().len();
    let gen = Generator::new(phi, phrase.chars().count(), r);
    let result: String = phrase.chars().zip(gen).map(|(mi, ki)| {
        let mi = alphabet.index_of(mi)+1;
        let ai = pow_mod(g, ki, p);
        let bi = (pow_mod(y, ki, p) * mi) % p;
        to_string(ai, len) + &to_string(bi, len)
    }).collect::<Vec<String>>().join("");
    Ok(result)
}

pub fn decrypt(phrase: &str, p: usize, x: usize) -> Result<String, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    let phrase = validate_dec(phrase, x, p)?;
    let buffer: Vec<usize> = phrase.into_iter().map(|(ai, bi)| {
        (bi * pow_mod(pow_mod(ai, x, p), p-2, p)) % p - 1
    }).collect();
    let mut result = String::new();
    for num in buffer {
        if alphabet.has(num) {
            result.push(alphabet.get(num));
        } else {
            Err(InvalidIndex)?;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod elgamal_tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let phrase = "отодно";
        let p = 41;
        let g = 3;
        let y = 14;
        let valid = "273727251404272627401404";
        assert_eq!(encrypt(phrase, p, g, y, Some(vec![3, 11, 7])).unwrap(), valid);
    }

    #[test]
    fn test_decrypt() {
        let phrase = "273727251404272627401404";
        let p = 41;
        let x = 15;
        let valid = "отодно";
        assert_eq!(decrypt(phrase, p, x).unwrap(), valid);
    }
}