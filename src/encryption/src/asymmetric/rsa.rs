use rand::Rng;
use std::error::Error;
use num::Integer;
use super::{pow_mod, to_string};
use crate::alphabet::Alphabet;
use crate::errors::{InvalidIndex, InvalidKeyError, InvalidTextError};
use crate::methods::validate_single;

fn proto(letters: &[usize], power: usize, modula: usize) -> Vec<usize> {
    letters.iter().map(|x| pow_mod(*x, power, modula)).collect()
}

pub fn gen_keys(p: usize, q: usize) -> (usize, usize, usize) {
    let phi = (p-1) * (q-1);
    let n = p*q;
    let mut rng = rand::thread_rng();
    let mut e = rng.gen_range(1..phi);
    while phi.gcd(&e) != 1 {
        e = rng.gen_range(1..phi);
    }
    let d = pow_mod(e, phi-1, phi);
    (e, d, n)
}

pub fn encrypt(phrase: &str, n: usize, e: usize) -> Result<String, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    if e >= n { Err(InvalidKeyError::new("E должно быть меньше либо равно n"))?; }
    let mut result = String::new();
    let len = n.to_string().len();
    let _: () = proto(
        &phrase.chars().into_iter().map(|letter| alphabet.index_of(letter)+1).collect::<Vec<usize>>(),
        e,
        n
    ).into_iter().map(|res| result.push_str(&to_string(res, len))).collect();
    Ok(result)
}

fn validate(phrase: &str, n: usize, d: usize) -> Result<Vec<usize>, Box<dyn Error>> {
    let len = n.to_string().len();
    if d >= n { Err(InvalidKeyError::new("D должно быть меньше либо равно n"))?; }
    if phrase.chars().count() % len != 0 {
        Err(InvalidTextError)?;
    }
    let result: Vec<_> = phrase.chars().collect::<Vec<char>>()
        .windows(len).step_by(len)
        .map(|x| x.iter().collect::<String>().parse::<usize>().unwrap())
        .collect();
    for letter in result.iter() {
        if *letter >= n {
            Err(InvalidTextError)?;
        }
    }
    Ok(result)
}

pub fn decrypt(phrase: &str, n: usize, d: usize) -> Result<String, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    let mut result = String::new();
    let phrase = validate(phrase, n, d)?;
    let buffer = proto(&phrase, d, n);
    for num in buffer {
        if alphabet.has(num - 1) {
            result.push(alphabet.get(num-1));
        } else {
            Err(InvalidIndex)?;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod rsa_tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let phrase = "отодногопорченогояблокавесьвоззагниваеттчк";
        let n = 77;
        let e = 23;
        let valid = "641764594964096404644019624964096465744564440105620257056450500109492505016217171944";
        assert_eq!(encrypt(phrase, n, e).unwrap(), valid);
    }

    #[test]
    fn test_decrypt() {
        let phrase = "641764594964096404644019624964096465744564440105620257056450500109492505016217171944";
        let n = 77;
        let d = 47;
        let valid = "отодногопорченогояблокавесьвоззагниваеттчк";
        assert_eq!(decrypt(phrase, n, d).unwrap(), valid);
    }
}
