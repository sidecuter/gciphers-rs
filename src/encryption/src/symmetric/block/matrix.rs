use std::error::Error;
use crate::alphabet::Alphabet;
use crate::errors::{InvalidIndex, InvalidSize, NullSizedValue};
use num::cast::AsPrimitive;
use crate::methods::validate_single;

struct Matrix<T> where
    T: Copy
{
    rows: usize,
    cols: usize,
    data: Vec<T>
}

impl Matrix<f32> {
    fn det(&mut self) -> Result<f32, Box<dyn Error>>{
        if self.rows == 0 || self.cols == 0 {
            Err(InvalidSize::new("Матрица пуста"))?;
        }
        if self.rows == 1 {
            return Ok(*self.data.first().unwrap());
        }
        let mut result: f32 = 1.;
        if *self.data.first().unwrap() == 0. {
            if self.swap_zero().is_err() {
                return Ok(0.);
            } else { result *= -1.; }
        }
        let first_row: Vec<f32> = self.data[0..self.cols].to_vec();
        for i in 1..self.rows {
            if *self.data.get(i * self.cols).unwrap() != 0. {
                let mnoj: f32 = *self.data.get(i * self.cols).unwrap()
                    / *self.data.first().unwrap();
                let mut iter = first_row.iter();
                for elem in self.data[i * self.cols..(i + 1) * self.cols].iter_mut() {
                    *elem -= *iter.next().unwrap() * mnoj;
                }
            }
        }
        result *= *self.data.first().unwrap() * self.get_minor(0, 0)?.det()?;
        Ok(result)
    }

    fn swap_zero(&mut self) -> Result<(), Box<dyn Error>> {
        if self.rows == 0 || self.cols == 0 {
            Err(InvalidSize::new("Матрица пуста"))?;
        }
        let mut flag = false;
        for i in 0..self.rows {
            if *self.data.get(i * self.cols).unwrap() != 0. && !flag {
                for j in 0..self.cols {
                    let tmp = *self.data.get(j).unwrap();
                    *self.data.get_mut(j).unwrap() =
                        *self.data.get(i * self.cols + j).unwrap();
                    *self.data.get_mut(i * self.cols + j).unwrap() = tmp;
                }
                flag = true;
            }
        }
        if !flag { Err(InvalidIndex)?; }
        Ok(())
    }

    fn get_minor(&self, iski: usize, iskj: usize) -> Result<Matrix<f32>, Box<dyn Error>>{
        if self.rows != self.cols {
            Err(InvalidSize::new("Матрица не является квадратной"))?;
        }
        let mut data = Vec::new();
        for (index, number) in self.data.iter().enumerate() {
            if index / self.cols != iski && index % self.cols != iskj {
                data.push(*number);
            }
        }
        Ok(Matrix {
            rows: self.rows - 1,
            cols: self.cols - 1,
            data
        })
    }
}

impl Matrix<isize> {
    fn max(&self) -> Result<isize, Box<dyn Error>> {
        Ok(*self.data.iter().max().ok_or(InvalidSize::new("Матрица пуста"))?)
    }

