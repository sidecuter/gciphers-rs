use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};
use num::Integer;
use primes::is_prime;
use rand::Rng;
use regex::Regex;
use crate::alphabet::Alphabet;
use crate::errors::InvalidKeyError;
use crate::methods::{modd, validate_single};
use super::{phi, pow_mod};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub a: isize,
    pub b: isize,
    pub modula: usize,
    pub point: Option<(usize, usize)>
}

#[derive(Copy, Clone)]
pub struct CipherValue(Point, usize);

impl CipherValue {
    pub fn new(s: &str, a: isize, b: isize, modula: usize) -> Self {
        let buff: Vec<usize> =  s.to_string()
            .replace(['(', ')'], "")
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        match buff[..] {
            [x, y, e, ..] => Self(Point::new(a, b, x, y, modula), e),
            [] | [_] | [_, _] => panic!("Непредвиденное поведение")
        }
    }
}

impl Display for CipherValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.point.is_some() {
            let (x, y) = self.get_x_y();
            write!(f, "({},{})" , x, y)
        } else {
            write!(f, "(О)")
        }
    }
}

impl Point {
    pub fn new(a: isize, b: isize, x: usize, y: usize, modula: usize) -> Self {
        let point = Some((x, y));
        Self { a, b, point, modula }
    }

    pub fn get_x_y(&self) -> (usize, usize) {
        if let Some((x, y)) = self.point {
            (x, y)
        } else { (0, 0) }
    }

    pub fn get_x_y_isize(&self) -> (isize, isize) {
        if let Some((x, y)) = self.point {
            (x as isize, y as isize)
        } else { (0, 0) }
    }

    fn lambda_xx(&self, rhs: &Self) -> Option<usize> {
        let (rhs_x, rhs_y) = rhs.get_x_y_isize();
        let (self_x, self_y) = self.get_x_y_isize();
        let left = modd(rhs_y - self_y, self.modula);
        let right = modd(rhs_x - self_x, self.modula);
        if right == 0 { None? }
        Some(Self::div_by_mod(left, right, self.modula))
    }

    fn lambda_x2(&self) -> Option<usize> {
        let (self_x, self_y) = self.get_x_y_isize();
        let left = modd(3 * self_x * self_x + self.a, self.modula);
        let right = modd(2 * self_y, self.modula);
        if right == 0 { None? }
        Some(Self::div_by_mod(left, right, self.modula))
    }

    fn div_by_mod(a: usize, b: usize, modula: usize) -> usize {
        let phi = phi(modula);
        let b = pow_mod(b, phi - 1, modula);
        (a * b) % modula
    }

