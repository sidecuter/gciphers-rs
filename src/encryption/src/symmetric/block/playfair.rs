use std::collections::HashSet;
use std::error::Error;
use crate::alphabet::Alphabet;
use crate::errors::{InvalidKeyError, NullSizedValue};
use crate::methods::modd;
use itertools::Itertools;

struct PlayfairTable {
    cols: usize,
    data: Vec<char>
}

impl PlayfairTable {
    fn new(key: &str) -> Self {
        let alphabet = "абвгдежзиклмнопрстуфхцчшщъыэюя".to_string();
        let mut data: Vec<char> = key.chars().collect();
        for letter in alphabet.chars() {
            if !data.contains(&letter) { data.push(letter); }
        }
        PlayfairTable { cols: 6, data }
    }

    fn find_letter(&self, letter: char) -> (usize, usize) {
        let index = self.data.iter().position(|x| *x == letter).unwrap();
        (index / self.cols, index % self.cols)
    }

    fn get_pair(&self, letter1: char, letter2: char, rev: bool) -> (char, char) {
        let dir: isize = if rev { -1 } else { 1 };
        let (i1, j1) = self.find_letter(letter1);
        let (i2, j2) = self.find_letter(letter2);
        if i1 == i2 {
            let j1 = modd(j1 as isize + dir, self.cols);
            let j2 = modd(j2 as isize + dir, self.cols);
            (*self.data.get(i1 * self.cols + j1).unwrap(),
             *self.data.get(i2 * self.cols + j2).unwrap())
        }
        else if j1 == j2 {
            let i1 = modd(i1 as isize + dir, self.cols);
            let i2 = modd(i2 as isize + dir, self.cols);
            (*self.data.get(i1 * self.cols + j1).unwrap(),
             *self.data.get(i2 * self.cols + j2).unwrap())
        }
        else {
            (*self.data.get(i1 * self.cols + j2).unwrap(),
             *self.data.get(i2 * self.cols + j1).unwrap())
        }
    }
}

fn validate_key(key: &str) -> Result<(), Box<dyn Error>> {
    let mut set = HashSet::new();
    for letter in key.chars() {
        if !set.contains(&letter) { set.insert(letter); }
        else {
            Err(InvalidKeyError::new("Ключ должен состоять из уникальных значений"))?;
        }
    }
    Ok(())
}

fn prepare_phrase(phrase: &str) -> String {
    let mut result = String::new();
    let mut prev_letter = phrase.chars().next().unwrap();
    let mut i = 1;
    result.push(prev_letter);
    for letter in phrase.chars().skip(1) {
        if letter == prev_letter && (i+1)%2 == 0 {
            result.push('\u{0444}');
            i = (i + 1) % 2;
        }
        result.push(match letter {
            '\u{044C}' => '\u{044A}',
            '\u{0451}' => '\u{0435}',
            '\u{0439}' => '\u{0438}',
            letter => letter
        });
        prev_letter = letter;
        i = (i + 1) % 2;
    }
    if result.chars().count() % 2 != 0 {
        result.push('\u{0444}');
    }
    result
}

fn proto(phrase: &str, key: &str, rev: bool) -> Result<String, Box<dyn Error>> {
    validate(phrase, key)?;
    let table = PlayfairTable::new(key);
    let phrase = prepare_phrase(phrase);
    let mut result = String::new();
    for (prev, next) in phrase.chars().tuples() {
        let (prev, next) = table.get_pair(prev, next, rev);
        result.push(prev);
        result.push(next);
    }
    Ok(result)
}

pub fn encrypt(phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    proto(phrase, key, false)
}

pub fn decrypt(phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    proto(phrase, key, true)
}

fn validate(phrase: &str, key: &str) -> Result<(), Box<dyn Error>> {
    let polybius_alp = Alphabet::from("абвгдежзиклмнопрстуфхцчшщъыэюя".to_string());
    let alphabet = Alphabet::new();
    if phrase.is_empty() { Err(NullSizedValue::new("Фраза"))?; }
    if key.is_empty() { Err(NullSizedValue::new("Ключ"))?; }
    alphabet.validate(phrase)?;
    polybius_alp.validate(key)?;
    validate_key(key)
}

#[cfg(test)]
mod playfair_tests {
    use super::*;

    #[test]
    fn test_prepare_phrase() {
        let phrase = "отодногопорченогояблокавесьвоззагниваеттчк";
        let valid = "отодногопорченогояблокавесъвоззагниваетфтчкф";
        assert_eq!(prepare_phrase(phrase), valid);
    }

    #[test]
    fn test_encrypt() {
        let result = encrypt(
            "отодногопорченогояблокавесьвоззагниваеттчк",
            "респавн").unwrap();
        assert_eq!(result, "тимжжижламаурблжтюгктлврспяетжжвдбтрвскшошфъ");
    }

    #[test]
    fn test_decrypt() {
        let result = decrypt(
            "тимжжижламаурблжтюгктлврспяетжжвдбтрвскшошфъ",
            "респавн").unwrap();
        assert_eq!(result, "отодногопорченогояблокавесъвоззагниваетфтчкф");
    }

    #[test]
    fn test_validate() {
        assert_eq!(validate("аф", "ре").unwrap(), ());
    }

    #[test]
    #[should_panic]
    fn test_validate_phrase_panic() {
        validate("z", "респавн").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_validate_key_panic() {
        validate("аф", "z").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_validate_key_non_unique_panic() {
        validate_key("респавнн").unwrap();
    }
}