    fn mul(&self, rhs: &Matrix<isize>) -> Matrix<isize> {
        let mut data: Vec<isize> = Vec::new();
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let mut sum: isize = 0;
                for k in 0..self.cols {
                    let lhs = *self.data.get(i * self.cols + k).unwrap();
                    let rhs1 = *rhs.data.get(k * rhs.cols + j).unwrap();
                    sum += lhs * rhs1;
                }
                data.push(sum);
            }
        }
        Matrix {
            rows: self.rows,
            cols: rhs.cols,
            data
        }
    }

    fn get_minor(&self, iski: usize, iskj: usize) -> Result<Matrix<isize>, Box<dyn Error>>{
        if self.rows != self.cols {
            Err(InvalidSize::new("Матрица не является квадратной"))?;
        }
        let mut data = Vec::new();
        for (count, elem) in self.data.iter().enumerate() {
            if count / self.cols != iski && count % self.cols != iskj {
                data.push(*elem);
            }
        }
        Ok(Matrix {
            rows: self.rows - 1,
            cols: self.cols - 1,
            data
        })
    }

    fn partial_reverse(&self) -> Result<Matrix<isize>, Box<dyn Error>> {
        if self.rows != self.cols {
            Err(InvalidSize::new("Матрица не является квадратной"))?;
        }
        let mut multiplicator: isize = 1;
        let mut data = Vec::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                let det = Matrix::from(&self.get_minor(i, j)?).det()?.round();
                data.push(
                    multiplicator * det as isize
                );
                multiplicator *= -1;
            }
            if self.rows % 2 == 0 { multiplicator *= -1; }
        }
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data
        }.transp()
    }

    fn transp(&self) -> Result<Matrix<isize>, Box<dyn Error>> {
        if self.rows != self.cols {
            Err(InvalidSize::new("Матрица не является квадратной"))?;
        }
        let mut tmp;
        let mut data = Vec::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                tmp = *self.data.get(j * self.cols + i).unwrap();
                data.push(tmp);
            }
        }
        Ok(Matrix {
            rows: self.rows,
            cols: self.cols,
            data
        })
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Matrix<T> where
    T: Copy
{
    type Error = InvalidSize;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let rows = value.len();
        if rows == 0 { return Err(InvalidSize::new("Матрица не может быть нулевой")); }
        let cols = value.first().unwrap().len();
        if cols == 0 { return Err(InvalidSize::new("Матрица не может быть нулевой")); }
        let mut data = Vec::new();
        for row in value {
            if row.len() != cols {
                return Err(InvalidSize::new("Исходный массив не может быть ступенчатым"));
            }
            for item in row {
                data.push(item);
            }
        }
        let result = Matrix {
            rows, cols, data
        };
        Ok(result)
    }
}

impl<T> TryFrom<&Vec<T>> for Matrix<T> where
    T: Copy
{
    type Error = InvalidSize;

    fn try_from(value: &Vec<T>) -> Result<Self, Self::Error> {
        let rows = value.len();
        if rows == 0 { return Err(InvalidSize::new("Матрица не может быть нулевой")); }
        let mut data = Vec::new();
        for item in value {
            data.push(*item);
        }
        let result = Matrix {
            rows, data, cols: 1
        };
        Ok(result)
    }
}

impl<T> From<&Matrix<T>> for Matrix<f32> where
    T: AsPrimitive<f32>
{
    fn from(value: &Matrix<T>) -> Self {
        let mut data: Vec<f32> = Vec::new();
        for elem in value.data.iter() {
            data.push((*elem).as_());
        }
        Self {
            rows: value.rows,
            cols: value.cols,
            data
        }
    }
}

fn split_string(phrase: &str, border: usize) -> Result<Vec<Matrix<isize>>, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    let mut result = Vec::new();
    let mut phrase = phrase.to_owned();
    if phrase.chars().count() % border != 0 {
        for _ in 0..(border - phrase.chars().count() % border) {
            phrase.push('\u{0444}');
        }
    }
    let length = phrase.chars().count();
    let letters: Vec<char> = phrase.chars().collect();
    for i in (0..length).step_by(border) {
        let buffer: Vec<isize> = letters[i..i+border].iter().map(
            |letter| alphabet.index_of(*letter) as isize + 1).collect();
        result.push(Matrix::try_from(&buffer)?);
    }
    Ok(result)
}

fn count_digits(number: usize) -> usize {
    number.to_string().len()
}

fn get_numbers(phrase: &str, avg_length: usize, n: usize)
    -> Result<Vec<Matrix<isize>>, Box<dyn Error>>
{
    let mut result = Vec::new();
    if (phrase.chars().count() / avg_length) % n != 0 {
        Err(InvalidSize::new("Некорректная фраза"))?
    }
    let mut buffer = Vec::new();
    for i in 0..phrase.chars().count() / avg_length {
        let number: isize = phrase[i*avg_length..i*avg_length+avg_length].parse()?;
        buffer.push(number);
        if (i + 1) % n == 0 {
            result.push(Matrix::try_from(&buffer)?);
            buffer.clear();
        }
    }
    Ok(result)
}

pub fn encrypt(phrase: &str, matrix: Vec<Vec<isize>>) -> Result<String, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    let matrix = Matrix::try_from(matrix)?;
    if Matrix::from(&matrix).det()?.round() == 0. {
        Err(NullSizedValue::new("Определитель равен 0"))?;
    }
    let letter_boxes = split_string(phrase, matrix.rows)?;
    let result_boxes: Vec<Matrix<isize>> = letter_boxes.iter().map(
        |letter_box| matrix.mul(letter_box)).collect();
    let count = count_digits(
        matrix.max()? as usize *
            ((1 + alphabet.len()) as f32 / 2.).round() as usize * matrix.rows
    );
    let mut result = String::new();
    for result_box in result_boxes {
        for number in result_box.data {
            let format = format!("{:0count$}", number);
            result.push_str(&format);
        }
    }
    Ok(result)
}

