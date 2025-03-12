use std::{iter::from_fn, sync::{LazyLock, Mutex}};

static PRIMES: LazyLock<Mutex<Vec<u128>>> = LazyLock::new(|| {
    let x = Mutex::new(Vec::new());
    x.lock().unwrap().push(2);
    x
});


pub fn primes_less_than(x: u128) -> impl Iterator<Item = u128> {
    let mut curr = 1;
    let mut curr_index = 0;
    let max_known;
    {
        max_known = *PRIMES.lock().unwrap().iter().max().unwrap();
    }
    std::iter::from_fn(move || {
        if curr < max_known {
            let p = *PRIMES.lock().unwrap().get(curr_index).unwrap();
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
                    for &p in PRIMES.lock().unwrap().iter() {
                        if curr%p == 0 {
                            prime = false;
                            break;
                        }
                    }
                    if prime {
                        {
                            let mut primes = PRIMES.lock().unwrap();
                            if !primes.contains(&curr){
                                primes.push(curr);
                            }
                        }
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
}