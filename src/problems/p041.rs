use std::collections::HashSet;

use crate::utils::PrimesIter;

fn is_pandigital(n: &usize) -> bool {
    let n_string = format!("{n}");
    let length = n_string.len();
    n_string.chars().collect::<HashSet<_>>() == (1..length).map(|x|"0123456789".chars().nth(x).unwrap_or('f')).collect::<HashSet<_>>()
}

pub fn main() {
    // takes *way* too long. should instead enumerate pandigitals in decending order and check primality.
    let answer = PrimesIter::new().take_while(|p| *p <= 987654321).filter(is_pandigital).max().unwrap();
    println!("{answer}");
}