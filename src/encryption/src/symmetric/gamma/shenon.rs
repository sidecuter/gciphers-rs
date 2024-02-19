use std::error::Error;
use crate::alphabet::Alphabet;
use crate::errors::InvalidSize;
use crate::methods::{modd, validate_single};

struct Generator {
    state: usize,
    a: usize,
    c: usize,
    modd: usize
}

impl Generator {
    fn new(state: usize, a: usize, c: usize, modd: usize) -> Self {
        Self { state, a, c, modd }
    }

    fn step(&mut self) -> usize {
        self.state = (self.a * self.state + self.c) % self.modd;
        self.state
    }
}

fn validate(alphabet: &Alphabet, phrase: &str, t0: &str, a: &str, c: &str)
    -> Result<(usize, usize, usize), Box<dyn Error>>
{
    let t0: usize = t0.parse()?;
    let a: usize = a.parse()?;
    let c: usize = c.parse()?;
    if 0 == t0 || alphabet.len() < t0 { Err(InvalidSize::new("T0 должен быть в пределах от 1 до 32"))?; }
    if 0 == a || a % 4 != 1 { Err(InvalidSize::new("a должно быть отлично от 0 и остаток от деления на 4 равен 1"))?; }
    if 0 == c || c % 2 == 0 { Err(InvalidSize::new("c должно быть отлично от 0 и нечетным"))?; }
    validate_single(alphabet, phrase)?;
    Ok((t0, a, c))
}

pub fn encrypt(phrase: &str, t0: &str, a: &str, c: &str) -> Result<String, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    let (t0, a, c) = validate(&alphabet, phrase, t0, a, c)?;
    let mut gen = Generator::new(t0, a, c, alphabet.len());
    Ok(phrase.chars().map(move |letter| {
        let pos = (gen.step() + alphabet.index_of(letter)) % alphabet.len();
        alphabet.get(pos)
    }).collect())
}

pub fn decrypt(phrase: &str, t0: &str, a: &str, c: &str) -> Result<String, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    let (t0, a, c) = validate(&alphabet, phrase, t0, a, c)?;
    let mut gen = Generator::new(t0, a, c, alphabet.len());
    Ok(phrase.chars().map(move |letter| {
        let pos = modd(
            alphabet.index_of(letter) as isize - gen.step() as isize,
            alphabet.len());
        alphabet.get(pos)
    }).collect())
}

#[cfg(test)]
mod shenon_tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let result = encrypt(
            "отодногопорченогояблокавесьвоззагниваеттчк",
            "3",
            "9",
            "5"
        ).unwrap();
        assert_eq!(result, "очалсчщщчыкжсюмцюфгввгжээожбкихггтъйдоиэяч");
    }

    #[test]
    fn test_decrypt() {
        let result = decrypt(
            "очалсчщщчыкжсюмцюфгввгжээожбкихггтъйдоиэяч",
            "3",
            "9",
            "5"
        ).unwrap();
        assert_eq!(result, "отодногопорченогояблокавесьвоззагниваеттчк");
    }

    #[test]
    fn test_step() {
        let mut gen = Generator::new(3, 9, 5, 32);
        let result = gen.step();
        assert_eq!(result, 0);
    }
}
