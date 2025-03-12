use std::{collections::HashSet, iter, sync::LazyLock};
use crate::utils::{self, MyIteratorExt};

static ABUNDANTS : LazyLock<Vec<u64>> = LazyLock::new(|| {
    (1..28123).filter(|&x|x<d(x)).collect()
});

static PRIMES : LazyLock<Vec<u64>> = LazyLock::new(|| {
    utils::primes_less_than(28124).map(|x| x as u64).collect()
});

fn primes_less_than(x:u64) -> impl Iterator<Item = u64> {
    let mut curr = 0;
    iter::from_fn( move || {
        let p = PRIMES.get(curr);
        curr += 1;
        match p {
            Some(&p) => {
                if p >= x {
                    None
                } else {
                    Some(p)
                }
            }
            None => None
        }
    })
}

fn d(x: u64) -> u64 {
    if x == 0 {
        return 0;
    }
    let mut total_product = 1;
    let sigma = primes_less_than(x+1).map( move |p| {
            if total_product == x {
                return Err("hi")
            }
            let p = p as u64;
            let mut total = 1;
            let mut curr_pow = p;
            loop {
                if x%curr_pow != 0 {
                    break
                }
                total += curr_pow;
                curr_pow*=p;
                total_product*=p;
            }
            Ok(total)
        }).to_my_iterator()
        .terminate_on_err()
        .fold(1,|x,y| x*y);
    if sigma < x {
        println!("{sigma},{x}")
    }
    sigma-x
}

pub fn main() {
    let abundant_sums = ABUNDANTS
        .iter()
        .flat_map(|&x| ABUNDANTS
            .iter()
            .filter(move |&&y| y<=x)
            .map(move |&y| x + y)
        )
        .filter(|&x| x < 28123).collect::<HashSet<_>>();
    println!("{}",(1..28123)
        .filter(|x|!abundant_sums.contains(x))
        .sum::<u64>()
    );
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d() {
        assert_eq!(d(0),0);
        assert_eq!(d(1),0);
        assert_eq!(d(28),28);
        assert_eq!(d(12),16);
        assert_eq!(d(28123),1); // max and is prime.
    }
}