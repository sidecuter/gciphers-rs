use std::error::Error;
use crate::alphabet::Alphabet;
use crate::errors::{InvalidIndex, InvalidSize, NullSizedValue, OutOfBounds};

struct Table {
    columns: usize,
    last_row: usize,
    last_column: usize,
    alphabet: Alphabet
}

impl Table {
    fn build(rows: usize, columns: usize) -> Result<Table, Box<dyn Error>> {
        let alphabet = Alphabet::new();
        if rows * columns < alphabet.len() {
            return Err(Box::new(InvalidSize::new(
                "Произведение количества строк на столбцов должно превышать длину алфавита"
            )))
        }
        let last_row = if alphabet.len() % columns == 0 { alphabet.len() / columns }
        else { alphabet.len() / columns + 1 };
        let last_column = columns - (last_row * columns - alphabet.len());
        Ok(Table {
            columns, last_row, last_column, alphabet
        })
    }

    fn index_of(&self, letter: char) -> (usize, usize) {
        let index = self.alphabet.index_of(letter);
        (index / self.columns + 1, index % self.columns + 1)
    }

    fn get(&self, row: usize, column: usize) -> char {
        self.alphabet.get((row - 1) * self.columns + (column - 1))
    }
}

pub fn encrypt(phrase: &str, rows: &str, columns: &str) -> Result<String, Box<dyn Error>> {
    let table = validate(phrase, rows, columns)?;
    let mut result = String::new();
    for letter in phrase.chars() {
        let (row, column) = table.index_of(letter);
        result.push_str(&row.to_string());
        result.push_str(&column.to_string());
    }
    Ok(result)
}

pub fn decrypt(phrase: &str, rows: &str, columns: &str) -> Result<String, Box<dyn Error>> {
    let table = validate_dec(phrase, rows, columns)?;
    let mut result = String::new();
    let mut letter = phrase.chars();
    for _ in 0..phrase.chars().count() / 2 {
        let row: usize = letter.next().unwrap().to_digit(10).unwrap() as usize;
        let column: usize = letter.next().unwrap().to_digit(10).unwrap() as usize;
        result.push(table.get(row, column));
    }
    Ok(result)
}

fn validate(
    phrase: &str, rows: &str, columns: &str
) -> Result<Table, Box<dyn Error>> {
    if phrase.len() == 0 { return Err(Box::new(NullSizedValue::new("Фраза"))); }
    if rows.len() == 0 { return Err(Box::new(NullSizedValue::new("Количество рядов"))); }
    if columns.len() == 0 { return Err(Box::new(NullSizedValue::new("Количество столбцов"))); }
    let table = Table::build(rows.parse()?, columns.parse()?)?;
    table.alphabet.validate(phrase)?;
    Ok(table)
}

fn validate_dec(
    phrase: &str, rows: &str, columns: &str
) -> Result<Table, Box<dyn Error>> {
    if phrase.len() == 0 { return Err(Box::new(NullSizedValue::new("Фраза"))); }
    if phrase.chars().count() % 2 != 0 {return Err(Box::new(InvalidSize::new(
        "Количество цифр должно быть кратным 2"
    )));}
    if rows.len() == 0 { return Err(Box::new(NullSizedValue::new("Количество рядов"))); }
    if columns.len() == 0 { return Err(Box::new(NullSizedValue::new("Количество столбцов"))); }
    let table = Table::build(rows.parse()?, columns.parse()?)?;
    let mut letter = phrase.chars();
    for _ in 0..phrase.chars().count() / 2 {
        let row: usize = letter.next().unwrap().to_digit(10).ok_or(InvalidIndex)? as usize;
        let column: usize = letter.next().unwrap().to_digit(10).ok_or(InvalidIndex)? as usize;
        if row > table.last_row || row <= 0{
            return Err(Box::new(OutOfBounds::new("ряд")));
        }
        if column > table.columns || column <= 0 {
            return Err(Box::new(OutOfBounds::new("столбец")));
        }
        if column > table.last_column && row == table.last_row {
            return Err(Box::new(OutOfBounds::new("столбец")));
        }
    }
    Ok(table)
}

#[cfg(test)]
mod polybius_tests {
    use super::*;

    #[test]
    fn test_table_build() {
        let table = Table::build(6, 6).unwrap();
        assert_eq!(table.columns, 6);
        assert_eq!(table.last_row, 6);
        assert_eq!(table.last_column, 2);
    }

    #[test]
    fn fn_table_index_of() {
        let table = Table::build(6, 6).unwrap();
        let (row, column) = table.index_of('\u{0430}');
        assert_eq!(row, 1);
        assert_eq!(column, 1);
    }

    #[test]
    fn fn_table_get() {
        let table = Table::build(6, 6).unwrap();
        let letter = table.get(1, 1);
        assert_eq!(letter, '\u{0430}');
    }

    #[test]
    fn test_encrypt() {
        let valid = String::from("334133153233143334333546163233143362122633251113163655133322221114322313111641414625");
        let result = encrypt(
            "отодногопорченогояблокавесьвоззагниваеттчк",
            "6", "6").unwrap();
        assert_eq!(result, valid);
    }

    #[test]
    fn test_decrypt() {
        let valid = String::from("отодногопорченогояблокавесьвоззагниваеттчк");
        let result = decrypt(
            "334133153233143334333546163233143362122633251113163655133322221114322313111641414625",
            "6", "6").unwrap();
        assert_eq!(result, valid)
    }

    #[test]
    #[should_panic]
    fn fn_table_build_panic() {
        Table::build(4, 4).unwrap();
    }

    #[test]
    #[should_panic]
    fn fn_table_index_of_panic() {
        let table = Table::build(6, 6).unwrap();
        table.index_of('o');
    }

    #[test]
    #[should_panic]
    fn fn_table_get_panic() {
        let table = Table::build(6, 6).unwrap();
        table.get(6, 3);
    }

    #[test]
    #[should_panic]
    fn test_encrypt_panic() {
        encrypt("z", "6", "6").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_decrypt_panic() {
        decrypt("77", "6", "6").unwrap();
    }
}
