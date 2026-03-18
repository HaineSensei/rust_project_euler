use std::{iter::from_fn, sync::{LazyLock, Mutex}};

pub mod ui_display;

static PRIMES: LazyLock<Mutex<Vec<u128>>> = LazyLock::new(|| {
    let x = Mutex::new(Vec::new());
    x.lock().unwrap().push(2);
    x
});

pub fn primes_less_than(x: u128) -> impl Iterator<Item = u128> {
    let mut curr = 1;
    let mut curr_index = 0;
    let max_known;
    let mut curr_primes;
    let mut prev_size;
    let mut new_primes = Vec::new();
    {
        curr_primes = PRIMES.lock().unwrap().clone();
        max_known = *curr_primes.iter().max().unwrap();
        prev_size = curr_primes.len();
    }
    std::iter::from_fn(move || {
        if prev_size + 1000 <= curr_primes.len() {
            PRIMES.lock().unwrap().append(&mut new_primes);
            prev_size += 1000;
        }
        if curr < max_known {
            let p = *(&curr_primes).get(curr_index).unwrap();
            curr = p + 1;
            if x > p {
                curr_index += 1;
                Some(p)
            } else {
                None
            }
        } else {
            loop {
                if curr >= x {
                    return None;
                } else {
                    let mut prime = true;
                    for &p in &curr_primes {
                        if curr%p == 0 {
                            prime = false;
                            break;
                        }
                    }
                    if prime {
                        (&mut curr_primes).push(curr);
                        (&mut new_primes).push(curr);
                        return Some(curr);
                    } else {
                        curr += 1;
                    }
                }
            }
        }
    }).into_iter()
}

fn u64_len(x:u64) -> u64 {
    if x == 0 {
        0
    } else {
        u64_len(x>>1)
    }
}

pub fn is_prime(x:u64) -> bool {
    if x < 1000 {
        return primes_less_than(x as u128 + 1).collect::<Vec<_>>().contains(&(x as u128));
    }

    let crude_sqrt = (x >> (u64_len(x)/2)) * 2 ;
    for p in primes_less_than(crude_sqrt as u128) {
        if x % p as u64 == 0 {
            return false;
        }
    }
    true
}

pub struct MyIterator<'a, T>(&'a mut dyn Iterator<Item = T>);

impl<T> Iterator for MyIterator<'_, T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            MyIterator(x) => x.next()
        }
    }
}

pub trait MyIteratorExt<T> : Iterator<Item = T> {
    fn to_my_iterator(&mut self) -> MyIterator<'_, T>;
}

impl<'a, T, I> MyIteratorExt<T> for I
where
    I : Iterator<Item = T> + 'a
{
    fn to_my_iterator(&mut self) -> MyIterator<'_, T> {
        MyIterator(self)
    }
}

impl<'a, T: Copy+Clone, E> MyIterator<'a, Result<T,E>> {
    pub fn terminate_on_err(mut self) -> impl Iterator<Item = T> + 'a {
        from_fn( move || {
            match self.next() {
                Some(Ok(x)) => Some(x),
                _ => None,
            }
        }).into_iter()
    }
}

pub fn digits(x: u64) -> Vec<u8> {
    format!("{x}")
        .chars()
        .map(|c|{
            c.to_string().parse().unwrap()
        })
        .collect()
}

pub fn u64_from_digits(digits: &[u8]) -> u64 {
    digits.iter().fold(0,|x,&y|x*10 + (y as u64))
}

pub struct PrimesIter {
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
    pub fn new() -> Self {
        PrimesIter { initial: true, primes_so_far: vec![2], next_check: 3 }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes_less_than() {
        assert_eq!(primes_less_than(20).collect::<Vec<_>>(),[2,3,5,7,11,13,17,19]);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));

        let x = primes_less_than(1000).last().unwrap();
        let y = x*x;
        assert!(is_prime(x as u64));
        assert!(!is_prime(y as u64));
    }

    #[test]
    fn test_moves() {
        fn iterator() -> impl Iterator<Item = u128> {
            let vec = vec![1,2,3];
            let mut c = 0;
            from_fn(move || {
                for x in &vec {
                    c += x
                }
                Some(c)
            })
        }
        let mut iter = iterator();
        assert_eq!(iter.next().unwrap(),6);
        assert_eq!(iter.next().unwrap(),12);
    }

    #[test]
    fn test_digits() {
        let num = 1294;
        let digits = digits(num);
        assert_eq!(digits,[1,2,9,4])
    }

    #[test]
    fn test_u64_from_digits() {
        let digits = vec![3,5,6,3,2,9];
        let num = u64_from_digits(&digits);
        assert_eq!(num, 356329);
    }

    #[test]
    fn digits_u64_from_digits_inverse() {
        let ds = vec![2,4,6,7,3,6];
        let num = 859291;
        assert_eq!(digits(u64_from_digits(&ds)), ds);
        assert_eq!(u64_from_digits(&digits(num)), num);
    }

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