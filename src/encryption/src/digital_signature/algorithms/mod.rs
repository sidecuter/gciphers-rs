use crate::alphabet::Alphabet;

pub mod egsa;
pub mod rsa_sign;

fn square_hash(phrase: &str, modula: usize) -> usize {
    let alphabet = Alphabet::new();
    let mut hi = 0;
    let _: () = phrase
        .chars()
        .map(move |letter| {
            let index = alphabet.index_of(letter) + 1;
            hi = (hi + index).pow(2) % modula;
        })
        .collect();
    hi
}
