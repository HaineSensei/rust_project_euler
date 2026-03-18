// looking for 11 primes s.t. ABCD is both left and right truncatable forces ABC to be left truncatable and BCD to be right truncatable.

use std::collections::HashSet;

use crate::utils::PrimesIter;

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