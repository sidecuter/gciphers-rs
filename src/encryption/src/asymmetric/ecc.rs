use std::collections::HashMap;
use std::error::Error;
use std::ops::{Add, AddAssign};
use num::Integer;
use num::integer::Roots;
use rand::Rng;
use crate::alphabet::Alphabet;
use crate::methods::{modd, validate_single};
use super::{phi, pow_mod};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    a: usize,
    b: usize,
    modula: usize,
    point: Option<(usize, usize)>
}

impl Point {
    fn new(a: usize, b: usize, x: usize, y: usize, modula: usize) -> Self {
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
        let left = modd(3 * self_x * self_x + self.a as isize, self.modula);
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
        for _ in 2..n {
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

fn get_points(a: usize, b: usize, modula: usize) -> Vec<Vec<Point>> {
    let ys: Vec<usize> = (0..modula).collect();
    let y2s: HashMap<usize, usize> = ys.iter().map(|y| (((*y)*(*y)) % modula, *y)).collect();
    let xs = ys.clone();
    let y4x: Vec<usize> = xs.iter().map(|x| ((*x).pow(3) + a * (*x) + b) % modula).collect();
    let y4x: Vec<Option<usize>> = y4x.iter()
        .map(|y| if y2s.contains_key(y) { Some(*y) } else { None }).collect();
    let mut xys = Vec::new();
    for (y4xi, xsi) in y4x.into_iter().zip(xs.into_iter())
        .filter(|a| a.0.is_some())
        .map(|a| (a.0.unwrap(), a.1)) {
        if y2s[&y4xi] != 0 {
            xys.push(vec![Point::new(a, b, xsi, y2s[&y4xi], modula)]);
            xys.push(vec![Point::new(a, b, xsi, modd(-(y2s[&y4xi] as isize), modula), modula)]);
        } else {
            xys.push(vec![Point::new(a, b, xsi, y2s[&y4xi], modula)]);
        }
    }
    for i in 0..xys.len() {
        for j in 0..modula - 2 {
            let next_elem = xys[i][0].clone() + xys[i][j].clone();
            xys[i].push(next_elem);
        }
    }
    xys
}

fn get_q(n: usize) -> usize{
    let mut q = 1;
    for i in 2..n.sqrt() {
        if n.gcd(&i) == 1 { q = i; }
    }
    if q == 1 { n } else { q }
}

pub fn get_keys(a: usize, b: usize, modula: usize) -> (Point, usize, Point, usize) {
    let mut rng = rand::thread_rng();
    let points_group = get_points(a, b, modula);
    let n = points_group.len() + 1;
    let q = get_q(n);
    let h = n / q;
    let mut index = rng.gen_range(0..points_group.len());
    while points_group[index][0].mul(h).point.is_none() {
        index = rng.gen_range(0..points_group.len());
    }
    let g = points_group[index][0].mul(h);
    let secret = rng.gen_range(1..q);
    let open = g.mul(secret);
    (g, q, open, secret)
}

pub fn encrypt(phrase: &str, db: Point, g: Point, q: usize) -> Result<String, Box<dyn Error>> {
    let alphabet = Alphabet::new();
    validate_single(&alphabet, phrase)?;
    let mut rng = rand::thread_rng();
    let k = rng.gen_range(1..q);
    let _r = g.mul(k);
    let _p = db.mul(k);
    todo!()
}

#[cfg(test)]
mod ecc_tests {
    use super::*;

    #[test]
    fn test_get_points() {
        println!("{:#?}", get_points(1, 3, 7))
    }
}
