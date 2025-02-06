use std::error::Error;
use crate::alphabet::Alphabet;
use super::{encrypt as m_encrypt, decrypt as m_decrypt};

pub fn encrypt(phrase: &str) -> Result<String, Box<dyn Error>> {
    let alphabet: Alphabet = Alphabet::new();
    m_encrypt(&alphabet, phrase, alphabet.alphabet())
}

pub fn decrypt(phrase: &str) -> Result<String, Box<dyn Error>> {
    let alphabet: Alphabet = Alphabet::new();
    m_decrypt(&alphabet, phrase, alphabet.alphabet())
}

#[cfg(test)]
mod trithemium_tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let result = encrypt("отодногопорченогояблокавесьвоззагниваеттчк").unwrap();
        assert_eq!(result, "оурзсуйхччъвсъьтюруювяцщэкцэкдеягокедкшщяу");
    }

    #[test]
    fn test_decrypt() {
        let result = decrypt("оурзсуйхччъвсъьтюруювяцщэкцэкдеягокедкшщяу").unwrap();
        assert_eq!(result, "отодногопорченогояблокавесьвоззагниваеттчк");
    }

    #[test]
    #[should_panic]
    fn test_encrypt_panic() {
        encrypt("z").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_decrypt_panic() {
        decrypt("z").unwrap();
    }
}
