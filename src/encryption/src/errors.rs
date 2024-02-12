use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct InvalidTextError;

#[derive(Debug, Clone)]
pub struct InvalidKeyError {
    pub message: String
}

#[derive(Debug, Clone)]
pub struct NullSizedValue {
    pub message: String
}

#[derive(Debug, Clone)]
pub struct InvalidSize {
    pub message: String
}

#[derive(Debug, Clone)]
pub struct OutOfBounds {
    pub message: String
}

#[derive(Debug, Clone)]
pub struct InvalidIndex;

impl InvalidKeyError {
    pub fn new(message: &str) -> Self { InvalidKeyError {
        message: message.to_owned()
    } }
}

impl OutOfBounds {
    pub fn new(message: &str) -> Self { OutOfBounds {
        message: message.to_owned()
    } }
}

impl InvalidSize {
    pub fn new(message: &str) -> Self { InvalidSize {
        message: message.to_owned()
    } }
}

impl NullSizedValue {
    pub fn new(message: &str) -> Self { NullSizedValue {
        message: message.to_owned()
    } }
}

impl Display for InvalidTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Алфавит не содержит такой буквы")
    }
}

impl Display for InvalidIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "По такому индексу не найдено элемента")
    }
}

impl Display for InvalidKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Display for InvalidSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Display for NullSizedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} отсутствует", self.message)
    }
}

impl Display for OutOfBounds {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Указанный {} выходит за границы таблицы", self.message)
    }
}

impl Error for InvalidTextError {}
impl Error for InvalidKeyError {}
impl Error for NullSizedValue {}
impl Error for InvalidIndex {}
impl Error for InvalidSize {}
impl Error for OutOfBounds {}
