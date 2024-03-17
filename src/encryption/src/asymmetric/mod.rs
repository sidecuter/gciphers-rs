use num::Integer;
use crate::methods::modd;

pub mod rsa;
pub mod elgamal;
pub mod ecc;

fn pow_mod(number: usize, power: usize, modula: usize) -> usize {
    let mut result = number;
    for _ in 1..power {
        result *= number;
        result %= modula;
    }
    result
}

fn get_numbers(phrase: &str, len: usize) -> Vec<usize> {
    phrase.chars().collect::<Vec<char>>()
        .windows(len).step_by(len)
        .map(|x| x.iter().collect::<String>().parse::<usize>().unwrap()).collect()
}

fn euclid(mut a: usize, mut b: usize) -> Vec<usize> {
    let mut result = Vec::new();
    while a != 0 && b != 0 {
        if a > b {
            result.push(a/b);
            a %= b;
        }
        else {
            result.push(b/a);
            b %= a;
        }
    }
    result
}

fn eq(a: usize, b: usize, m: usize) -> usize {
    let mut q = euclid(a, m);
    if m < a {
        q.insert(0, 0);
    }
    let mut p: Vec<usize> = vec![1, q[0]];
    for i in 1..q.len() {
        p.push(p[i]*q[i]+p[i-1]);
    }
    let n = q.len();
    modd((-1isize).pow((n - 1) as u32) * p[n - 1] as isize * b as isize, m)
}

fn phi(number: usize) -> usize {
    (1..=number).filter(|x| number.gcd(x) == 1).count()
}

fn to_string(number: usize, len: usize) -> String {
    format!("{:0size$}", number, size=len)
}

#[cfg(test)]
mod test_methods {
    use super::*;

    #[test]
    fn test_eq() {
        assert_eq!(48, eq(321, 9, 531))
    }
}
