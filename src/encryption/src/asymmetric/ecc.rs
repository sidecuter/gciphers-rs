use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};
use num::Integer;
use num::integer::Roots;
use primes::is_prime;
use rand::Rng;
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

    fn get_x_y(&self) -> (usize, usize) {
        if let Some((x, y)) = self.point {
            (x, y)
        } else { (0, 0) }
    }

    fn get_x_y_isize(&self) -> (isize, isize) {
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

    fn mul(&self, n: usize) -> Self {
        let point = self.clone();
        let mut temp = point.clone();
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
        *self = self.clone() + rhs;
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

pub fn get_keys() -> (Point, usize, usize) {
    let mut rng = rand::thread_rng();
    //let modula: usize = rng.gen_range(33..60);
    //let mut a: isize = rng.gen_range(1..modula as isize);
    //let mut b: isize = rng.gen_range(1..modula as isize);
    // while !validate_ell(a, b, modula) {
    //     a = rng.gen_range(1..modula as isize);
    //     b = rng.gen_range(1..modula as isize);
    // }
    let a = 3;
    let b = 4;
    let modula = 11;
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
    (g, q, secret)
}

fn validate_ell(a: isize, b: isize, modula: usize) -> bool {
    modd(4 * a.pow(3) + 27*b.pow(2), modula) != 0
}

pub fn enc(mi: isize, db: &Point, g: &Point, mut k: usize, q: usize) -> (Point, usize) {
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
    (r, modd(mi * x, p.modula))
}

pub fn encrypt(phrase: &str, cb: usize, g: Point, q: usize) -> Result<String, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    let db= g.mul(cb);
    if !validate_ell(g.a, g.b, g.modula) {
        Err(InvalidKeyError::new("Кривая не соответстует условию"))?;
    }
    let mut rng = rand::thread_rng();
    let phrase: Vec<usize> = phrase.chars().map(|x| alphabet.index_of(x) + 1).collect();
    let mut result = Vec::new();
    for mi in phrase {
        let k = rng.gen_range(1..q);
        let (r, e) = enc(mi as isize, &db, &g, k, q);
        result.push(format!("({},{})", r.to_string(), e));
    }
    Ok(result.join(","))
}

#[cfg(test)]
mod ecc_tests {
    use super::*;

    #[test]
    fn test_get_points() {
        println!("{:#?}", get_points(1, 3, 7))
    }

    #[test]
    fn test_enc() {
        let g = Point::new(3, 4, 4, 6, 11);
        let db = g.mul(4);
        let (r, e) = enc(10, &db, &g, 5, 7);
        println!("({},{})", r.to_string(), e);
    }

    #[test]
    fn test_encrypt() {
        let phrase = "отодно";
        let (g, q, secret) = get_keys();
        println!("{}", encrypt(phrase ,secret, g, q).unwrap())
    }
}
