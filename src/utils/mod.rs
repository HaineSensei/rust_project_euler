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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes_less_than() {
        assert_eq!(primes_less_than(20).collect::<Vec<_>>(),[2,3,5,7,11,13,17,19]);
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
}