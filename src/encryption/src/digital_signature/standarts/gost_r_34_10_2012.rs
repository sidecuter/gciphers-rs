use std::error::Error;
use rand::Rng;
use crate::asymmetric::pow_mod;
use crate::digital_signature::algorithms::square_hash;
use crate::methods::modd;
pub use crate::asymmetric::ecc::Point;
pub use crate::asymmetric::ecc::get_keys;

pub fn sign(message: &str, x: usize, g: Point, q: usize, m: usize) -> Result<String, Box<dyn Error>> {
    let mut rang = rand::thread_rng();
    let mut h = square_hash(message, m);
    if h == 0 {
        h = 1;
    }
    let mut k= 0;
    let mut p: Point = Point::default();
    while p.point.is_none() || p.get_x_y().0 == 0 {
        k = rang.gen_range(1..q);
        p = g.mul(k);
    }
    let r = modd(p.get_x_y_isize().0, q);
    let s = modd((k * h + r * x) as isize, q);
    Ok(format!("{},{}", r, s))
}

pub fn check_sign(message: &str, y: Point, g: Point, q: usize, sign: &str, m: usize)
    -> Result<bool, Box<dyn Error>>
{
    let mut h = square_hash(message, m);
    if h == 0 {
        h = 1;
    }
    let parsed: Vec<usize> = sign.split(',')
        .map(|x| x.to_string().parse::<usize>().expect("Not a number"))
        .collect();
    let [r, s] = parsed[..] else { Err("1")? };
    if r == 0 || s >= q {
        return Ok(false)
    }
    let h_1 = pow_mod(h, q-2, q);
    let u1 = modd((s * h_1) as isize, q);
    let u2 = modd(-((r * h_1) as isize), q);
    let p = g.mul(u1) + y.mul(u2);
    if p.point.is_none() {
        return Ok(false)
    }
    if modd(p.get_x_y_isize().0, q) != r {
        Ok(false)
    } else {
        Ok(true)
    }
}
