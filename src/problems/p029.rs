use std::{collections::{HashMap, HashSet}, hash::Hash, ops::Mul, sync::LazyLock};

use crate::utils::primes_less_than;

// Could make PartialEq and Eq more precise by including 0 powers, but should be easy to avoid that.
#[derive(Debug, PartialEq, Eq)]
struct PrimeProduct(HashMap<usize,usize>);

// actually PrimeProduct*usize gives the PrimeProduct raised to the power of usize here lol.
impl Mul<usize> for &PrimeProduct {
    type Output = PrimeProduct;

    fn mul(self, rhs: usize) -> Self::Output {
        let mut out = HashMap::new();
        for (&p, exp) in &self.0 {
            out.insert(p,exp*rhs);
        }
        PrimeProduct(out)
    }
}

// only need up to 100 due to the values we're considering
static PRIMES : LazyLock<Vec<usize>> = LazyLock::new(|| {
    primes_less_than(100).map(|x| x as usize).collect()
});

fn prime_product(n: usize) -> PrimeProduct {
    let mut out: HashMap<usize, usize> = HashMap::new();
    let mut curr = n;
    for &p in PRIMES.iter() {
        while curr % p == 0 {
            curr /= p;
            if out.contains_key(&p) {
                let exp = *out.get(&p).unwrap();
                out.remove(&p);
                out.insert(p,exp+1);
            } else {
                out.insert(p, 1);
            }
        }
    }
    PrimeProduct(out)
}

impl Hash for PrimeProduct {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut vec = self.0.iter().collect::<Vec<_>>();
        vec.sort();
        vec.hash(state)
    }
}

static PRIME_PRODUCTS : LazyLock<HashMap<usize,PrimeProduct>> = LazyLock::new(|| {
    let mut prime_products = HashMap::new();
    for n in 2..=100 {
        prime_products.insert(n, prime_product(n));
    }
    prime_products
});

pub fn main() {
    let values = (2..=100)
        .flat_map(|a| (2usize..=100)
            .map( move |b| &PRIME_PRODUCTS[&a]*b)
        )
        .collect::<HashSet<_>>();
    println!("{}", values.len());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_product() {
        assert_eq!(prime_product(12), PrimeProduct(HashMap::from([(2,2), (3,1)])));
        assert_eq!(prime_product(23), PrimeProduct(HashMap::from([(23,1)])));
        assert_eq!(&prime_product(10)*2,prime_product(100));
    }
}