pub fn decrypt(phrase: &str, matrix: Vec<Vec<isize>>) -> Result<String, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate_decrypt(phrase)?;
    let matrix = Matrix::try_from(matrix)?;
    let det = Matrix::from(&matrix).det()?.round() as isize;
    if det == 0 {
        Err(NullSizedValue::new("Определитель равен 0"))?;
    }
    let count = count_digits(
        matrix.max()? as usize *
            ((1 + alphabet.len()) as f32 / 2.).round() as usize * matrix.rows
    );
    let number_boxes = get_numbers(phrase, count, matrix.rows)?;
    let matrix = matrix.partial_reverse()?;
    let result_boxes: Vec<Matrix<isize>> = number_boxes.iter().map(
        |number_box| matrix.mul(number_box)).collect();
    let mut result = String::new();
    for result_box in result_boxes {
        for number in result_box.data {
            if number % det != 0 {
                Err(InvalidSize::new("Некорректная фраза"))?
            }
            let number = number / det;
            if number > alphabet.len() as isize || number <= 0 {
                Err(InvalidSize::new("Некорректная фраза"))?
            }
            result.push(alphabet.get(number as usize - 1));
        }
    }
    Ok(result)
}

fn validate_decrypt(phrase: &str) -> Result<(), Box<dyn Error>> {
    if phrase.is_empty() { return Err(Box::new(NullSizedValue::new("Фраза"))); }
    let alphabet = Alphabet::from("0123456789".to_string());
    alphabet.validate(phrase)
}

#[cfg(test)]
mod matrix_test {
    use super::*;

    #[test]
    fn test_matrix_det0() {
        let data: Vec<Vec<isize>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let matr = Matrix::try_from(data).unwrap();
        assert_eq!(Matrix::from(&matr).det().unwrap().round(), 0.);
    }

    #[test]
    fn test_matrix_det_non_zero() {
        let data: Vec<Vec<isize>> = vec![vec![2, 5, 6], vec![4, 3, 2], vec![7, 1, 5]];
        let matr = Matrix::try_from(data).unwrap();
        assert_eq!(Matrix::from(&matr).det().unwrap().round(), -106.);
    }

    #[test]
    fn test_encrypt() {
        let elems: Vec<Vec<isize>> = vec![vec![2, 5, 6], vec![4, 3, 2], vec![7, 1, 5]];
        let result = encrypt(
            "отодногопорченогояблокавесьвоззагниваеттчк", elems).unwrap();
        assert_eq!(result, "215147199170092124179093123259159242172096131275125203154074101045053093276136205129073076045043077091089122146060108224170212");
    }

    #[test]
    fn test_decrypt() {
        let elems: Vec<Vec<isize>> = vec![vec![2, 5, 6], vec![4, 3, 2], vec![7, 1, 5]];
        let result = decrypt("215147199170092124179093123259159242172096131275125203154074101045053093276136205129073076045043077091089122146060108224170212",
                             elems).unwrap();
        assert_eq!(result, "отодногопорченогояблокавесьвоззагниваеттчк");
    }

    #[test]
    #[should_panic]
    fn test_encrypt_det_panic() {
        let elems: Vec<Vec<isize>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        encrypt(
            "отодногопорченогояблокавесьвоззагниваеттчк", elems).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_encrypt_null_matrix_panic() {
        let elems: Vec<Vec<isize>> = Vec::new();
        encrypt(
            "отодногопорченогояблокавесьвоззагниваеттчк", elems).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_encrypt_ladder_matrix_panic() {
        let elems: Vec<Vec<isize>> = vec![vec![1], vec![4, 5], vec![7, 8, 9]];
        encrypt(
            "отодногопорченогояблокавесьвоззагниваеттчк", elems).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_decrypt_panic() {
        let elems: Vec<Vec<isize>> = vec![vec![2, 5, 6], vec![4, 3, 2], vec![7, 1, 5]];
        decrypt("222222222", elems).unwrap();
    }
}
