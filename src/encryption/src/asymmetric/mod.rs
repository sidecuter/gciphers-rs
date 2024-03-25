use num::Integer;

pub mod ecc;
pub mod elgamal;
pub mod rsa;

pub fn pow_mod(number: usize, power: usize, modula: usize) -> usize {
    let mut result = number;
    for _ in 1..power {
        result *= number;
        result %= modula;
    }
    result
}

fn get_numbers(phrase: &str, len: usize) -> Vec<usize> {
    phrase
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .step_by(len)
        .map(|x| x.iter().collect::<String>().parse::<usize>().unwrap())
        .collect()
}

fn phi(number: usize) -> usize {
    (1..=number).filter(|x| number.gcd(x) == 1).count()
}

fn to_string(number: usize, len: usize) -> String {
    format!("{:0size$}", number, size = len)
}
