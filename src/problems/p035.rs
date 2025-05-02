use std::sync::LazyLock;

use crate::utils::{digits, u64_from_digits, primes_less_than};

fn cycles_of_digits(x:u64) -> Vec<u64> {
    let digits = digits(x);
    let mut out = Vec::new();
    for i in 0..digits.len() {
        out.push(u64_from_digits(&[digits[i..].to_vec(),digits[..i].to_vec()].concat()))
    }
    out
}

const MAXIMUM_AS_U128: u128 = 1000000;
const MAXIMUM_AS_U64: u64 = MAXIMUM_AS_U128 as u64;

static PRIMES: LazyLock<Vec<u128>> = LazyLock::new(|| primes_less_than(MAXIMUM_AS_U128).collect());

fn is_prime(x:u64) -> bool {
    PRIMES.contains(&(x as u128))
}

pub fn main() {
    let mut count = 0;
    for num in 1..MAXIMUM_AS_U64 {
        if cycles_of_digits(num).iter().all(|&x|is_prime(x)) {
            count +=1;
        }
    }
    println!("{count}");
}
