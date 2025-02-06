pub mod belazo;
pub mod trithemium;
pub mod vigenere;

use std::error::Error;
use crate::alphabet::Alphabet;
use crate::methods::{modd, validate_two};

fn proto(alphabet: &Alphabet, phrase: &str, key: &str, reverse: bool) -> Result<String, Box<dyn Error>> {
    validate_two(alphabet, phrase, key)?;
    let mut cur_key: usize = 0;
    let key_length = key.chars().count();
    let result: String = phrase.chars().map(move |letter| {
        let shift = alphabet.index_of(key.chars().nth(cur_key).unwrap()) as isize;
        let pos = alphabet.index_of(letter) as isize;
        cur_key = modd((cur_key + 1) as isize, key_length);
        alphabet.get(
            modd(
                pos + shift * (if reverse {-1} else {1}),
                alphabet.len()
            )
        )
    }).collect();
    Ok(result)
}

fn encrypt (alphabet: &Alphabet, phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    proto(alphabet, phrase, key, false)
}

fn decrypt (alphabet: &Alphabet, phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    proto(alphabet, phrase, key, true)
}
