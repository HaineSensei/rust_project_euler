use std::{iter::Rev, ops::RangeInclusive, sync::LazyLock};

use itertools::{Itertools, Permutations};

use crate::utils::{PrimesIter};

/// an iterator object for N-pandigitals (when 1<=N<=9) from greatest to least.
struct PandigitalIter<const N: usize> {
    permutation_iter: Permutations<Rev<RangeInclusive<usize>>>,
}


impl<const N: usize> PandigitalIter<N> {
    fn new() -> Self {
        Self { permutation_iter: (1..=N).rev().permutations(N) }
    }
}

impl<const N: usize> Iterator for PandigitalIter<N> {
    type Item=usize;

    fn next(&mut self) -> Option<Self::Item> {
        self
        .permutation_iter
        .next()
        .map(
            |x|{
                x
                .iter()
                .map(|i|{
                    format!("{i}")
                    .chars()
                    .next()
                    .unwrap()
                })
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
            }
        )
    }
}

static PRIMES: LazyLock<Vec<usize>> = LazyLock::new(||
    PrimesIter::new()
    .take_while(|p| p*p <= 7654321)
    .collect()
);

fn is_prime(x:&usize) -> bool {
    !PRIMES.iter().any(|p|x%p==0)
}

pub fn main() {
    // don't bother with 2,3,5,6,8,9-pandigitals which are all multiples of 3.
    let pandigitals = PandigitalIter::<7>::new()
        .chain(PandigitalIter::<4>::new())
        .chain(PandigitalIter::<1>::new());

    let max_prime_pandigital = {
        pandigitals
        .filter(is_prime)
        .next()
        .unwrap()
    };

    println!("{max_prime_pandigital}");
}