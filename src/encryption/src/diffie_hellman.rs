use std::error::Error;
use rand::Rng;
use crate::asymmetric::pow_mod;

pub fn gen_keys(a: usize, n: usize) -> Result<(usize, usize), Box<dyn Error>> {
    if a >= n || a <= 1{
        Err("введены некорректные начальные значения")?;
    }
    let mut rnd = rand::thread_rng();
    let k = rnd.gen_range(2..n-1);
    let y = get_y(a, n, k);
    Ok((k, y))
}

pub fn get_y(a: usize, n: usize, k: usize) -> usize {
    pow_mod(a, k, n)
}

pub fn get_k(n: usize, k: usize, y: usize) -> usize {
    pow_mod(y, k, n)
}
