fn main() {
    println!("{}",prime(1000001));
}

fn prime(n:usize) -> u64 {
    let mut primes = Vec::<u64>::new();
    let mut current: u64 = 1;
    'outer: while primes.len() < n {
        current += 1;
        for p in primes.iter() {
            if p*p>current {
                break
            }
            if current%p==0 {
                continue 'outer;
            }
        }
        primes.push(current);
    }
    primes[n-1]
}