use std::error::Error;
use crate::alphabet::Alphabet;
use std::vec::Vec;
use crate::errors::{InvalidIndex, InvalidKeyError};

fn get_order (alphabet: &Alphabet, key: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut result: Vec<usize> = Vec::new();
    for _ in key.chars() { result.push(0) };
    let mut letter_indexes: Vec<usize> = Vec::new();
    for letter in key.chars() {
        let pos = alphabet.index_of(letter);
        letter_indexes.push(pos);
    }
    let mut positions = letter_indexes.clone();
    positions.sort();
    let mut i: usize = 0;
    for position in positions {
        while letter_indexes.contains(&position) {
            let pos = letter_indexes.iter(
            ).position(|x| x == &position).ok_or(InvalidIndex)?;
            if let Some(elem) = result.get_mut(pos) { *elem = i + 1; }
            if let Some(elem) = letter_indexes.get_mut(pos) { *elem = alphabet.len(); }
            i += 1;
        }
    }
    Ok(result)
}

fn get_rows_and_keys(
    alphabet: &Alphabet,
    phrase: &str,
    key: &str
) -> Result<(Vec<usize>, usize), Box<dyn Error>> {
    let phrase_len = phrase.chars().count();
    let keys = get_order(&alphabet, &key)?;
    let row_o = phrase_len / keys.len();
    let row = if phrase_len % keys.len() != 0 {row_o + 1} else {row_o};
    Ok((keys, row))
}

fn sort(order: &mut Vec<usize>, data: &mut Vec<Vec<isize>>) {
    let mut i: usize = 0;
    loop {
        match order.get(i) {
            Some(index) => {
                if *index - 1 == i {
                    i += 1;
                    continue;
                }
            },
            None => break
        }
        let index = match order.get(i) {
            Some(index) => *index - 1,
            None => continue
        };
        data.swap(i, index);
        order.swap(i, index);
    }
}

fn get_empty_slots(keys: &Vec<usize>, last_row: usize) -> Option<Vec<usize>> {
    let mut result: Vec<usize> = Vec::new();
    for key in keys.iter().rev().take(last_row) {
        result.push(*key);
    }
    if last_row > 0 { result.sort(); Some(result) } else { None }
}

fn get_result(alphabet: &Alphabet, buffer: &Vec<Vec<isize>>, row: usize, col: usize)
    -> Result<String, Box<dyn Error>>
{
    let mut result = String::new();
    for i in 0..row {
        for j in 0..col {
            let index = *buffer.get(j)
                .ok_or(InvalidIndex)?.get(i).ok_or(InvalidIndex)?;
            if index != -1 {
                result.push(alphabet.get(index as usize));
            }
        }
    }
    Ok(result)
}

pub fn encrypt(phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    let (keys, row) = get_rows_and_keys(&alphabet, phrase, key)?;
    let mut buffer: Vec<Vec<isize>> = vec![Vec::new(); keys.len()];
    let mut letter = phrase.chars();
    for _ in 0..row {
        for j in 0..keys.len() {
            let val = match letter.next() {
                Some(letter) => alphabet.index_of(letter) as isize,
                None => -1
            };
            buffer.get_mut(j).ok_or(InvalidIndex)?.push(val);
        }
    }
    sort(&mut keys.clone(), &mut buffer);
    get_result(&alphabet, &buffer, row, keys.len())
}

fn prepare_keys(keys: &Vec<usize>) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut result: Vec<usize> = Vec::new();
    for _ in 0..keys.len() { result.push(0); };
    for (i, elem) in keys.iter().enumerate() {
        match result.iter_mut().nth(*elem - 1) {
            Some(elem) => *elem = i + 1,
            None => Err(InvalidKeyError::new("Ключ содержит невалидные значения"))?
        };
    }
    Ok(result)
}

pub fn decrypt(phrase: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    let (keys, row) = get_rows_and_keys(&alphabet, phrase, key)?;
    let mut buffer: Vec<Vec<isize>> = Vec::new();
    let mut letter = phrase.chars();
    for _ in 0..keys.len() { buffer.push(Vec::new()); }
    let empty_slots = get_empty_slots(
        &keys,
        row * keys.len() - phrase.chars().count()
    );
    let mut k: usize = 0;
    for i in 0..row {
        for j in 0..keys.len() {
            let elem = match &empty_slots {
                Some(slot) => {
                    match slot.iter().nth(k) {
                        Some(elem) => Some(*elem),
                        None => None
                    }
                },
                None => None
            };
            let val = if i + 1 == row && elem.is_some() && elem.ok_or(InvalidIndex)? - 1 == j {
                k += 1;
                -1
            } else {
                match letter.next() {
                    Some(letter) => alphabet.index_of(letter) as isize,
                    None => -1
                }
            };
            buffer.get_mut(j).ok_or(InvalidIndex)?.push(val);
        }
    }
    sort(&mut prepare_keys(&keys)?, &mut buffer);
    get_result(&alphabet, &buffer, row, keys.len())
}

