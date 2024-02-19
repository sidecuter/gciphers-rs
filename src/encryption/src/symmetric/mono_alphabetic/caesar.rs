use std::error::Error;
use crate::methods::{modd};
use crate::alphabet::Alphabet;
use crate::errors::{NullSizedValue, InvalidKeyError};

fn proto(phrase: &str, shift: isize) -> Result<String, Box<dyn Error>> {
    let alphabet: Alphabet = Alphabet::new();
    validate(&alphabet, phrase, shift.abs())?;
    let result: String = phrase.chars().map(|letter|
        alphabet.get(modd(alphabet.index_of(letter) as isize + shift, alphabet.len()))
    ).collect();
    Ok(result)
}

pub fn encrypt(phrase: &str, shift: isize) -> Result<String, Box<dyn Error>> {
    proto(phrase, shift)
}
pub fn decrypt(phrase: &str, shift: isize) -> Result<String, Box<dyn Error>> {
    proto(phrase, -shift)
}

fn validate(alphabet: &Alphabet, text: &str, shift: isize) -> Result<(), Box<dyn Error>> {
    if text.is_empty() { return Err(Box::new(NullSizedValue::new("Фраза"))); }
    if shift >= alphabet.len() as isize || shift < 1 {
        return Err(Box::new(InvalidKeyError::new("Сдвиг не принадлежит заданному диапазону от 1 до 32")));
    }
    alphabet.validate(text)
}

#[cfg(test)]
mod caesar_tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let result = encrypt(
            "отодногопорченогояблокавесьвоззагниваеттчк", 3).unwrap();
        assert_eq!(result, "схсзрсжстсуъирсжсвдоснгеифяесккгжрлегиххън");
    }

    #[test]
    fn test_decrypt() {
        let result = decrypt(
            "схсзрсжстсуъирсжсвдоснгеифяесккгжрлегиххън", 3).unwrap();
        assert_eq!(result, "отодногопорченогояблокавесьвоззагниваеттчк");
    }

    #[test]
    #[should_panic]
    fn test_encrypt_panic() {
        encrypt("z", 3).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_decrypt_panic() {
        decrypt("z", 3).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_encrypt_shift_panic() {
        encrypt("отодногопорченогояблокавесьвоззагниваеттчк", 32).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_decrypt_shift_panic() {
        decrypt("отодногопорченогояблокавесьвоззагниваеттчк", 32).unwrap();
    }
}
