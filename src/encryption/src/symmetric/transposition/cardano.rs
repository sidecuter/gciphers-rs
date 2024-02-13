use std::error::Error;
use std::str::Chars;
use rand::Rng;
use crate::alphabet::Alphabet;
use crate::errors::{InvalidSize, NullSizedValue};

struct CardanoTable {
    rows: usize,
    cols: usize,
    grid: Vec<bool>,
    data: Vec<char>,
}

impl CardanoTable {
    fn new(rows: usize, cols: usize, grid: Vec<bool>) -> Self {
        CardanoTable { rows, cols, grid, data: vec!['\u{0}'; rows*cols] }
    }

    fn new_data(rows: usize, cols: usize, grid: Vec<bool>, data: Vec<char>) -> Self {
        CardanoTable { rows, cols, grid, data }
    }

    fn reflect(&mut self, direction: bool) {
        if direction { self.reflect_vertical(); }
        else { self.reflect_horizontal(); }
    }

    fn reflect_vertical(&mut self) {
        let mut result: Vec<bool> = Vec::new();
        for i in 0..self.rows {
            for j in (0..self.cols).rev() {
                result.push(*self.grid.get(i*self.cols+j).unwrap());
            }
        }
        self.grid = result;
    }

    fn reflect_horizontal(&mut self) {
        let mut result: Vec<bool> = Vec::new();
        for i in (0..self.rows).rev() {
            for j in 0..self.cols {
                result.push(*self.grid.get(i*self.cols+j).unwrap());
            }
        }
        self.grid = result;
    }

    fn fill_partial(&mut self, letter_iter: &mut Chars) {
        let alphabet = Alphabet::new();
        for (i, flag) in self.grid.iter().enumerate() {
            if *flag {
                *self.data.get_mut(i).unwrap() = match letter_iter.next() {
                    Some(letter) => letter,
                    None => alphabet.get(
                        rand::thread_rng().gen_range(0..alphabet.len())
                    )
                }
            }
        }
    }

    fn fill(&mut self, phrase: &str, directions: &[bool]) -> String {
        let mut letter_iter = phrase.chars();
        for direction in directions {
            self.fill_partial(&mut letter_iter);
            self.reflect(*direction);
        }
        self.fill_partial(&mut letter_iter);
        self.data.iter().collect()
    }

    fn extract_partial(&mut self) -> String {
        let mut result = String::new();
        for (i, flag) in self.grid.iter().enumerate() {
            if *flag { result.push(*self.data.get(i).unwrap()); }
        }
        result
    }

    fn extract(&mut self, directions: &[bool]) -> String {
        let mut result = String::new();
        for direction in directions {
            result.push_str(&self.extract_partial());
            self.reflect(*direction);
        }
        result.push_str(&self.extract_partial());
        result
    }
}

pub fn encrypt(phrase: &str, grid: Vec<bool>, rows: usize, cols: usize, dirs: Vec<bool>)
    -> Result<String, Box<dyn Error>>
{
    validate(phrase, &grid, rows, cols, &dirs)?;
    let mut table = CardanoTable::new(rows, cols, grid);
    Ok(table.fill(phrase, &dirs))
}

pub fn decrypt(phrase: &str, grid: Vec<bool>, rows: usize, cols: usize, dirs: Vec<bool>)
    -> Result<String, Box<dyn Error>>
{
    validate(phrase, &grid, rows, cols, &dirs)?;
    let mut table = CardanoTable::new_data(
        rows, cols, grid, validate_data(phrase, rows, cols)?
    );
    Ok(table.extract(&dirs))
}

fn validate(phrase: &str, grid: &Vec<bool>, rows: usize, cols: usize, dirs: &Vec<bool>)
    -> Result<(), Box<dyn Error>>
{
    let alphabet = Alphabet::new();
    if phrase.len() == 0 { Err(NullSizedValue::new("Фраза"))?; }
    if rows*cols < phrase.chars().count() {
        Err(InvalidSize::new("Фраза должна быть меньше размера решетки"))?;
    }
    if rows % 2 != 0 || cols % 2 != 0 {
        Err(InvalidSize::new("Размеры решетки должны быть кратны 2"))?;
    }
    if grid.iter().filter(|x| **x == true).count() != rows * cols / 4 {
        Err(InvalidSize::new("Количество выколотых ячеек в решетке должно быть равно произведению размеров решетки деленому на 4"))?;
    }
    if dirs.iter().count() != 3 {
        Err(InvalidSize::new("Количество отражений должно быть равно 3"))?;
    }
    alphabet.validate(phrase)?;
    Ok(())
}

fn validate_data(data: &str, rows: usize, cols: usize) -> Result<Vec<char>, Box<dyn Error>> {
    if data.chars().count() != rows*cols {
        Err(InvalidSize::new("Длина фразы должна совпадать с размером решетки"))?;
    }
    Ok(data.chars().collect())
}

#[cfg(test)]
mod cardano_tests {
    use super::*;

    #[test]
    fn test_fill() {
        let mut table = CardanoTable::new(2, 2, vec![false, true, false, false]);
        let directions = vec![true, false, true];
        let result = table.fill("окно", &directions);
        assert_eq!(result, "коно");
    }

    #[test]
    fn test_extract() {
        let mut table = CardanoTable::new_data(
            2, 2, vec![false, true, false, false],
            "коно".chars().collect()
        );
        let directions = vec![true, false, true];
        let result = table.extract(&directions);
        assert_eq!(result, "окно");
    }

    #[test]
    fn test_encrypt() {
        let result = encrypt(
            "окноокноокноокно",
            vec![false, true, false, true,
                 true, false, true, false,
                 false, false, false, false,
                 false, false, false, false],
            4, 4,
            vec![true, false, true]
        ).unwrap();
        assert_eq!(result, "ооккннооооккнноо");
    }

    #[test]
    fn test_decrypt() {
        let result = decrypt(
            "ооккннооооккнноо",
            vec![false, true, false, true,
                 true, false, true, false,
                 false, false, false, false,
                 false, false, false, false],
            4, 4,
            vec![true, false, true]
        ).unwrap();
        assert_eq!(result, "окноокноокноокно");
    }

    #[test]
    fn test_crypt() {
        let result = encrypt(
            "криптография",
            vec![false, true, false, true,
                 true, false, true, false,
                 false, false, false, false,
                 false, false, false, false],
            4, 4,
            vec![true, false, true]
        ).unwrap();
        assert_eq!(result, "ткоригпрлаьфидяи");
    }

    #[test]
    fn test_crypt1() {
        let result = decrypt(
            "ткоригпрлаьфидяи",
            vec![false, true, false, true,
                 true, false, true, false,
                 false, false, false, false,
                 false, false, false, false],
            4, 4,
            vec![true, false, true]
        ).unwrap();
        assert_eq!(result, "криптография");
    }


}
