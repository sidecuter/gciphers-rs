use std::error::Error;
use crate::errors::InvalidTextError;

pub struct Alphabet {
    alphabet: String,
    len: usize,
}

impl Alphabet {
    pub fn has(&self, index_of: usize) -> bool {
        match self.alphabet.chars().nth(index_of) {
            Some(_) => true,
            None => false
        }
    }

    pub fn get(&self, index: usize) -> char {
        match self.alphabet.chars().nth(index) {
            Some(letter) => letter,
            None => panic!("Данная функция подразумевает, что передаваемые данные были предварительно проверены")
        }
    }

    pub fn contains(&self, letter: char) -> bool {
        self.alphabet.contains(letter)
    }

    pub fn index_of(&self, letter: char) -> usize {
        match self.alphabet.chars().position(|x| x == letter) {
            Some(index) => index,
            None => panic!("Данная функция подразумевает, что передаваемые данные были предварительно проверены")
        }
    }

    pub fn new() -> Alphabet{
        let alphabet = "абвгдежзийклмнопрстуфхцчшщъыьэюя".to_string();
        Alphabet {
            len: alphabet.chars().count(),
            alphabet,
        }
    }

    pub fn alphabet(&self) -> &String {
        &self.alphabet
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn validate(&self, text: &str) -> Result<(), Box<dyn Error>> {
        for letter in text.chars() {
            if !self.contains(letter) { Err(InvalidTextError)?; }
        }
        Ok(())
    }
}

impl Default for Alphabet {
    fn default() -> Self {
        Alphabet::new()
    }
}

impl From<String> for Alphabet {
    fn from(value: String) -> Self {
        Alphabet {
            len: value.chars().count(),
            alphabet: value,
        }
    }
}

#[cfg(test)]
mod alphabet_tests {
    use crate::alphabet::Alphabet;

    #[test]
    fn test_new() {
        let alphabet = Alphabet::new();
        assert_eq!(alphabet.len, 32);
        assert_eq!(alphabet.alphabet, "абвгдежзийклмнопрстуфхцчшщъыьэюя".to_string());
    }

    #[test]
    fn test_get() {
        let alphabet = Alphabet::new();
        assert_eq!(alphabet.get(0), 'а');
    }

    #[test]
    fn test_index_of() {
        let alphabet = Alphabet::new();
        assert_eq!(alphabet.index_of('\u{0430}'), 0);
    }

    #[test]
    fn test_contains() {
        let alphabet = Alphabet::new();
        assert!(alphabet.contains('а'));
    }

    #[test]
    fn test_len() {
        let alphabet = Alphabet::new();
        assert_eq!(alphabet.len(), 32);
    }

    #[test]
    fn test_alphabet() {
        let alphabet = Alphabet::new();
        assert_eq!(*alphabet.alphabet(), "абвгдежзийклмнопрстуфхцчшщъыьэюя".to_string());
    }

    #[test]
    fn test_validate() {
        let alphabet = Alphabet::new();
        alphabet.validate("отодногопорченогояблокавесьвоззагниваеттчк").unwrap();
        assert!(true);
    }

    #[test]
    fn test_from() {
        let alphabet = Alphabet::from("ая".to_string());
        assert_eq!(alphabet.len, 2);
        assert_eq!(alphabet.alphabet, "ая".to_string());
    }

    #[test]
    #[should_panic]
    fn test_get_panic() {
        let alphabet = Alphabet::new();
        assert_eq!(alphabet.get(33), 'а');
    }

    #[test]
    #[should_panic]
    fn test_index_of_panic() {
        let alphabet = Alphabet::new();
        assert_eq!(alphabet.index_of('o'), 0);
    }

    #[test]
    #[should_panic]
    fn test_contains_panic() {
        let alphabet = Alphabet::new();
        assert!(alphabet.contains('o'));
    }

    #[test]
    #[should_panic]
    fn test_validate_panic() {
        let alphabet = Alphabet::new();
        alphabet.validate("o").unwrap();
    }
}
