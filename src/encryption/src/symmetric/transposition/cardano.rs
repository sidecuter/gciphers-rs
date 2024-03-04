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

    fn validate(&self) -> Result<(), Box<dyn Error>> {
        let t1 = Self::new(self.rows, self.cols, self.grid.clone());
        let mut t2 = Self::new(t1.rows, t1.cols, t1.grid.clone());
        t2.reflect_vertical();
        let mut t3 = Self::new(t2.rows, t2.cols, t2.grid.clone());
        t3.reflect_horizontal();
        let mut t4 = Self::new(t3.rows, t3.cols, t3.grid.clone());
        t4.reflect_vertical();
        let monster = t1.grid.iter().zip(
            t2.grid.iter().zip(
                t3.grid.iter().zip(
                    t4.grid.iter()
                )
            )
        );
        for (elem1, (elem2, (elem3, elem4))) in monster {
            let buffer = [*elem1, *elem2, *elem3, *elem4];
            let count = buffer.iter().filter(|elem| **elem).count();
            if count != 1 {
                Err("Ячейки решетки накладываются друг на друга")?;
            }
        }
        Ok(())
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
                *self.data.get_mut(i).unwrap() = if let Some(letter) = letter_iter.next() { letter }
                else { alphabet.get(rand::thread_rng().gen_range(0..alphabet.len())) };
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
    table.validate()?;
    Ok(table.fill(phrase, &dirs))
}

pub fn decrypt(phrase: &str, grid: Vec<bool>, rows: usize, cols: usize, dirs: Vec<bool>)
    -> Result<String, Box<dyn Error>>
{
    validate(phrase, &grid, rows, cols, &dirs)?;
    let mut table = CardanoTable::new_data(
        rows, cols, grid, validate_data(phrase, rows, cols)?
    );
    table.validate()?;
    Ok(table.extract(&dirs))
}

fn validate(phrase: &str, grid: &[bool], rows: usize, cols: usize, dirs: &[bool])
    -> Result<(), Box<dyn Error>>
{
    let alphabet = Alphabet::new();
    if phrase.is_empty() { Err(NullSizedValue::new("Фраза"))?; }
    if rows*cols < phrase.chars().count() {
        Err(InvalidSize::new("Фраза должна быть меньше размера решетки"))?;
    }
    if rows % 2 != 0 || cols % 2 != 0 {
        Err(InvalidSize::new("Размеры решетки должны быть кратны 2"))?;
    }
    if grid.iter().filter(|x| **x).count() != rows * cols / 4 {
        Err(InvalidSize::new("Количество выколотых ячеек в решетке должно быть равно произведению размеров решетки деленому на 4"))?;
    }
    if dirs.len() != 3 {
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
}
