use std::error::Error;
use crate::alphabet::Alphabet;
use crate::methods::validate_single;

fn proto(phrase: &str) -> Result<String, Box<dyn Error>> {
    let alphabet: Alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    let result: String = phrase.chars().map(
        |letter| alphabet.get(alphabet.len() - 1 - alphabet.index_of(letter))
    ).collect();
    Ok(result)
}

pub fn encrypt(phrase: &str) -> Result<String, Box<dyn Error>> {
    proto(phrase)
}

pub fn decrypt(phrase: &str) -> Result<String, Box<dyn Error>> {
    proto(phrase)
}

#[cfg(test)]
mod atbash_test {
    use super::*;

    #[test]
    fn test_encrypt() {
        let result = encrypt("отодногопорченогояблокавесьвоззагниваеттчк").unwrap();
        assert_eq!(result, "снсытсьсрспиътсьсаюфсхяэъогэсшшяьтчэяънних");
    }

    #[test]
    fn test_decrypt() {
        let result = encrypt("снсытсьсрспиътсьсаюфсхяэъогэсшшяьтчэяънних").unwrap();
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