#[cfg(test)]
mod vetrical_tests {
    use crate::alphabet::Alphabet;
    use std::vec::Vec;
    use super::*;

    #[test]
    fn test_vertical_get_order() {
        let alphabet = Alphabet::new();
        let keys_control: Vec<usize> = vec![4, 5, 3, 6, 1, 2];
        let keys = get_order(&alphabet, "супчик").unwrap();
        assert_eq!(keys, keys_control);
    }

    #[test]
    fn test_vertical_get_rows_and_keys() {
        let alphabet = Alphabet::new();
        let phrase = String::from("отодногопорченогояблока");
        let key = String::from("супчик");
        let keys_control: Vec<usize> = vec![4, 5, 3, 6, 1, 2];
        let row_control: usize = 4;
        let (keys, row) = get_rows_and_keys(
            &alphabet, &phrase, &key).unwrap();
        assert_eq!(keys, keys_control);
        assert_eq!(row, row_control);
    }

    #[test]
    fn test_vertical_prepare_keys() {
        let keys: Vec<usize> = vec![4, 5, 3, 6, 1, 2];
        let keys_control: Vec<usize> = vec![5, 6, 3, 1, 2, 4];
        let keys = prepare_keys(&keys).unwrap();
        assert_eq!(keys, keys_control);
    }

    #[test]
    fn test_vertical_sort() {
        let mut keys: Vec<usize> = vec![4, 5, 3, 6, 1, 2];
        let keys_control: Vec<usize> = vec![1, 2, 3, 4, 5, 6];
        let mut vals: Vec<Vec<isize>> = vec![
            vec![14, 3, 5, 1],
            vec![18, 14, 13, 11],
            vec![14, 15, 14, 14],
            vec![4, 14, 3, 10],
            vec![13, 16, 14, 0],
            vec![14, 23, 31, -1]
        ];
        let vals_control: Vec<Vec<isize>> = vec![
            vec![13, 16, 14, 0],
            vec![14, 23, 31, -1],
            vec![14, 15, 14, 14],
            vec![14, 3, 5, 1],
            vec![18, 14, 13, 11],
            vec![4, 14, 3, 10]
        ];
        sort(&mut keys, &mut vals);
        assert_eq!(keys, keys_control);
        assert_eq!(vals, vals_control);
    }

    #[test]
    fn test_vertical_get_empty_slots() {
        let keys: Vec<usize> = vec![4, 5, 3, 6, 1, 2];
        let slots_control: Vec<usize> = vec![1,2,6];
        let slots = get_empty_slots(&keys, 3).unwrap();
        assert_eq!(slots, slots_control);
        let slots_none = get_empty_slots(&keys, 0).is_none();
        assert!(slots_none);
    }

    #[test]
    fn test_vertical_get_result () {
        let alphabet = Alphabet::new();
        let vals: Vec<Vec<isize>> = vec![
            vec![13, 16, 14],
            vec![14, 23, 31],
            vec![14, 15, 14],
            vec![14, 3, 5],
            vec![18, 14, 13],
            vec![4, 14, 3]
        ];
        let result_control = String::from("нооотдрчпгооояоенг");
        let result = get_result(&alphabet, &vals, 3, 6).unwrap();
        assert_eq!(result, result_control);
    }
    #[test]
    fn test_vertical_encrypt() {
        let phrase = String::from("отодногопорченогояблокавесьвоззагниваеттчк");
        let key = String::from("супчик");
        let result = encrypt(&phrase, &key).unwrap();
        assert_eq!(result, "нооотдрчпгооояоенгавоблкозьесвивгзанчктает");
    }

    #[test]
    fn test_vertical_decrypt() {
        let phrase = String::from("нооотдрчпгооояоенгавоблкозьесвивгзанчктает");
        let key = String::from("супчик");
        let result = decrypt(&phrase, &key).unwrap();
        assert_eq!(result, "отодногопорченогояблокавесьвоззагниваеттчк");
    }
}
