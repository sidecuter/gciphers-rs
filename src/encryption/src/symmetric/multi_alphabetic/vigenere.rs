use std::error::Error;
use crate::alphabet::Alphabet;
use crate::errors::{InvalidSize, NullSizedValue};
use crate::methods::modd;
use super::encrypt as m_encrypt;

pub fn encrypt(phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate(&alphabet, phrase, key)?;
    let mut key = key.to_owned();
    key.push_str(phrase);
    m_encrypt(&alphabet, phrase, &key)
}

pub fn decrypt(phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate(&alphabet, phrase, key)?;
    let mut buffer = key.chars().next().unwrap();
    let result = phrase.chars().map(move |letter| {
            buffer = alphabet.get(modd(
                alphabet.index_of(letter) as isize - alphabet.index_of(buffer) as isize,
                alphabet.len()));
            buffer
        }
    ).collect();
    Ok(result)
}

fn validate(alphabet: &Alphabet, phrase: &str, key: &str) -> Result<(), Box<dyn Error>> {
    if phrase.is_empty() { return Err(Box::new(NullSizedValue::new("Фраза"))); }
    if key.is_empty() { return Err(Box::new(NullSizedValue::new("Ключ"))); }
    if key.chars().count() > 1 { return Err(Box::new(InvalidSize::new("Ключ должен быть одной буквой"))); }
    alphabet.validate(phrase)?;
    alphabet.validate(key)
}

#[cfg(test)]
mod vigenere_tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let result = encrypt(
            "отодногопорченогояблокавесьвоззагниваеттчк", "ю").unwrap();
        assert_eq!(result, "маатсыссээюзьтысснамщшквзцнюрхозгрхквечдйб");
    }

    #[test]
    fn test_decrypt() {
        let result = decrypt(
            "маатсыссээюзьтысснамщшквзцнюрхозгрхквечдйб", "ю").unwrap();
        assert_eq!(result, "отодногопорченогояблокавесьвоззагниваеттчк");
    }

    #[test]
    #[should_panic]
    fn test_encrypt_panic() {
        encrypt("z", "ю").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_decrypt_panic() {
        decrypt("z", "ю").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_encrypt_key_panic() {
        encrypt("я", "z").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_decrypt_key_panic() {
        decrypt("я", "z").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_encrypt_key_length_panic() {
        encrypt("я", "юю").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_decrypt_key_length_panic() {
        decrypt("я", "юю").unwrap();
    }
}
