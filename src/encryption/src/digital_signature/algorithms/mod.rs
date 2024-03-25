use crate::alphabet::Alphabet;

pub mod egsa;
pub mod rsa_sign;

fn square_hash(phrase: &str, modula: usize) -> usize {
    let alphabet = Alphabet::new();
    let mut hi = 0;
    for mi in phrase.chars().map(|letter| alphabet.index_of(letter) + 1) {
        hi = ((hi + mi) * (hi + mi)) % modula;
    }
    hi
}

#[cfg(test)]
mod algorithms_tests {
    use super::*;

    #[test]
    fn test_square_hash() {
        let phrase = "отодногопорченогояблокавесьвоззагниваеттчк";
        let valid = 4;
        let result = square_hash(phrase, 11);
        assert_eq!(result, valid);
    }
}
