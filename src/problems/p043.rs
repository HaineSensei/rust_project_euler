use std::{iter::Filter, ops::RangeInclusive};

use itertools::{Itertools, Permutations};

const PRIMES: [usize;7] = [2,3,5,7,11,13,17];

struct PandigitalNumber([char;10]);

impl PandigitalNumber {
    fn satisfies_property(&self) -> bool {
        (1..8).all(|x|self.slice(x).is_multiple_of(PRIMES[x-1]))
    }

    fn slice(&self, initial_idx: usize) -> usize {
        self.0[initial_idx..initial_idx+3]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap()
    }

    fn as_usize(&self) -> usize {
        self
        .0
        .iter()
        .collect::<String>()
        .parse()
        .unwrap()
    }
}

struct PandigitalIter {
    perm_iter: Filter<Permutations<RangeInclusive<char>>,fn(&Vec<char>) -> bool>
}

impl PandigitalIter<> {
    fn new() -> Self {
        Self {
            perm_iter: ('0'..='9').permutations(10).filter(|x| x[0] != '0')
        }
    }
}

impl Iterator for PandigitalIter {
    type Item=PandigitalNumber;

    fn next(&mut self) -> Option<Self::Item> {
        self
        .perm_iter
        .next()
        .map(|x|PandigitalNumber(x.try_into().unwrap()))
    }
}

pub fn main() {
    let total = PandigitalIter::new().filter_map(|x|
        if x.satisfies_property() {
            Some(x.as_usize())
        } else {
            None
        }
    )
    .sum::<usize>();
    println!("{total}");
}

#[cfg(test)]
mod tests {
    use crate::problems::p043::PandigitalNumber;


    #[test]
    fn test_char_range() {
        assert_eq!(('0'..='9').collect::<String>().as_str(),"0123456789");
    }

    #[test]
    fn test_pandigital_number_slice() {
        let number = PandigitalNumber(
            "1406357289"
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
        );
        assert_eq!(number.slice(1),406);
        assert_eq!(number.slice(2),63);
    }

    #[test]
    fn test_pandigital_property() {
        let number = PandigitalNumber(
            "1406357289"
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
        );
        assert!(number.satisfies_property())
    }
}