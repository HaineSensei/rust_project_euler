// looking for 11 primes s.t. ABCD is both left and right truncatable forces ABC to be left truncatable and BCD to be right truncatable.

use std::collections::HashSet;

struct PrimesIter {
    initial:bool,
    primes_so_far: Vec<usize>,
    next_check: usize
}

impl Iterator for PrimesIter {
    type Item=usize;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {initial,primes_so_far, next_check} = self;
        if *initial {
            *initial = false;
            return Some(2)
        }
        while primes_so_far.iter().any(|p| *next_check%*p==0) {
            *next_check+=2
        }
        let out = *next_check;
        primes_so_far.push(out);
        *next_check+=2;
        Some(out)
    }
}

impl PrimesIter {
    fn new() -> Self {
        PrimesIter { initial: true, primes_so_far: vec![2], next_check: 3 }
    }
}

enum DigitSplit {
    Single,
    Multiple { left: usize, right: usize }
}

impl DigitSplit {
    fn from(n:usize) -> Self {
        let string = format!("{n}");
        match string.len() {
            1 => Self::Single,
            n => Self::Multiple {left: string[..n-1].parse().unwrap(), right: string[1..].parse().unwrap()}
        }
    }
}


pub fn main() {
    // including single digits
    let mut left_trunc: HashSet<usize> = HashSet::new(); 
    let mut right_trunc: HashSet<usize> = HashSet::new();
    let mut dual_trunc: HashSet<usize> = HashSet::new();
    for p in PrimesIter::new() {
        match DigitSplit::from(p) {
            DigitSplit::Single => {
                left_trunc.insert(p);
                right_trunc.insert(p);
            },
            DigitSplit::Multiple { left, right } => {
                let left_bool = left_trunc.contains(&left);
                let right_bool = right_trunc.contains(&right);
                if left_bool {
                    left_trunc.insert(p);
                }
                if right_bool {
                    right_trunc.insert(p);
                }
                if left_bool&&right_bool {
                    dual_trunc.insert(p);
                    if dual_trunc.len() == 11 {
                        break
                    }
                }
            },
        }
    }
    println!(
        "{}",
        dual_trunc
            .iter()
            .sum::<usize>()
    );
}


#[cfg(test)]
mod tests {
    use crate::{problems::p037::PrimesIter, utils::primes_less_than};

    #[test]
    fn primesiter_works() {
        let mut primes = primes_less_than(100);
        for p in PrimesIter::new() {
            if let Some(prime) = primes.next() {
                assert_eq!(p,prime as usize);
            } else {
                break
            }
        }
    }
}
