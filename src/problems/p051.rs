use crate::utils::{digits,u64_from_digits,is_prime};
use itertools::Itertools;

struct Mask<const NUMBER_DIGITS: usize> {
    mask_digits: [bool;NUMBER_DIGITS]
}

impl<const N: usize> Mask<N> {
    fn new(xs: &[bool; N]) -> Self {
        Mask::<N>{
            mask_digits: *xs
        }
    }

    fn all_for(digits: &[u8; N]) -> Vec<Self> {
        let mut out = Vec::new();
        for indexes in (0..N).powerset() {
            if indexes.is_empty() {
                continue;
            }
            let mut term = [false;N];
            for i in indexes {
                term[i] = true;
            }
            out.push(Mask{mask_digits: term})
        }
        out
    }

    fn possibilities(&self, x: u64) -> Vec<u64> {
        let mut digits: [u8;N] = digits(x).try_into().unwrap();
        let start = match self.mask_digits[0] {
            true => 1,
            false => 0,
        };
        let mut out = Vec::new();
        for d in start..10 {
            for i in 0..N {
                match self.mask_digits[i] {
                    true => {
                        digits[i] = d;
                    },
                    false => {},
                }
            }
            out.push(u64_from_digits(&digits));
        }
        out
    }
}



pub fn main() {
    let out = (10000..=99999).filter(|&x| {
        let digits = digits(x);
        let masks: Vec<Mask<5>> = Mask::all_for(&digits.try_into().unwrap());
        for mask in masks {
            if mask.possibilities(x).iter().filter(|&&x|is_prime(x)).count() == 8 {
                return true;
            }
        }
        false
    }).next().unwrap();
}
