use std::error::Error;
use crate::alphabet::Alphabet;
use super::{encrypt as m_encrypt, decrypt as m_decrypt};

pub fn encrypt(phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let alphabet: Alphabet = Alphabet::new();
    m_encrypt(&alphabet, phrase, key)
}

pub fn decrypt(phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let alphabet: Alphabet = Alphabet::new();
    m_decrypt(&alphabet, phrase, key)
}

#[cfg(test)]
mod belazo_tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let result = encrypt(
            "отодногопорченогояблокавесьвоззагниваеттчк", "арбуз").unwrap();
        assert_eq!(result, "овпчфоупвхрзжахгюафтоъбхмсмгбозрдапвржещчъ");
    }

    #[test]
    fn test_decrypt() {
        let result = decrypt(
            "овпчфоупвхрзжахгюафтоъбхмсмгбозрдапвржещчъ", "арбуз").unwrap();
        assert_eq!(result, "отодногопорченогояблокавесьвоззагниваеттчк");
    }

    #[test]
    #[should_panic]
    fn test_encrypt_panic() {
        encrypt(
            "z", "арбуз").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_decrypt_panic() {
        decrypt(
            "z", "арбуз").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_encrypt_key_panic() {
        encrypt(
            "я", "z").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_decrypt_key_panic() {
        decrypt(
            "я", "z").unwrap();
    }
}
