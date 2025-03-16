use std::fmt::Display;

use crate::utils::{primes_less_than, MyIteratorExt};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct QuadraticPair {
    a: i64,
    b: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct QuadraticPairValue {
    val: usize,
    quad: QuadraticPair
}

impl QuadraticPair {

    fn new(a: i64, b: u64) -> QuadraticPair {
        QuadraticPair{
            a,
            b,
        }
    }

    fn apply(self, n: u64) -> i128 {
        let (n,a,b) = (n as i128, self.a as i128, self.b as i128);
        (n + a)*n + b
    }

    fn value(self) -> usize {
        (0..).map(|n|match is_prime(self.apply(n)) {
            true => Ok(()),
            false => Err(())
        })
            .to_my_iterator()
            .terminate_on_err()
            .collect::<Vec<_>>()
            .len()
    }

    fn product(self) -> i128 {
        let (a, b) = (self.a as i128, self.b as i128);
        a*b
    }

}

impl QuadraticPairValue {
    fn new(quad: QuadraticPair) -> QuadraticPairValue {
        QuadraticPairValue {
            quad,
            val: quad.value()
        }
    }
    
    #[allow(dead_code)]
    fn apply(self, n: u64) -> i128 {
        self.quad.apply(n)
    }

    #[allow(dead_code)]
    fn value(self) -> usize {
        self.val
    }

    fn product(self) -> i128 {
        self.quad.product()
    }
}

fn is_prime(n: i128) -> bool {
    if n <= 1 {
        false
    } else {
        let over_estimate_sqrt = (n as f64).sqrt() as u128 + 1; 
        for p in primes_less_than(over_estimate_sqrt) {
            if n as u128 % p == 0 {
                return if n as u128 == p {
                    true
                } else {
                    false
                };
            }
        }
        true
    }
}

impl Display for QuadraticPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (a, b) = (self.a, self.b);
        match (a.cmp(&0), b.cmp(&0)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => {
                write!(f, "n^2 - {}n - {}", -a, b)
            },
            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => {
                write!(f, "n^2 - {}n", -a)
            },
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => {
                write!(f, "n^2 - {}n + {b}", -a)
            },
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => {
                write!(f, "n^2 - {}", b)
            },
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => {
                write!(f, "n^2")
            },
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => {
                write!(f, "n^2 + {b}")
            },
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => {
                write!(f, "n^2 + {a}n - {b}")
            },
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => {
                write!(f, "n^2 + {a}n")
            },
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
                write!(f, "n^2 + {a}n + {b}")
            },
        }
    }
}

impl Display for QuadraticPairValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.quad)
    }
}

pub fn main() {
    let quads: Vec<QuadraticPairValue> = (-999..1000).flat_map(|a| (0..=1000).filter(|&b|is_prime(b as i128)).map( move |b| QuadraticPairValue::new(QuadraticPair::new(a,b)))).collect();
    if let Some(quad) = quads.iter().max() {
        println!("{quad}: {}",quad.product())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2),true);
        assert_eq!(is_prime(3),true);
        assert_eq!(is_prime(4),false);
        assert_eq!(is_prime(53),true);
        assert_eq!(is_prime(57),false);
        assert_eq!(is_prime(65537),true);
        assert_eq!(is_prime(65537*65537),false);
    }

    #[test]
    fn test_value() {
        let quad1 = QuadraticPair::new(1,41);
        assert_eq!(quad1.value(),40);
        let quad2 = QuadraticPair::new(-79,1601);
        assert_eq!(quad2.value(),80);
    }
}