    pub fn mul(&self, n: usize) -> Self {
        let point = *self;
        let mut temp = point;
        for _ in 1..n {
            temp += point;
        }
        temp
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut temp = Self::new(self.a, self.b, 0, 0, self.modula);
        if self.point.is_some() && rhs.point.is_some() {
            let (rhs_x, rhs_y) = rhs.get_x_y_isize();
            let (self_x, self_y) = self.get_x_y_isize();
            let (lambda, x) = if self_x != rhs_x || self_y != rhs_y {
                let lambda = self.lambda_xx(&rhs);
                let lambda = if let Some(lambda) = lambda { lambda as isize }
                else { temp.point = None; return temp; };
                let x = modd(lambda * lambda - self_x - rhs_x, self.modula) as isize;
                (lambda, x)

            } else {
                let lambda = self.lambda_x2();
                let lambda = if let Some(lambda) = lambda { lambda as isize }
                else { temp.point = None; return temp; };
                let x = modd(lambda * lambda - 2 * self_x, self.modula) as isize;
                (lambda, x)
            };
            let y = modd((lambda * modd(self_x - x, self.modula) as isize) - self_y, self.modula);
            temp.point = Some((x as usize, y));
            temp
        } else if self.point.is_none() && rhs.point.is_none() {
            temp.point = None;
            temp
        } else if self.point.is_none() {
            rhs
        } else {
            self
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Default for Point {
    fn default() -> Self {
        Point {
            a: 0, b: 0, modula: 0, point: None
        }
    }
}

fn get_points(a: isize, b: isize, modula: usize) -> Vec<Point> {
    let ys: Vec<usize> = (0..modula).collect();
    let y2s: HashMap<usize, usize> = ys.iter().map(|y| (((*y)*(*y)) % modula, *y)).collect();
    let xs = ys.clone();
    let y4x: Vec<usize> = xs.iter().map(|x|
        modd((*x).pow(3) as isize + a * (*x) as isize + b, modula)).collect();
    let y4x: Vec<Option<usize>> = y4x.iter()
        .map(|y| if y2s.contains_key(y) { Some(*y) } else { None }).collect();
    let mut xys = Vec::new();
    for (y4xi, xsi) in y4x.into_iter().zip(xs.into_iter())
        .filter(|a| a.0.is_some())
        .map(|a| (a.0.unwrap(), a.1)) {
        xys.push(Point::new(a, b, xsi, y2s[&y4xi], modula));
        if y2s[&y4xi] != 0 {
            xys.push(Point::new(a, b, xsi, modd(-(y2s[&y4xi] as isize), modula), modula));
        }
    }
    xys
}

fn get_q(n: usize) -> usize{
    let mut q = 1;
    for i in 3..n {
        if n.gcd(&i) == i && is_prime(i as u64) { q = i; }
    }
    if q == 1 { n } else { q }
}

pub fn get_keys() -> (Point, usize, usize, Point) {
    let mut rng = rand::thread_rng();
    let mut modula: usize = rng.gen_range(34..60);
    while !is_prime(modula as u64) {
        modula = rng.gen_range(34..60);
    }
    let mut a: isize = rng.gen_range(1..10);
    let mut b: isize = rng.gen_range(1..10);
    while !validate_ell(a, b, modula) {
        a = rng.gen_range(1..10);
        b = rng.gen_range(1..10);
    }
    let points_group = get_points(a, b, modula);
    let n = points_group.len() + 1;
    let q = get_q(n);
    let h = n / q;
    let mut index = rng.gen_range(0..points_group.len());
    while points_group[index].mul(h).point.is_none() {
        index = rng.gen_range(0..points_group.len());
    }
    let g = points_group[index].mul(h);
    let secret = rng.gen_range(1..q);
    let open = g.mul(secret);
    (g, q, secret, open)
}

fn validate_ell(a: isize, b: isize, modula: usize) -> bool {
    modd(4 * a.pow(3) + 27*b.pow(2), modula) != 0
}

pub fn enc(mi: isize, db: &Point, g: &Point, mut k: usize, q: usize) -> CipherValue {
    let mut r = g.mul(k);
    let mut p = db.mul(k);
    let (mut x, _) = p.get_x_y_isize();
    let mut rng = rand::thread_rng();
    while x == 0 {
        k = rng.gen_range(1..q);
        r = g.mul(k);
        p = db.mul(k);
        (x, _) = p.get_x_y_isize();
    }
    CipherValue(r, modd(mi * x, p.modula))
}

pub fn encrypt(phrase: &str, db: Point, g: Point, q: usize) -> Result<String, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    if !validate_ell(g.a, g.b, g.modula) {
        Err(InvalidKeyError::new("Кривая не соответстует условию"))?;
    }
    if !is_prime(g.modula as u64) {
        Err(InvalidKeyError::new("Модуль кривой не является простым числом"))?;
    }
    let mut rng = rand::thread_rng();
    let result = phrase.chars().map(|x| {
        let mi = alphabet.index_of(x) + 1;
        let k = rng.gen_range(1..q);
        enc(mi as isize, &db, &g, k, q).to_string()
    }).collect::<Vec<String>>().join("");
    Ok(result)
}

pub fn dec(cb: usize, value: CipherValue, modula: usize) -> usize {
    let q = value.0.mul(cb);
    let (x, _) = q.get_x_y();
    modd(value.1 as isize * pow_mod(x, modula-2, modula) as isize, modula)
}

pub fn decrypt(phrase: &str, cb: usize, a: isize, b: isize, modula: usize)
    -> Result<String, Box<dyn Error>>
{
    let alphabet = Alphabet::from("0123456789(),".to_string());
    alphabet.validate(phrase)?;
    let alphabet = Alphabet::new();
    if !validate_ell(a, b, modula) {
        Err(InvalidKeyError::new("Кривая не соответстует условию"))?;
    }
    if !is_prime(modula as u64) {
        Err(InvalidKeyError::new("Модуль кривой не является простым числом"))?;
    }
    let re = Regex::new(r"(\(\(\d+,\d+\),\d+\))").unwrap();
    let result: String = re.find_iter(phrase).map(|x| {
        let val = CipherValue::new(x.as_str(), a, b, modula);
        let m = dec(cb, val, modula);
        alphabet.get(m-1)
    }).collect();
    Ok(result)
}

#[cfg(test)]
mod ecc_tests {
    use super::*;

    #[test]
    fn test_enc() {
        let g = Point::new(3, 4, 4, 6, 11);
        let db = g.mul(4);
        enc(10, &db, &g, 5, 7);
    }

    #[test]
    fn test_encrypt() {
        let phrase = "отодно";
        let a = 2;
        let b = 7;
        let p = 47;
        let db = Point::new(a, b, 8, 21, p);
        let g = Point::new(a, b, 8, 26, p);
        let q = 3;
        encrypt(phrase, db, g, q).unwrap();
    }

    #[test]
    fn test_decrypt() {
        let phrase = "((8,21),26)((8,21),11)((8,21),26)((8,26),40)((8,21),18)((8,21),26)";
        let a = 2;
        let b = 7;
        let p = 47;
        let cb = 2;
        let valid = "отодно";
        assert_eq!(valid, decrypt(phrase, cb, a, b, p).unwrap());
    }
}
